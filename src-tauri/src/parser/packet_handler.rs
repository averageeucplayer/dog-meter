use std::{cell::{RefCell, RefMut}, path::Path, rc::Rc, sync::Arc, time::{Duration, Instant}};

use anyhow::Result;
use chrono::Utc;
use hashbrown::HashMap;
use log::info;
use meter_core::{decryption::DamageEncryptionHandlerInner, packets::{definitions::*, opcodes::Pkt}};

use crate::{abstractions::{ConnectionFactory, EventEmitter}, flags::Flags, models::{self, events::*, *}, parser::utils::{get_and_set_region, write_local_players}};

use super::{encounter_state::EncounterState, entity_tracker::{get_current_and_max_hp, EntityTracker}, id_tracker::IdTracker, party_tracker::PartyTracker, stats_api::StatsApi, status_tracker::{get_status_effect_value, StatusTracker}, utils::{debug_print, get_class_from_id, on_shield_change, parse_pkt, update_party}};


pub fn handle_packet<E: EventEmitter, C: ConnectionFactory>(
    op_code: Pkt,
    data: Vec<u8>,
    state: &mut EncounterState,
    party_tracker: Rc<RefCell<PartyTracker>>,
    entity_tracker: &mut EntityTracker,
    status_tracker: &mut RefMut<'_, StatusTracker>,
    id_tracker: Rc<RefCell<IdTracker>>,
    party_cache: &mut Option<Vec<Vec<String>>>,
    party_map_cache: &mut HashMap<i32, Vec<String>>,
    local_info: &mut LocalInfo,
    stats_api: &mut StatsApi,
    region_file_path: &str,
    party_freeze: &mut bool,
    local_player_path: &str,
    raid_end_cd: &mut Instant,
    flags: &Flags,
    event_emitter: Arc<E>,
    connection_factory: Arc<C>,
    damage_handler: &DamageEncryptionHandlerInner,
    version: &str
) -> Result<()> {

    match op_code {
        Pkt::CounterAttackNotify => {
            if let Some(pkt) =
                parse_pkt(&data, PKTCounterAttackNotify::new, "PKTCounterAttackNotify")
            {
                if let Some(entity) = entity_tracker.entities.get(&pkt.source_id) {
                    state.on_counterattack(entity);
                }
            }
        }
        Pkt::DeathNotify => {
            if let Some(pkt) = parse_pkt(&data, PKTDeathNotify::new, "PKTDeathNotify") {
                if let Some(entity) = entity_tracker.entities.get(&pkt.target_id) {
                    debug_print(format_args!(
                        "death: {}, {}, {}",
                        entity.name, entity.entity_type, entity.id
                    ));
                    state.on_death(entity);
                }
            }
        }
        Pkt::IdentityGaugeChangeNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTIdentityGaugeChangeNotify::new,
                "PKTIdentityGaugeChangeNotify",
            ) {
                state.on_identity_gain(&pkt);
                if flags.can_emit_details() {
                    event_emitter.emit(
                        IdentityUpdate {
                            gauge1: pkt.identity_gauge1,
                            gauge2: pkt.identity_gauge2,
                            gauge3: pkt.identity_gauge3,
                        },
                    ).unwrap();
                }
            }
        }
        Pkt::InitEnv => {
            // three methods of getting local player info
            // 1. MigrationExecute    + InitEnv      + PartyInfo
            // 2. Cached Local Player + InitEnv      + PartyInfo
            //    > character_id        > entity_id    > player_info
            // 3. InitPC

            if let Some(pkt) = parse_pkt(&data, PKTInitEnv::new, "PKTInitEnv") {
                party_tracker.borrow_mut().reset_party_mappings();
                state.raid_difficulty = "".to_string();
                state.raid_difficulty_id = 0;
                state.damage_is_valid = true;
                *party_cache = None;
                *party_map_cache = HashMap::new();
                let entity = entity_tracker.init_env(pkt);
                state.on_init_env(entity, &stats_api, event_emitter, connection_factory, version);
                stats_api.valid_zone = false;
                get_and_set_region(region_file_path, state);
                info!("region: {:?}", state.region);
            }
        }
        Pkt::InitPC => {
            if let Some(pkt) = parse_pkt(&data, PKTInitPC::new, "PKTInitPC") {
                let (hp, max_hp) = get_current_and_max_hp(&pkt.stat_pairs);
                let entity = entity_tracker.init_pc(pkt);
                info!(
                    "local player: {}, {}, {}, eid: {}, id: {}",
                    entity.name,
                    get_class_from_id(&entity.class_id),
                    entity.gear_level,
                    entity.id,
                    entity.character_id
                );

                local_info
                    .local_players
                    .entry(entity.character_id)
                    .and_modify(|e| {
                        e.name = entity.name.clone();
                        e.count += 1;
                    })
                    .or_insert(LocalPlayer {
                        name: entity.name.clone(),
                        count: 1,
                    });

                write_local_players(&local_info, local_player_path)?;

                state.on_init_pc(entity, hp, max_hp)
            }
        }
        Pkt::NewPC => {
            if let Some(pkt) = parse_pkt(&data, PKTNewPC::new, "PKTNewPC") {
                let (hp, max_hp) = get_current_and_max_hp(&pkt.pc_struct.stat_pairs);
                let entity = entity_tracker.new_pc(pkt);
                debug_print(format_args!(
                    "new PC: {}, {}, {}, eid: {}, cid: {}",
                    entity.name,
                    get_class_from_id(&entity.class_id),
                    entity.gear_level,
                    entity.id,
                    entity.character_id
                ));
                state.on_new_pc(entity, hp, max_hp);
            }
        }
        Pkt::NewNpc => {
            if let Some(pkt) = parse_pkt(&data, PKTNewNpc::new, "PKTNewNpc") {
                let (hp, max_hp) = get_current_and_max_hp(&pkt.npc_struct.stat_pairs);
                let entity = entity_tracker.new_npc(pkt, max_hp);
                debug_print(format_args!(
                    "new {}: {}, eid: {}, id: {}, hp: {}",
                    entity.entity_type, entity.name, entity.id, entity.npc_id, max_hp
                ));
                state.on_new_npc(entity, hp, max_hp);
            }
        }
        Pkt::NewNpcSummon => {
            if let Some(pkt) = parse_pkt(&data, PKTNewNpcSummon::new, "PKTNewNpcSummon") {
                let (hp, max_hp) = get_current_and_max_hp(&pkt.npc_struct.stat_pairs);
                let entity = entity_tracker.new_npc_summon(pkt, max_hp);
                debug_print(format_args!(
                    "new {}: {}, eid: {}, id: {}, hp: {}",
                    entity.entity_type, entity.name, entity.id, entity.npc_id, max_hp
                ));
                state.on_new_npc(entity, hp, max_hp);
            }
        }
        Pkt::NewProjectile => {
            if let Some(pkt) = parse_pkt(&data, PKTNewProjectile::new, "PKTNewProjectile") {
                entity_tracker.new_projectile(&pkt);
                if entity_tracker.id_is_player(pkt.projectile_info.owner_id)
                    && pkt.projectile_info.skill_id > 0
                {
                    let key = (pkt.projectile_info.owner_id, pkt.projectile_info.skill_id);
                    if let Some(timestamp) = state.skill_tracker.skill_timestamp.get(&key) {
                        state
                            .skill_tracker
                            .projectile_id_to_timestamp
                            .insert(pkt.projectile_info.projectile_id, timestamp);
                    }
                }
            }
        }
        Pkt::NewTrap => {
            if let Some(pkt) = parse_pkt(&data, PKTNewTrap::new, "PKTNewTrap") {
                entity_tracker.new_trap(&pkt);
                if entity_tracker.id_is_player(pkt.trap_struct.owner_id)
                    && pkt.trap_struct.skill_id > 0
                {
                    let key = (pkt.trap_struct.owner_id, pkt.trap_struct.skill_id);
                    if let Some(timestamp) = state.skill_tracker.skill_timestamp.get(&key) {
                        state
                            .skill_tracker
                            .projectile_id_to_timestamp
                            .insert(pkt.trap_struct.object_id, timestamp);
                    }
                }
            }
        }

        Pkt::RaidBegin => {
            if let Some(pkt) = parse_pkt(&data, PKTRaidBegin::new, "PKTRaidBegin") {
                debug_print(format_args!("raid begin: {}", pkt.raid_id));
                match pkt.raid_id {
                    308226 | 308227 | 308239 | 308339 => {
                        state.raid_difficulty = "Trial".to_string();
                        state.raid_difficulty_id = 7;
                    }
                    308428 | 308429 | 308420 | 308410 | 308411 | 308414 | 308422 | 308424
                    | 308421 | 308412 | 308423 | 308426 | 308416 | 308419 | 308415 | 308437
                    | 308417 | 308418 | 308425 | 308430 => {
                        state.raid_difficulty = "Challenge".to_string();
                        state.raid_difficulty_id = 8;
                    }
                    _ => {
                        state.raid_difficulty = "".to_string();
                        state.raid_difficulty_id = 0;
                    }
                }

                stats_api.valid_zone = VALID_ZONES.contains(&pkt.raid_id);
            }
        }
        Pkt::RaidBossKillNotify => {
            state.on_phase_transition(1, stats_api, event_emitter, connection_factory, version);
            state.raid_clear = true;
            debug_print(format_args!("phase: 1 - RaidBossKillNotify"));
        }
        Pkt::RaidResult => {
            *party_freeze = true;
            state.party_info = if let Some(party) = party_cache.take() {
                party
            } else {
                update_party(&party_tracker, &entity_tracker)
            };
            state.on_phase_transition(0, stats_api, event_emitter, connection_factory, version);
            *raid_end_cd = Instant::now();
            debug_print(format_args!("phase: 0 - RaidResult"));
        }
        Pkt::RemoveObject => {
            if let Some(pkt) = parse_pkt(&data, PKTRemoveObject::new, "PKTRemoveObject") {
                for upo in pkt.unpublished_objects {
                    entity_tracker.entities.remove(&upo.object_id);
                    status_tracker.remove_local_object(upo.object_id);
                }
            }
        }
        Pkt::SkillCastNotify => {
            if let Some(pkt) = parse_pkt(&data, PKTSkillCastNotify::new, "PKTSkillCastNotify") {
                let mut entity = entity_tracker.get_source_entity(pkt.source_id);
                entity_tracker.guess_is_player(&mut entity, pkt.skill_id);
                if entity.class_id == 202 {
                    state.on_skill_start(
                        &entity,
                        pkt.skill_id,
                        None,
                        None,
                        Utc::now().timestamp_millis(),
                    );
                }
            }
        }
        Pkt::SkillStartNotify => {
            if let Some(pkt) = parse_pkt(&data, PKTSkillStartNotify::new, "PKTSkillStartNotify")
            {
                let mut entity = entity_tracker.get_source_entity(pkt.source_id);
                entity_tracker.guess_is_player(&mut entity, pkt.skill_id);
                let tripod_index =
                    pkt.skill_option_data
                        .tripod_index
                        .map(|tripod_index| models::TripodIndex {
                            first: tripod_index.first,
                            second: tripod_index.second,
                            third: tripod_index.third,
                        });
                let tripod_level =
                    pkt.skill_option_data
                        .tripod_level
                        .map(|tripod_level| models::TripodLevel {
                            first: tripod_level.first,
                            second: tripod_level.second,
                            third: tripod_level.third,
                        });
                let timestamp = Utc::now().timestamp_millis();
                let (skill_id, summon_source) = state.on_skill_start(
                    &entity,
                    pkt.skill_id,
                    tripod_index,
                    tripod_level,
                    timestamp,
                );

                if entity.entity_type == EntityType::PLAYER && skill_id > 0 {
                    state
                        .skill_tracker
                        .new_cast(entity.id, skill_id, summon_source, timestamp);
                }
            }
        }
        // Pkt::SkillStageNotify => {
        //     let pkt = PKTSkillStageNotify::new(&data);
        // }
        Pkt::SkillDamageAbnormalMoveNotify => {
            if Instant::now() - *raid_end_cd < Duration::from_secs(10) {
                debug_print(format_args!(
                    "ignoring damage - SkillDamageAbnormalMoveNotify"
                ));
                return Ok(());
            }
            if let Some(pkt) = parse_pkt(
                &data,
                PKTSkillDamageAbnormalMoveNotify::new,
                "PKTSkillDamageAbnormalMoveNotify",
            ) {
                let now = Utc::now().timestamp_millis();
                let owner = entity_tracker.get_source_entity(pkt.source_id);
                let local_character_id = id_tracker
                    .borrow()
                    .get_local_character_id(entity_tracker.local_entity_id);
                let target_count = pkt.skill_damage_abnormal_move_events.len() as i32;
                let player_stats = stats_api.get_stats(&state);
                for mut event in pkt.skill_damage_abnormal_move_events.into_iter() {
                    if !damage_handler.decrypt_damage_event(&mut event.skill_damage_event) {
                        state.damage_is_valid = false;
                        continue;
                    }
                    let target_entity =
                        entity_tracker.get_or_create_entity(event.skill_damage_event.target_id);
                    let source_entity = entity_tracker.get_or_create_entity(pkt.source_id);
                    let (se_on_source, se_on_target) = status_tracker
                        .get_status_effects(&owner, &target_entity, local_character_id);
                    let damage_data = DamageData {
                        skill_id: pkt.skill_id,
                        skill_effect_id: pkt.skill_effect_id,
                        damage: event.skill_damage_event.damage,
                        modifier: event.skill_damage_event.modifier as i32,
                        target_current_hp: event.skill_damage_event.cur_hp,
                        target_max_hp: event.skill_damage_event.max_hp,
                        damage_attribute: event.skill_damage_event.damage_attr,
                        damage_type: event.skill_damage_event.damage_type,
                    };

                    state.on_damage(
                        &owner,
                        &source_entity,
                        &target_entity,
                        damage_data,
                        se_on_source,
                        se_on_target,
                        target_count,
                        &entity_tracker,
                        &player_stats,
                        now,
                        event_emitter.as_ref()
                    );
                }
            }
        }
        Pkt::SkillDamageNotify => {
            // use this to make sure damage packets are not tracked after a raid just wiped
            if Instant::now() - *raid_end_cd < Duration::from_secs(10) {
                debug_print(format_args!("ignoring damage - SkillDamageNotify"));
                return Ok(());
            }
            if let Some(pkt) =
                parse_pkt(&data, PKTSkillDamageNotify::new, "PktSkillDamageNotify")
            {
                let now = Utc::now().timestamp_millis();
                let owner = entity_tracker.get_source_entity(pkt.source_id);
                let local_character_id = id_tracker
                    .borrow()
                    .get_local_character_id(entity_tracker.local_entity_id);
                let target_count = pkt.skill_damage_events.len() as i32;
                let player_stats = stats_api.get_stats(&state);
                for mut event in pkt.skill_damage_events.into_iter() {
                    if !damage_handler.decrypt_damage_event(&mut event) {
                        state.damage_is_valid = false;
                        continue;
                    }
                    let target_entity = entity_tracker.get_or_create_entity(event.target_id);
                    // source_entity is to determine battle item
                    let source_entity = entity_tracker.get_or_create_entity(pkt.source_id);
                    let (se_on_source, se_on_target) = status_tracker
                        .get_status_effects(&owner, &target_entity, local_character_id);
                    let damage_data = DamageData {
                        skill_id: pkt.skill_id,
                        skill_effect_id: pkt.skill_effect_id.unwrap_or_default(),
                        damage: event.damage,
                        modifier: event.modifier as i32,
                        target_current_hp: event.cur_hp,
                        target_max_hp: event.max_hp,
                        damage_attribute: event.damage_attr,
                        damage_type: event.damage_type,
                    };
                    state.on_damage(
                        &owner,
                        &source_entity,
                        &target_entity,
                        damage_data,
                        se_on_source,
                        se_on_target,
                        target_count,
                        &entity_tracker,
                        &player_stats,
                        now,
                        event_emitter.as_ref()
                    );
                }
            }
        }
        Pkt::PartyInfo => {
            if let Some(pkt) = parse_pkt(&data, PKTPartyInfo::new, "PKTPartyInfo") {
                entity_tracker.party_info(pkt, &local_info);
                let local_player_id = entity_tracker.local_entity_id;
                if let Some(entity) = entity_tracker.entities.get(&local_player_id) {
                    state.update_local_player(entity);
                }
                *party_cache = None;
                *party_map_cache = HashMap::new();
            }
        }
        Pkt::PartyLeaveResult => {
            if let Some(pkt) = parse_pkt(&data, PKTPartyLeaveResult::new, "PKTPartyLeaveResult")
            {
                party_tracker
                    .borrow_mut()
                    .remove(pkt.party_instance_id, pkt.name);
                *party_cache = None;
                *party_map_cache = HashMap::new();
            }
        }
        Pkt::PartyStatusEffectAddNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTPartyStatusEffectAddNotify::new,
                "PKTPartyStatusEffectAddNotify",
            ) {
                // info!("{:?}", pkt);
                let shields =
                    entity_tracker.party_status_effect_add(pkt, &state.encounter.entities);
                for status_effect in shields {
                    let source = entity_tracker.get_source_entity(status_effect.source_id);
                    let target_id =
                        if status_effect.target_type == StatusEffectTargetType::Party {
                            id_tracker
                                .borrow()
                                .get_entity_id(status_effect.target_id)
                                .unwrap_or_default()
                        } else {
                            status_effect.target_id
                        };
                    let target = entity_tracker.get_source_entity(target_id);
                    // info!("SHIELD SOURCE: {} > TARGET: {}", source.name, target.name);
                    state.on_boss_shield(&target, status_effect.value);
                    state.on_shield_applied(
                        &source,
                        &target,
                        status_effect.status_effect_id,
                        status_effect.value,
                    );
                }
            }
        }
        Pkt::PartyStatusEffectRemoveNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTPartyStatusEffectRemoveNotify::new,
                "PKTPartyStatusEffectRemoveNotify",
            ) {
                let character_id = pkt.character_id;
                let (is_shield, shields_broken, _left_workshop) =
                    entity_tracker.party_status_effect_remove(pkt);
                if is_shield {
                    for status_effect in shields_broken {
                        let change = status_effect.value;
                        on_shield_change(
                            entity_tracker,
                            &id_tracker,
                            state,
                            status_effect,
                            change,
                        );
                    }
                }
            }
        }
        Pkt::PartyStatusEffectResultNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTPartyStatusEffectResultNotify::new,
                "PKTPartyStatusEffectResultNotify",
            ) {
                // info!("{:?}", pkt);
                party_tracker.borrow_mut().add(
                    pkt.raid_instance_id,
                    pkt.party_instance_id,
                    pkt.character_id,
                    0,
                    None,
                );
            }
        }
        Pkt::StatusEffectAddNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTStatusEffectAddNotify::new,
                "PKTStatusEffectAddNotify",
            ) {
                let status_effect = entity_tracker.build_and_register_status_effect(
                    &pkt.status_effect_data,
                    pkt.object_id,
                    Utc::now(),
                    Some(&state.encounter.entities),
                );
                if status_effect.status_effect_type == StatusEffectType::Shield {
                    let source = entity_tracker.get_source_entity(status_effect.source_id);
                    let target_id =
                        if status_effect.target_type == StatusEffectTargetType::Party {
                            id_tracker
                                .borrow()
                                .get_entity_id(status_effect.target_id)
                                .unwrap_or_default()
                        } else {
                            status_effect.target_id
                        };
                    let target = entity_tracker.get_source_entity(target_id);
                    state.on_boss_shield(&target, status_effect.value);
                    state.on_shield_applied(
                        &source,
                        &target,
                        status_effect.status_effect_id,
                        status_effect.value,
                    );
                }
            }
        }
        Pkt::StatusEffectRemoveNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTStatusEffectRemoveNotify::new,
                "PKTStatusEffectRemoveNotify",
            ) {
                let (is_shield, shields_broken, _left_workshop) =
                    status_tracker.remove_status_effects(
                        pkt.object_id,
                        pkt.status_effect_instance_ids,
                        pkt.reason,
                        StatusEffectTargetType::Local,
                    );
                if is_shield {
                    if shields_broken.is_empty() {
                        let target = entity_tracker.get_source_entity(pkt.object_id);
                        state.on_boss_shield(&target, 0);
                    } else {
                        for status_effect in shields_broken {
                            let change = status_effect.value;
                            on_shield_change(
                                entity_tracker,
                                &id_tracker,
                                state,
                                status_effect,
                                change,
                            );
                        }
                    }
                }
            }
        }
        Pkt::TriggerBossBattleStatus => {
            // need to hard code clown because it spawns before the trigger is sent???
            if state.encounter.current_boss_name.is_empty()
                || state.encounter.fight_start == 0
                || state.encounter.current_boss_name == "Saydon"
            {
                state.on_phase_transition(3, stats_api, event_emitter, connection_factory, version);
                debug_print(format_args!(
                    "phase: 3 - resetting encounter - TriggerBossBattleStatus"
                ));
            }
        }
        Pkt::TriggerStartNotify => {
            if let Some(pkt) =
                parse_pkt(&data, PKTTriggerStartNotify::new, "PKTTriggerStartNotify")
            {
                match pkt.signal {
                    57 | 59 | 61 | 63 | 74 | 76 => {
                        *party_freeze = true;
                        state.party_info = if let Some(party) = party_cache.take() {
                            party
                        } else {
                            update_party(&party_tracker, &entity_tracker)
                        };
                        state.raid_clear = true;
                        state.on_phase_transition(2, stats_api, event_emitter, connection_factory, version);
                        *raid_end_cd = Instant::now();
                        debug_print(format_args!("phase: 2 - clear - TriggerStartNotify"));
                    }
                    58 | 60 | 62 | 64 | 75 | 77 => {
                        *party_freeze = true;
                        state.party_info = if let Some(party) = party_cache.take() {
                            party
                        } else {
                            update_party(&party_tracker, &entity_tracker)
                        };
                        state.raid_clear = false;
                        state.on_phase_transition(4, stats_api, event_emitter, connection_factory, version);
                        *raid_end_cd = Instant::now();
                        debug_print(format_args!("phase: 4 - wipe - TriggerStartNotify"));
                    }
                    27 | 10 | 11 => {
                        // debug_print(format_args!("old rdps sync time - {}", pkt.trigger_signal_type));
                    }
                    _ => {}
                }
            }
        }
        Pkt::ZoneMemberLoadStatusNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTZoneMemberLoadStatusNotify::new,
                "PKTZoneMemberLoadStatusNotify",
            ) {
                stats_api.valid_zone = VALID_ZONES.contains(&pkt.zone_id);

                if state.raid_difficulty_id >= pkt.zone_id && !state.raid_difficulty.is_empty()
                {
                    return Ok(());
                }

                debug_print(format_args!("raid zone id: {}", &pkt.zone_id));
                debug_print(format_args!("raid zone id: {}", &pkt.zone_level));
                
                match pkt.zone_level {
                    0 => {
                        state.raid_difficulty = "Normal".to_string();
                        state.raid_difficulty_id = 0;
                    }
                    1 => {
                        state.raid_difficulty = "Hard".to_string();
                        state.raid_difficulty_id = 1;
                    }
                    2 => {
                        state.raid_difficulty = "Inferno".to_string();
                        state.raid_difficulty_id = 2;
                    }
                    3 => {
                        state.raid_difficulty = "Challenge".to_string();
                        state.raid_difficulty_id = 3;
                    }
                    4 => {
                        state.raid_difficulty = "Solo".to_string();
                        state.raid_difficulty_id = 4;
                    }
                    5 => {
                        state.raid_difficulty = "The First".to_string();
                        state.raid_difficulty_id = 5;
                    }
                    _ => {}
                }
            }
        }
        Pkt::ZoneObjectUnpublishNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTZoneObjectUnpublishNotify::new,
                "PKTZoneObjectUnpublishNotify",
            ) {
                status_tracker.remove_local_object(pkt.object_id);
            }
        }
        Pkt::StatusEffectSyncDataNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTStatusEffectSyncDataNotify::new,
                "PKTStatusEffectSyncDataNotify",
            ) {
                let (status_effect, old_value) =
                    status_tracker.sync_status_effect(
                        pkt.status_effect_instance_id,
                        pkt.character_id,
                        pkt.object_id,
                        pkt.value,
                        entity_tracker.local_character_id,
                    );
                if let Some(status_effect) = status_effect {
                    if status_effect.status_effect_type == StatusEffectType::Shield {
                        let change = old_value
                            .checked_sub(status_effect.value)
                            .unwrap_or_default();
                        on_shield_change(
                            entity_tracker,
                            &id_tracker,
                            state,
                            status_effect,
                            change,
                        );
                    }
                }
            }
        }
        Pkt::TroopMemberUpdateMinNotify => {
            if let Some(pkt) = parse_pkt(
                &data,
                PKTTroopMemberUpdateMinNotify::new,
                "PKTTroopMemberUpdateMinNotify",
            ) {
                // info!("{:?}", pkt);
                if let Some(object_id) = id_tracker.borrow().get_entity_id(pkt.character_id) {
                    if let Some(entity) = entity_tracker.get_entity_ref(object_id) {
                        state
                            .encounter
                            .entities
                            .entry(entity.name.clone())
                            .and_modify(|e| {
                                e.current_hp = pkt.cur_hp;
                                e.max_hp = pkt.max_hp;
                            });
                    }
                    for se in pkt.status_effect_datas.iter() {
                        let val = get_status_effect_value(&se.value);
                        let (status_effect, old_value) =
                            status_tracker.sync_status_effect(
                                se.status_effect_instance_id,
                                pkt.character_id,
                                object_id,
                                val,
                                entity_tracker.local_character_id,
                            );
                        if let Some(status_effect) = status_effect {
                            if status_effect.status_effect_type == StatusEffectType::Shield {
                                let change = old_value
                                    .checked_sub(status_effect.value)
                                    .unwrap_or_default();
                                on_shield_change(
                                    entity_tracker,
                                    &id_tracker,
                                    state,
                                    status_effect,
                                    change,
                                );
                            }
                        }
                    }
                }
            }
        }
        Pkt::NewTransit => {
            if let Some(pkt) = parse_pkt(&data, PKTNewTransit::new, "PKTNewZoneKey") {
                damage_handler.update_zone_instance_id(pkt.channel_id);
            }
        }
        _ => {}
    }

    Ok(())
}