use chrono::Utc;
use hashbrown::HashMap;
use log::{info, warn};
use meter_core::packets::definitions::PKTIdentityGaugeChangeNotify;
use moka::sync::Cache;
use rsntp::SntpClient;
use std::cmp::max;
use std::default::Default;
use std::sync::Arc;

use crate::abstractions::{ConnectionFactory, EventEmitter};
use crate::models::events::{ClearEncounter, PhaseTransition, RaidStart, ZoneChange};
use tokio::task;

use crate::parser::entity_tracker::EntityTracker;
use crate::models::*;
use crate::parser::skill_tracker::SkillTracker;
use crate::parser::stats_api::{PlayerStats, StatsApi};
use crate::parser::utils::*;

const RDPS_VALID_LIMIT: i64 = 25_000;

#[derive(Debug)]
pub struct EncounterState {
    pub encounter: Encounter,
    pub resetting: bool,
    pub boss_dead_update: bool,
    pub saved: bool,
    pub raid_clear: bool,
    pub party_info: Vec<Vec<String>>,
    pub raid_difficulty: String,
    pub raid_difficulty_id: u32,
    pub boss_only_damage: bool,
    pub region: Option<String>,
    pub rdps_valid: bool,
    pub skill_tracker: SkillTracker,
    pub damage_is_valid: bool,
    prev_stagger: i32,
    damage_log: HashMap<String, Vec<(i64, i64)>>,
    identity_log: HashMap<String, IdentityLog>,
    cast_log: HashMap<String, HashMap<u32, Vec<i32>>>,
    boss_hp_log: HashMap<String, Vec<BossHpLog>>,
    stagger_log: Vec<(i32, f32)>,
    stagger_intervals: Vec<(i32, i32)>,
    sntp_client: SntpClient,
    ntp_fight_start: i64,
    custom_id_map: HashMap<u32, u32>,
}

impl EncounterState {
    pub fn new() -> EncounterState {
        EncounterState {
            encounter: Encounter::default(),
            resetting: false,
            raid_clear: false,
            boss_dead_update: false,
            saved: false,

            prev_stagger: 0,
            damage_log: HashMap::new(),
            identity_log: HashMap::new(),
            boss_hp_log: HashMap::new(),
            cast_log: HashMap::new(),
            stagger_log: Vec::new(),
            stagger_intervals: Vec::new(),

            party_info: Vec::new(),
            raid_difficulty: "".to_string(),
            raid_difficulty_id: 0,
            boss_only_damage: false,
            region: None,

            sntp_client: SntpClient::new(),
            ntp_fight_start: 0,

            // todo
            rdps_valid: false,

            skill_tracker: SkillTracker::new(),

            custom_id_map: HashMap::new(),

            damage_is_valid: true,
        }
    }

    // keep all player entities, reset all stats
    pub fn soft_reset(&mut self, keep_bosses: bool) {
        let clone = self.encounter.clone();

        self.encounter.fight_start = 0;
        self.encounter.boss_only_damage = self.boss_only_damage;
        self.encounter.entities = HashMap::new();
        self.encounter.current_boss_name = "".to_string();
        self.encounter.encounter_damage_stats = Default::default();
        self.prev_stagger = 0;
        self.raid_clear = false;

        self.damage_log = HashMap::new();
        self.identity_log = HashMap::new();
        self.cast_log = HashMap::new();
        self.boss_hp_log = HashMap::new();
        self.stagger_log = Vec::new();
        self.stagger_intervals = Vec::new();
        self.party_info = Vec::new();

        self.ntp_fight_start = 0;

        self.rdps_valid = false;

        self.skill_tracker = SkillTracker::new();

        self.custom_id_map = HashMap::new();
        
        for (key, entity) in clone.entities.into_iter().filter(|(_, e)| {
            e.entity_type == EntityType::Player
                || (keep_bosses && e.entity_type == EntityType::Boss)
        }) {
            self.encounter.entities.insert(
                key,
                EncounterEntity {
                    name: entity.name,
                    id: entity.id,
                    character_id: entity.character_id,
                    npc_id: entity.npc_id,
                    class: entity.class,
                    class_id: entity.class_id,
                    entity_type: entity.entity_type,
                    gear_score: entity.gear_score,
                    max_hp: entity.max_hp,
                    current_hp: entity.current_hp,
                    is_dead: entity.is_dead,
                    ..Default::default()
                },
            );
        }
    }

    pub fn set_raid_difficulty(&mut self, zone_level: u32) {
        match zone_level {
            0 => {
                self.raid_difficulty = "Normal".to_string();
            }
            1 => {
                self.raid_difficulty = "Hard".to_string();
            }
            2 => {
                self.raid_difficulty = "Inferno".to_string();
            }
            3 => {
                self.raid_difficulty = "Challenge".to_string();
            }
            4 => {
                self.raid_difficulty = "Solo".to_string();
            }
            5 => {
                self.raid_difficulty = "The First".to_string();
            }
            _ => {}
        }

        self.raid_difficulty_id = zone_level;
    }

    // update local player as we get more info
    pub fn update_local_player(&mut self, entity: &Entity) {
        let entities = &mut self.encounter.entities;

        // we replace the existing local player if it exists, since its name might have changed (from hex or "You" to character name)
        if let Some(mut local) = entities.remove(&self.encounter.local_player) {
            // update local player name, insert back into encounter
            self.encounter.local_player.clone_from(&entity.name);
            update_player_entity(&mut local, entity);
            entities.insert(self.encounter.local_player.clone(), local);
        } else {
            // cannot find old local player by name, so we look by local player's entity id
            // this can happen when the user started meter late
            let old_local = entities
                .iter()
                .find(|(_, e)| e.id == entity.id)
                .map(|(key, _)| key.clone());

            // if we find the old local player, we update its name and insert back into encounter
            if let Some(old_local) = old_local {
                let mut new_local = entities[&old_local].clone();
                update_player_entity(&mut new_local, entity);
                entities.remove(&old_local);
                self.encounter.local_player.clone_from(&entity.name);
                entities.insert(self.encounter.local_player.clone(), new_local);
            }
        }
    }

    pub fn on_init_env<E: EventEmitter, C: ConnectionFactory>(
        &mut self,
        entity: Entity,
        stats_api: &StatsApi,
        event_emitter: Arc<E>,
        connection_factory: Arc<C>,
        version: &str
    ) {
        // if not already saved to db, we save again
        if !self.saved && !self.encounter.current_boss_name.is_empty() {
            self.save_to_db(stats_api, false, event_emitter.clone(), connection_factory, version);
        }

        let entities = &mut self.encounter.entities;

        // replace or insert local player
        if let Some(mut local_player) = entities.remove(&self.encounter.local_player)
        {
            update_player_entity(&mut local_player, &entity);
            entities.insert(entity.name.clone(), local_player);
        } else {
            let entity = encounter_entity_from_entity(&entity);
            entities.insert(entity.name.clone(), entity);
        }
        self.encounter.local_player = entity.name;

        // remove unrelated entities
        entities.retain(|_, e| {
            e.name == self.encounter.local_player || e.damage_stats.damage_dealt > 0
        });

        event_emitter
            .emit(ZoneChange {})
            .expect("failed to emit zone-change");

        self.soft_reset(false);
    }

    pub fn on_phase_transition<E: EventEmitter, C: ConnectionFactory>(
        &mut self,
        phase_code: i32,
        stats_api: &mut StatsApi,
        event_emitter: Arc<E>,
        connection_factory: Arc<C>,
        version: &str) {
        
        event_emitter.emit(PhaseTransition { phase_code })
            .expect("failed to emit phase-transition");

        match phase_code {
            0 | 2 | 3 | 4 => {
                if !self.encounter.current_boss_name.is_empty() {
                    stats_api.send_raid_info(&version, self);
                    if phase_code == 0 {
                        stats_api.valid_zone = false;
                    }
                    self.save_to_db(stats_api, false, event_emitter, connection_factory, version);
                    self.saved = true;
                }
                self.resetting = true;
            }
            _ => (),
        }
    }

    // replace local player
    pub fn on_init_pc(&mut self, entity: Entity, hp: i64, max_hp: i64) {
        let entities = &mut self.encounter.entities;

        entities.remove(&self.encounter.local_player);
        self.encounter.local_player.clone_from(&entity.name);
        let mut player = encounter_entity_from_entity(&entity);
        player.current_hp = hp;
        player.max_hp = max_hp;
        entities.insert(player.name.clone(), player);
    }

    // add or update player to encounter
    pub fn on_new_pc(&mut self, entity: Entity, hp: i64, max_hp: i64) {
        self.encounter
            .entities
            .entry(entity.name.clone())
            .and_modify(|player| {
                player.id = entity.id;
                player.gear_score = entity.gear_level;
                player.current_hp = hp;
                player.max_hp = max_hp;
                if entity.character_id > 0 {
                    player.character_id = entity.character_id;
                }
            })
            .or_insert_with(|| {
                let mut player = encounter_entity_from_entity(&entity);
                player.current_hp = hp;
                player.max_hp = max_hp;
                player
            });
    }

    // add or update npc to encounter
    // we set current boss if npc matches criteria
    pub fn on_new_npc(&mut self, entity: Entity, hp: i64, max_hp: i64) {
        let entity_name = entity.name.clone();
        self.encounter
            .entities
            .entry(entity_name.clone())
            .and_modify(|e| {
                if entity.entity_type != EntityType::Boss && e.entity_type != EntityType::Boss {
                    e.npc_id = entity.npc_id;
                    e.id = entity.id;
                    e.current_hp = hp;
                    e.max_hp = max_hp;
                } else if entity.entity_type == EntityType::Boss && e.entity_type == EntityType::Npc
                {
                    e.entity_type = EntityType::Boss;
                    e.npc_id = entity.npc_id;
                    e.id = entity.id;
                    e.current_hp = hp;
                    e.max_hp = max_hp;
                }
            })
            .or_insert_with(|| {
                let mut npc = encounter_entity_from_entity(&entity);
                npc.current_hp = hp;
                npc.max_hp = max_hp;
                npc
            });

        if let Some(npc) = self.encounter.entities.get(&entity_name) {
            if npc.entity_type == EntityType::Boss {
                // if current encounter has no boss, we set the boss
                // if current encounter has a boss, we check if new boss has more max hp, or if current boss is dead
                self.encounter.current_boss_name = if self
                    .encounter
                    .entities
                    .get(&self.encounter.current_boss_name)
                    .map_or(true, |boss| npc.max_hp >= boss.max_hp || boss.is_dead)
                {
                    entity_name
                } else {
                    self.encounter.current_boss_name.clone()
                };
            }
        }
    }

    pub fn on_death(&mut self, dead_entity: &Entity) {
        let entity = self
            .encounter
            .entities
            .entry(dead_entity.name.clone())
            .or_insert_with(|| encounter_entity_from_entity(dead_entity));

        if (dead_entity.entity_type != EntityType::Player
            && dead_entity.entity_type != EntityType::Boss)
            || entity.id != dead_entity.id
            || (entity.entity_type == EntityType::Boss && entity.npc_id != dead_entity.npc_id)
        {
            return;
        }

        if entity.entity_type == EntityType::Boss
            && dead_entity.entity_type == EntityType::Boss
            && entity.name == self.encounter.current_boss_name
            && !entity.is_dead
        {
            self.boss_dead_update = true;
        }

        entity.current_hp = 0;
        entity.is_dead = true;
        entity.damage_stats.deaths += 1;
        entity.damage_stats.death_time = Utc::now().timestamp_millis();
    }

    pub fn on_skill_start(
        &mut self,
        source_entity: &Entity,
        skill_id: u32,
        tripod_index: Option<TripodIndex>,
        tripod_level: Option<TripodLevel>,
        timestamp: i64,
    ) -> (u32, Option<Vec<u32>>) {
        // do not track skills if encounter not started
        if self.encounter.fight_start == 0 {
            return (0, None);
        }

        let (skill_name, skill_icon, summons) = get_skill_name_and_icon(
            Some(skill_id),
            None,
            &self.skill_tracker,
            source_entity.id,
        );

        let mut tripod_change = false;
        let entity = self
            .encounter
            .entities
            .entry(source_entity.name.clone())
            .or_insert_with(|| {
               
                let mut entity = encounter_entity_from_entity(source_entity);
                entity.skill_stats = SkillStats {
                    casts: 0,
                    ..Default::default()
                };
                entity.skills = HashMap::from([(
                    skill_id,
                    Skill {
                        id: skill_id,
                        name: {
                            if skill_name.is_empty() {
                                skill_id.to_string()
                            } else {
                                skill_name.clone()
                            }
                        },
                        icon: skill_icon,
                        tripod_index,
                        tripod_level,
                        summon_sources: summons,
                        casts: 0,
                        ..Default::default()
                    },
                )]);
                tripod_change = true;
                entity
            });

        if entity.class_id == 0
            && source_entity.entity_type == EntityType::Player
            && source_entity.class_id > 0
        {
            entity.class_id = source_entity.class_id;
            entity.class = get_class_from_id(&source_entity.class_id);
        }

        entity.is_dead = false;
        entity.skill_stats.casts += 1;

        let relative_timestamp = if self.encounter.fight_start == 0 {
            0
        } else {
            (timestamp - self.encounter.fight_start) as i32
        };

        // if skills have different ids but the same name, we group them together
        // dunno if this is right approach xd
        let mut skill_id = skill_id;
        let mut skill_summon_sources: Option<Vec<u32>> = None;
        if let Some(skill) = entity.skills.get_mut(&skill_id) {
            skill.casts += 1;
            tripod_change = check_tripod_index_change(skill.tripod_index, tripod_index)
                || check_tripod_level_change(skill.tripod_level, tripod_level);
            skill.tripod_index = tripod_index;
            skill.tripod_level = tripod_level;
            skill_summon_sources.clone_from(&skill.summon_sources);
        } else if let Some(skill) = entity
            .skills
            .values_mut()
            .find(|s| s.name == skill_name.clone())
        {
            skill.casts += 1;
            skill_id = skill.id;
            tripod_change = check_tripod_index_change(skill.tripod_index, tripod_index)
                || check_tripod_level_change(skill.tripod_level, tripod_level);
            skill.tripod_index = tripod_index;
            skill.tripod_level = tripod_level;
            skill_summon_sources.clone_from(&skill.summon_sources);
        } else {
            let (skill_name, skill_icon, summons) = get_skill_name_and_icon(
                Some(skill_id),
                None,
                &self.skill_tracker,
                source_entity.id,
            );
            skill_summon_sources.clone_from(&summons);
            entity.skills.insert(
                skill_id,
                Skill {
                    id: skill_id,
                    name: {
                        if skill_name.is_empty() {
                            skill_id.to_string()
                        } else {
                            skill_name
                        }
                    },
                    icon: skill_icon,
                    tripod_index,
                    tripod_level,
                    summon_sources: summons,
                    casts: 1,
                    ..Default::default()
                },
            );
            tripod_change = true;
        }
        if tripod_change {
            // let mut tripod_data: Vec<TripodData> = vec![];
            if let (Some(tripod_index), Some(tripod_level)) = (tripod_index, tripod_level) {
                let mut indexes = vec![tripod_index.first];
                if tripod_index.second != 0 {
                    indexes.push(tripod_index.second + 3);
                }
                // third row should never be set if second is not set
                if tripod_index.third != 0 {
                    indexes.push(tripod_index.third + 6);
                }
                // let levels = [tripod_level.first, tripod_level.second, tripod_level.third];
                // if let Some(effect) = SKILL_FEATURE_DATA.get(&skill_id) {
                //     for i in 0..indexes.len() {
                //         if let Some(entries) = effect.tripods.get(&indexes[i]) {
                //             let mut options: Vec<SkillFeatureOption> = vec![];
                //             for entry in &entries.entries {
                //                 if entry.level > 0 && entry.level == levels[i] {
                //                     options.push(entry.clone());
                //                 }
                //             }
                //             tripod_data.push(TripodData {
                //                 index: indexes[i],
                //                 options,
                //             });
                //         }
                //     }
                // }
            }

            // if !tripod_data.is_empty() {
            //     entity.skills.entry(skill_id).and_modify(|e| {
            //         e.tripod_data = Some(tripod_data);
            //     });
            // }
        }
        self.cast_log
            .entry(entity.name.clone())
            .or_default()
            .entry(skill_id)
            .or_default()
            .push(relative_timestamp);

        (skill_id, skill_summon_sources)
    }

    pub fn on_damage(
        &mut self,
        dmg_src_entity: &Entity,
        proj_entity: &Entity,
        dmg_target_entity: &Entity,
        damage_data: DamageData,
        se_on_source: Vec<StatusEffectDetails>,
        se_on_target: Vec<StatusEffectDetails>,
        timestamp: i64,
        event_emitter: &impl EventEmitter
    ) {
        let mut skill_id = damage_data.skill_id;
        let mut skill_effect_id = damage_data.skill_effect_id;
        let hit_flag = HitFlag::from(damage_data.modifier & 0xf);
        let hit_option_raw = (damage_data.modifier >> 4) & 0x7;
        let hit_option = HitOption::from(hit_option_raw);
        let entities = &mut self.encounter.entities;
        let damage_stats = &mut self.encounter.encounter_damage_stats;

        if hit_flag == HitFlag::Invincible {
            return;
        }

        if hit_flag == HitFlag::DamageShare
            && damage_data.skill_id.is_none()
            && damage_data.skill_effect_id.is_none()
        {
            return;
        }

        if proj_entity.entity_type == EntityType::Projectile
            && is_battle_item(&proj_entity.skill_effect_id, "attack")
        {
            skill_effect_id.replace(proj_entity.skill_effect_id);
        }

        let mut source_entity = entities
            .entry(dmg_src_entity.name.clone())
            .or_insert_with(|| encounter_entity_from_entity(dmg_src_entity))
            .to_owned();

        let mut target_entity = entities
            .entry(dmg_target_entity.name.clone())
            .or_insert_with(|| {
                let mut target_entity = encounter_entity_from_entity(dmg_target_entity);
                target_entity.current_hp = damage_data.target_current_hp;
                target_entity.max_hp = damage_data.target_max_hp;
                target_entity
            })
            .to_owned();

        // if boss only damage is enabled
        // check if target is boss and not player
        // check if target is player and source is boss
        if self.boss_only_damage
            && ((target_entity.entity_type != EntityType::Boss
                && target_entity.entity_type != EntityType::Player)
                || (target_entity.entity_type == EntityType::Player
                    && source_entity.entity_type != EntityType::Boss))
        {
            return;
        }

        if self.encounter.fight_start == 0 {
            self.encounter.fight_start = timestamp;
            self.skill_tracker.fight_start = timestamp;
            if let Some(skill_id) = skill_id {
                // source_entity.entity_type == EntityType::Player && skill_id > 0

                self.skill_tracker.new_cast(
                    source_entity.id,
                    skill_id,
                    None,
                    timestamp,
                );
            }

            if let Ok(result) = self.sntp_client.synchronize("time.cloudflare.com") {
                let dt = result.datetime().into_chrono_datetime().unwrap_or_default();
                self.ntp_fight_start = dt.timestamp_millis();
                // debug_print(format_args!("fight start local: {}, ntp: {}", Utc::now().to_rfc3339(), dt.to_rfc3339()));
            };

            self.encounter.boss_only_damage = self.boss_only_damage;
            event_emitter
                .emit(RaidStart { timestamp })
                .expect("failed to emit raid-start");
        }

        self.encounter.last_combat_packet = timestamp;

        source_entity.id = dmg_src_entity.id;

        if target_entity.id == dmg_target_entity.id {
            target_entity.current_hp = damage_data.target_current_hp;
            target_entity.max_hp = damage_data.target_max_hp;
        }

        let mut damage = damage_data.damage;
        if target_entity.entity_type != EntityType::Player && damage_data.target_current_hp < 0 {
            damage += damage_data.target_current_hp;
        }

        // let mut skill_id = if damage_data.skill_id != 0 {
        //     damage_data.skill_id
        // } else {
        //     skill_effect_id.unwrap()
        // };

        let skill_data = skill_id.and_then(|id| SKILL_DATA.get(&id));
        let mut skill_name = "".to_string();
        let mut skill_summon_sources: Option<Vec<u32>> = None;

        if let Some(skill_data) = skill_data {
            skill_name = skill_data.name.clone().unwrap_or_default();
            skill_summon_sources.clone_from(&skill_data.summon_source_skills);
        }

        if skill_name.is_empty() {
            (skill_name, _, skill_summon_sources) = get_skill_name_and_icon(
                skill_id,
                skill_effect_id,
                &self.skill_tracker,
                source_entity.id,
            );
        }
        
        let relative_timestamp = (timestamp - self.encounter.fight_start) as i32;

        let default_skill_id = skill_id.or(skill_effect_id).unwrap_or_default();

        if !source_entity.skills.contains_key(&default_skill_id) {
            let skill = source_entity.skills.values().find(|&s| s.name == *skill_name);

            if let Some(skill) = skill
            {
                skill_id = Some(skill.id);
            } else {
                let (skill_name, skill_icon, _) = get_skill_name_and_icon(
                    skill_id,
                    skill_effect_id,
                    &self.skill_tracker,
                    source_entity.id,
                );

                let default_skill_name = if skill_name.is_empty() {
                    default_skill_id.to_string()
                } else {
                    skill_name.clone()
                };

                let skill = Skill {
                    id: default_skill_id,
                    name: default_skill_name,
                    icon: skill_icon,
                    summon_sources: skill_summon_sources.clone(),
                    casts: 1,
                    ..Default::default()
                };

                source_entity.skills.insert(default_skill_id, skill);
            }
        }

        let skill = source_entity.skills.get_mut(&default_skill_id).unwrap();

        let mut skill_hit = SkillHit {
            damage,
            timestamp: relative_timestamp as i64,
            ..Default::default()
        };

        skill.total_damage += damage;
        if damage > skill.max_damage {
            skill.max_damage = damage;
        }
        skill.last_timestamp = timestamp;

        let source_damage_stats = &mut source_entity.damage_stats;

        source_damage_stats.damage_dealt += damage;

        let is_hyper_awakening = is_hyper_awakening_skill(skill.id);
        if is_hyper_awakening {
            source_damage_stats.hyper_awakening_damage += damage;
        }

        target_entity.damage_stats.damage_taken += damage;

        source_entity.skill_stats.hits += 1;
        skill.hits += 1;

        if hit_flag == HitFlag::Critical || hit_flag == HitFlag::DotCritical {
            source_entity.skill_stats.crits += 1;
            source_damage_stats.crit_damage += damage;
            skill.crits += 1;
            skill.crit_damage += damage;
            skill_hit.crit = true;
        }
        if hit_option == HitOption::BackAttack {
            source_entity.skill_stats.back_attacks += 1;
            source_damage_stats.back_attack_damage += damage;
            skill.back_attacks += 1;
            skill.back_attack_damage += damage;
            skill_hit.back_attack = true;
        }
        if hit_option == HitOption::FrontalAttack {
            source_entity.skill_stats.front_attacks += 1;
            source_damage_stats.front_attack_damage += damage;
            skill.front_attacks += 1;
            skill.front_attack_damage += damage;
            skill_hit.front_attack = true;
        }

        if source_entity.entity_type == EntityType::Player {
            damage_stats.total_damage_dealt += damage;
            damage_stats.top_damage_dealt = max(
                damage_stats.top_damage_dealt,
                source_damage_stats.damage_dealt,
            );

            self.damage_log
                .entry(source_entity.name.clone())
                .or_default()
                .push((timestamp, damage));

            let mut is_buffed_by_support = false;
            let mut is_buffed_by_identity = false;
            let mut is_debuffed_by_support = false;
            let mut is_buffed_by_hat = false;
            let se_on_source_ids = se_on_source
                .iter()
                .map(|se| {
                    if se.custom_id > 0 {
                        self.custom_id_map.insert(se.custom_id, se.status_effect_id);
                        se.custom_id
                    } else {
                        se.status_effect_id
                    }
                    // map_status_effect(se, &mut self.custom_id_map))
                })
                .collect::<Vec<_>>();

            for buff_id in se_on_source_ids.iter() {
                if !damage_stats.unknown_buffs.contains(buff_id)
                    && !damage_stats.buffs.contains_key(buff_id)
                {
                    let mut source_id: Option<u32> = None;
                    let original_buff_id = if let Some(deref_id) = self.custom_id_map.get(buff_id) {
                        source_id = Some(get_skill_id(*buff_id));
                        *deref_id
                    } else {
                        *buff_id
                    };

                    if let Some(status_effect) = get_status_effect_data(original_buff_id, source_id)
                    {
                        damage_stats.buffs.insert(*buff_id, status_effect);
                    } else {
                        damage_stats.unknown_buffs.insert(*buff_id);
                    }
                }
                if !is_buffed_by_support && !is_hat_buff(buff_id) {
                    if let Some(buff) = damage_stats.buffs.get(buff_id) {
                        if let Some(skill) = buff.source.skill.as_ref() {
                            is_buffed_by_support = is_support_class_id(skill.class_id)
                                && buff.buff_type & StatusEffectBuffTypeFlags::DMG.bits() != 0
                                && buff.target == StatusEffectTarget::PARTY
                                && (buff.buff_category == "classskill"
                                    || buff.buff_category == "arkpassive");
                        }
                    }
                }
                if !is_buffed_by_identity {
                    if let Some(buff) = damage_stats.buffs.get(buff_id) {
                        if let Some(skill) = buff.source.skill.as_ref() {
                            is_buffed_by_identity = is_support_class_id(skill.class_id)
                                && buff.buff_type & StatusEffectBuffTypeFlags::DMG.bits() != 0
                                && buff.target == StatusEffectTarget::PARTY
                                && buff.buff_category == "identity";
                        }
                    }
                }

                if !is_buffed_by_hat && is_hat_buff(buff_id) {
                    is_buffed_by_hat = true;
                }
            }
            let se_on_target_ids = se_on_target
                .iter()
                .map(|se| {
                    if se.custom_id > 0 {
                        self.custom_id_map.insert(se.custom_id, se.status_effect_id);
                        se.custom_id
                    } else {
                        se.status_effect_id
                    }
                    // map_status_effect(se, &mut self.custom_id_map))
                })
                .collect::<Vec<_>>();

            for debuff_id in se_on_target_ids.iter() {
                if !damage_stats.unknown_buffs.contains(debuff_id)
                    && !damage_stats.debuffs.contains_key(debuff_id)
                {
                    let mut source_id: Option<u32> = None;
                    let original_debuff_id =
                        if let Some(deref_id) = self.custom_id_map.get(debuff_id) {
                            source_id = Some(get_skill_id(*debuff_id));
                            *deref_id
                        } else {
                            *debuff_id
                        };

                    if let Some(status_effect) =
                        get_status_effect_data(original_debuff_id, source_id)
                    {
                        damage_stats.debuffs.insert(*debuff_id, status_effect);
                    } else {
                        damage_stats.unknown_buffs.insert(*debuff_id);
                    }
                }

                if !is_debuffed_by_support {
                    if let Some(debuff) =
                        damage_stats.debuffs.get(debuff_id)
                    {
                        if let Some(skill) = debuff.source.skill.as_ref() {
                            is_debuffed_by_support = is_support_class_id(skill.class_id)
                                && debuff.buff_type & StatusEffectBuffTypeFlags::DMG.bits() != 0
                                && debuff.target == StatusEffectTarget::PARTY;
                        }
                    }
                }
            }

            if is_buffed_by_support && !is_hyper_awakening {
                skill.buffed_by_support += damage;
                source_damage_stats.buffed_by_support += damage;
            }

            if is_buffed_by_identity && !is_hyper_awakening {
                skill.buffed_by_identity += damage;
                source_damage_stats.buffed_by_identity += damage;
            }

            if is_debuffed_by_support && !is_hyper_awakening {
                skill.debuffed_by_support += damage;
                source_damage_stats.debuffed_by_support += damage;
            }
            
            if is_buffed_by_hat {
                skill.buffed_by_hat += damage;
                source_damage_stats.buffed_by_hat += damage;
            }

            let stabilized_status_active =
                (source_entity.current_hp as f64 / source_entity.max_hp as f64) > 0.65;
            let mut filtered_se_on_source_ids: Vec<u32> = vec![];

            for buff_id in se_on_source_ids.iter() {
                if is_hyper_awakening && !is_hat_buff(buff_id) {
                    continue;
                }

                if let Some(buff) = damage_stats.buffs.get(buff_id) {
                    if !stabilized_status_active && buff.source.name.contains("Stabilized Status") {
                        continue;
                    }
                }

                filtered_se_on_source_ids.push(*buff_id);

                skill
                    .buffed_by
                    .entry(*buff_id)
                    .and_modify(|e| *e += damage)
                    .or_insert(damage);
                source_entity
                    .damage_stats
                    .buffed_by
                    .entry(*buff_id)
                    .and_modify(|e| *e += damage)
                    .or_insert(damage);
            }
            for debuff_id in se_on_target_ids.iter() {
                if is_hyper_awakening {
                    break;
                }

                skill
                    .debuffed_by
                    .entry(*debuff_id)
                    .and_modify(|e| *e += damage)
                    .or_insert(damage);
                source_entity
                    .damage_stats
                    .debuffed_by
                    .entry(*debuff_id)
                    .and_modify(|e| *e += damage)
                    .or_insert(damage);
            }

            skill_hit.buffed_by = filtered_se_on_source_ids;
            if !is_hyper_awakening {
                skill_hit.debuffed_by = se_on_target_ids;
            }

        }

        if target_entity.entity_type == EntityType::Player {
            damage_stats.total_damage_taken += damage;
            damage_stats.top_damage_taken = max(
                damage_stats.top_damage_taken,
                target_entity.damage_stats.damage_taken,
            );
        }
        // update current_boss
        else if target_entity.entity_type == EntityType::Boss {
            self.encounter
                .current_boss_name
                .clone_from(&target_entity.name);
            target_entity.id = dmg_target_entity.id;
            target_entity.npc_id = dmg_target_entity.npc_id;

            let log = self
                .boss_hp_log
                .entry(target_entity.name.clone())
                .or_default();

            let current_hp = if target_entity.current_hp >= 0 {
                target_entity.current_hp + target_entity.current_shield as i64
            } else {
                0
            };
            let hp_percent = if target_entity.max_hp != 0 {
                current_hp as f32 / target_entity.max_hp as f32
            } else {
                0.0
            };

            let relative_timestamp_s = relative_timestamp / 1000;

            if log.is_empty() || log.last().unwrap().time != relative_timestamp_s {
                let entry = BossHpLog::new(relative_timestamp_s, current_hp, hp_percent);
                log.push(entry);
            } else {
                let last = log.last_mut().unwrap();
                last.hp = current_hp;
                last.p = hp_percent;
            }
        }

        if let Some(skill_id) = skill_id {
            self.skill_tracker.on_hit(
                source_entity.id,
                proj_entity.id,
                skill_id,
                skill_hit,
                skill_summon_sources,
            );
        }

        entities.insert(source_entity.name.clone(), source_entity);
        entities.insert(target_entity.name.clone(), target_entity);
    }

    pub fn on_counterattack(&mut self, source_entity: &Entity) {
        let entity = self
            .encounter
            .entities
            .entry(source_entity.name.clone())
            .or_insert_with(|| {
                let mut entity = encounter_entity_from_entity(source_entity);
                entity.skill_stats = SkillStats {
                    counters: 0,
                    ..Default::default()
                };
                entity
            });
        entity.skill_stats.counters += 1;
    }

    pub fn on_identity_gain(&mut self, pkt: &PKTIdentityGaugeChangeNotify) {
        if self.encounter.fight_start == 0 {
            return;
        }

        if self.encounter.local_player.is_empty() {
            if let Some((_, entity)) = self
                .encounter
                .entities
                .iter()
                .find(|(_, e)| e.id == pkt.player_id)
            {
                self.encounter.local_player.clone_from(&entity.name);
            } else {
                return;
            }
        }

        if let Some(entity) = self
            .encounter
            .entities
            .get_mut(&self.encounter.local_player)
        {
            self.identity_log
                .entry(entity.name.clone())
                .or_default()
                .push((
                    Utc::now().timestamp_millis(),
                    (
                        pkt.identity_gauge1,
                        pkt.identity_gauge2,
                        pkt.identity_gauge3,
                    ),
                ));
        }
    }

    pub fn on_boss_shield(&mut self, name: &str, shield: u64) {
        self.encounter
            .entities
            .entry(name.to_string())
            .and_modify(|e| {
                e.current_shield = shield;
            });
    }

    pub fn on_shield_applied(
        &mut self,
        source_entity: &Entity,
        target_entity: &Entity,
        buff_id: u32,
        shield: u64,
    ) {
        let entities = &mut self.encounter.entities;

        if source_entity.entity_type == EntityType::Player
            && target_entity.entity_type == EntityType::Player
        {
            let mut target_entity_state = entities
                .entry(target_entity.name.clone())
                .or_insert_with(|| encounter_entity_from_entity(target_entity))
                .to_owned();
            let mut source_entity_state = entities
                .entry(source_entity.name.clone())
                .or_insert_with(|| encounter_entity_from_entity(source_entity))
                .to_owned();

            if !self.encounter.encounter_damage_stats.applied_shield_buffs.contains_key(&buff_id)
            {
                let mut source_id: Option<u32> = None;
                let original_buff_id = if let Some(deref_id) = self.custom_id_map.get(&buff_id) {
                    source_id = Some(get_skill_id(buff_id));
                    *deref_id
                } else {
                    buff_id
                };

                if let Some(status_effect) = get_status_effect_data(original_buff_id, source_id) {
                    self.encounter
                        .encounter_damage_stats
                        .applied_shield_buffs
                        .insert(buff_id, status_effect);
                }
            }

            if source_entity.id == target_entity.id {
                source_entity_state.damage_stats.shields_received += shield;
                source_entity_state.damage_stats.shields_given += shield;
                source_entity_state
                    .damage_stats
                    .shields_given_by
                    .entry(buff_id)
                    .and_modify(|e| *e += shield)
                    .or_insert(shield);
                source_entity_state
                    .damage_stats
                    .shields_received_by
                    .entry(buff_id)
                    .and_modify(|e| *e += shield)
                    .or_insert(shield);

                self.encounter
                    .entities
                    .insert(source_entity_state.name.clone(), source_entity_state);
            } else {
                target_entity_state.damage_stats.shields_received += shield;
                source_entity_state.damage_stats.shields_given += shield;
                source_entity_state
                    .damage_stats
                    .shields_given_by
                    .entry(buff_id)
                    .and_modify(|e| *e += shield)
                    .or_insert(shield);
                target_entity_state
                    .damage_stats
                    .shields_received_by
                    .entry(buff_id)
                    .and_modify(|e| *e += shield)
                    .or_insert(shield);

                self.encounter
                    .entities
                    .insert(target_entity_state.name.clone(), target_entity_state);
                self.encounter
                    .entities
                    .insert(source_entity_state.name.clone(), source_entity_state);
            }

            self.encounter.encounter_damage_stats.total_shielding += shield;
        }
    }

    pub fn on_shield_used(
        &mut self,
        source_entity: &Entity,
        target_entity: &Entity,
        buff_id: u32,
        shield_removed: u64,
    ) {
    if source_entity.entity_type != EntityType::Player
        || target_entity.entity_type != EntityType::Player
    {
        return;
    }

    let entities = &mut self.encounter.entities;

    let mut target_entity_state = entities
        .entry(target_entity.name.clone())
        .or_insert_with(|| encounter_entity_from_entity(target_entity))
        .to_owned();

    let mut source_entity_state = entities
        .entry(source_entity.name.clone())
        .or_insert_with(|| encounter_entity_from_entity(source_entity))
        .to_owned();

    let source_damage_stats = &mut source_entity_state.damage_stats;
    let target_damage_stats = &mut target_entity_state.damage_stats;

    if source_entity.id == target_entity.id {
        source_damage_stats.damage_absorbed += shield_removed;
        source_damage_stats.damage_absorbed_on_others += shield_removed;
        source_damage_stats.damage_absorbed_by
            .entry(buff_id)
            .and_modify(|e| *e += shield_removed)
            .or_insert(shield_removed);
        source_damage_stats
            .damage_absorbed_on_others_by
            .entry(buff_id)
            .and_modify(|e| *e += shield_removed)
            .or_insert(shield_removed);

        self.encounter
            .entities
            .insert(source_entity_state.name.clone(), source_entity_state);
    } else {
        target_damage_stats.damage_absorbed += shield_removed;
        target_damage_stats.damage_absorbed_on_others += shield_removed;
        target_damage_stats.damage_absorbed_by
            .entry(buff_id)
            .and_modify(|e| *e += shield_removed)
            .or_insert(shield_removed);
        source_damage_stats
            .damage_absorbed_on_others_by
            .entry(buff_id)
            .and_modify(|e| *e += shield_removed)
            .or_insert(shield_removed);

        entities.insert(target_entity_state.name.clone(), target_entity_state);
        entities.insert(source_entity_state.name.clone(), source_entity_state);
    }

    self.encounter
        .encounter_damage_stats
        .total_effective_shielding += shield_removed;
    }

    pub fn save_to_db<E: EventEmitter, C: ConnectionFactory>(
        &mut self,
        stats_api: &StatsApi,
        manual: bool,
        event_emitter: Arc<E>,
        connection_factory: Arc<C>,
        version: &str
    ) {
        let entities = &mut self.encounter.entities;
        let current_boss_name = &self.encounter.current_boss_name;

        if !manual {
            if self.encounter.fight_start == 0
                || self.encounter.current_boss_name.is_empty()
                || !entities
                    .contains_key(current_boss_name)
                || !entities
                    .values()
                    .any(|entity| entity.is_player_with_stats())
            {
                return;
            }

            if let Some(current_boss) = entities
                .get(current_boss_name)
            {
                if current_boss.current_hp == current_boss.max_hp {
                    return;
                }
            }
        }

        if !self.damage_is_valid {
            warn!("damage decryption is invalid, not saving to db");
        }

        let mut encounter = self.encounter.clone();
        let prev_stagger = self.prev_stagger;
        let damage_log = self.damage_log.clone();
        let identity_log = self.identity_log.clone();
        let cast_log = self.cast_log.clone();
        let boss_hp_log = self.boss_hp_log.clone();
        let stagger_log = self.stagger_log.clone();
        let stagger_intervals = self.stagger_intervals.clone();
        let raid_clear = self.raid_clear;
        let party_info = self.party_info.clone();
        let raid_difficulty = self.raid_difficulty.clone();
        let region = self.region.clone();
        let ntp_fight_start = self.ntp_fight_start;
        let rdps_valid = self.rdps_valid;
        let skill_cast_log = self.skill_tracker.get_cast_log();
        let stats_api = stats_api.clone();

        info!(
            "saving to db - cleared: [{}], difficulty: [{}] {}",
            raid_clear, self.raid_difficulty, encounter.current_boss_name
        );

        encounter.current_boss_name = update_current_boss_name(&encounter.current_boss_name);
        let version = version.to_string();

        task::spawn(async move {
            let player_infos = if !raid_difficulty.is_empty()
                && !encounter.current_boss_name.is_empty()
                && raid_clear
                && !manual
            {
                info!("fetching player info");
                let players = encounter
                    .entities
                    .values()
                    .filter(|e| is_valid_player(e))
                    .map(|e| e.name.clone())
                    .collect::<Vec<_>>();

                stats_api
                    .get_character_info(&version, &encounter.current_boss_name, players, region.clone())
                    .await
            } else {
                None
            };

            let mut connection = connection_factory.get_connection().unwrap();
            let tx = connection.transaction().expect("failed to create transaction");

            let encounter_id = insert_data(
                &tx,
                encounter,
                prev_stagger,
                damage_log,
                identity_log,
                cast_log,
                boss_hp_log,
                stagger_log,
                stagger_intervals,
                raid_clear,
                party_info,
                raid_difficulty,
                region,
                player_infos,
                &version,
                ntp_fight_start,
                rdps_valid,
                manual,
                skill_cast_log,
            );

            tx.commit().expect("failed to commit transaction");
            info!("saved to db");

            if raid_clear {
                event_emitter
                    .emit(ClearEncounter { encounter_id })
                    .expect("failed to emit clear-encounter");
            }
        });
    }
}
