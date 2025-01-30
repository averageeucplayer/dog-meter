use std::{cell::RefCell, rc::Rc};

use chrono::Utc;
use meter_core::{decryption::DamageEncryptionHandlerInner, packets::structures::SkillDamageEvent};

use crate::{abstractions::EventEmitter, models::DamageData, parser::{encounter_state::EncounterState, entity_tracker::EntityTracker, id_tracker::IdTracker, party_tracker::PartyTracker, status_tracker::StatusTracker}};

pub fn on_damage(
    source_id: u64,
    skill_id: Option<u32>,
    skill_effect_id: Option<u32>,
    events: Vec<SkillDamageEvent>,
    state: &mut EncounterState,
    entity_tracker: &mut EntityTracker,
    status_tracker: Rc<RefCell<StatusTracker>>,
    id_tracker: Rc<RefCell<IdTracker>>,
    damage_handler: &DamageEncryptionHandlerInner,
    event_emitter: &impl EventEmitter,
) {
    let now = Utc::now().timestamp_millis();
    let owner = entity_tracker.get_source_entity(source_id);
    let local_character_id = id_tracker
        .borrow()
        .get_local_character_id(entity_tracker.local_entity_id);

    for mut event in events.into_iter() {
        if !damage_handler.decrypt_damage_event(&mut event) {
            state.damage_is_valid = false;
            continue;
        }

        let target_entity =
            entity_tracker.get_or_create_entity(event.target_id);
        let source_entity = entity_tracker.get_or_create_entity(source_id);
        let (se_on_source, se_on_target) = status_tracker
            .borrow_mut()
            .get_status_effects(&owner, &target_entity, local_character_id);
        let damage_data = DamageData {
            skill_id: skill_id,
            skill_effect_id: skill_effect_id,
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
            now,
            event_emitter,
        );
    }
}

// pub fn on_damage_2() {
//     let now = Utc::now().timestamp_millis();
//     let owner = entity_tracker.get_source_entity(pkt.source_id);
//     let local_character_id = id_tracker
//         .borrow()
//         .get_local_character_id(entity_tracker.local_entity_id);

//     for mut event in pkt.skill_damage_events.into_iter() {
//         if !damage_handler.decrypt_damage_event(&mut event) {
//             state.damage_is_valid = false;
//             continue;
//         }
//         let target_entity = entity_tracker.get_or_create_entity(event.target_id);
//         // source_entity is to determine battle item
//         let source_entity = entity_tracker.get_or_create_entity(pkt.source_id);
//         let (se_on_source, se_on_target) = status_tracker
//             .borrow_mut()
//             .get_status_effects(&owner, &target_entity, local_character_id);
//         let damage_data = DamageData {
//             skill_id: pkt.skill_id,
//             skill_effect_id: pkt.skill_effect_id.unwrap_or_default(),
//             damage: event.damage,
//             modifier: event.modifier as i32,
//             target_current_hp: event.cur_hp,
//             target_max_hp: event.max_hp,
//             damage_attribute: event.damage_attr,
//             damage_type: event.damage_type,
//         };
//         state.on_damage(
//             &owner,
//             &source_entity,
//             &target_entity,
//             damage_data,
//             se_on_source,
//             se_on_target,
//             now,
//             event_emitter.as_ref()
//         );
//     }
// }