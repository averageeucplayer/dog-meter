pub mod encounter_state;
mod entity_tracker;
mod id_tracker;
mod party_tracker;
mod rdps;
mod skill_tracker;
mod stats_api;
mod status_tracker;
mod utils;
mod packet_handler;

use crate::abstractions::{ConnectionFactory, EventEmitter};
use crate::flags::Flags;
use crate::packet_sniffer::PacketSniffer;
use crate::parser::encounter_state::EncounterState;
use crate::parser::entity_tracker::EntityTracker;
use crate::parser::id_tracker::IdTracker;
use crate::models::{LocalInfo, Settings};
use crate::parser::party_tracker::PartyTracker;
use crate::parser::stats_api::StatsApi;
use crate::parser::status_tracker::StatusTracker;
use anyhow::Result;
use hashbrown::HashMap;
use log::{info, warn};
use packet_handler::handle_packet;
use utils::*;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct ParserOptions {

}

pub fn start<C: ConnectionFactory, T: PacketSniffer, E: EventEmitter>(
    options: ParserOptions,
    connection_factory: Arc<C>,
    packet_sniffer: T,
    event_emitter: Arc<E>,
    port: u16,
    settings: Settings,
    region_file_path: &str,
    local_player_path: &str,
    flags: &mut Flags,
    version: String
) -> Result<()> {
    let id_tracker: Rc<RefCell<IdTracker>> = Rc::new(RefCell::new(IdTracker::new()));
    let party_tracker: Rc<RefCell<PartyTracker>> = Rc::new(RefCell::new(PartyTracker::new(id_tracker.clone())));
    let status_tracker: Rc<RefCell<StatusTracker>> = Rc::new(RefCell::new(StatusTracker::new(party_tracker.clone())));
    let mut entity_tracker = EntityTracker::new(
        status_tracker.clone(),
        id_tracker.clone(),
        party_tracker.clone(),
    );
    let mut state = EncounterState::new();
    let mut party_freeze = false;
    let mut party_cache: Option<Vec<Vec<String>>> = None;
    let mut party_map_cache: HashMap<i32, Vec<String>> = HashMap::new();

    let mut stats_api: StatsApi = StatsApi::new(region_file_path.to_string());
    let rx = match packet_sniffer.start_capture(port, region_file_path.to_string()) {
        Ok(rx) => rx,
        Err(e) => {
            warn!("Error starting capture: {}", e);
            return Ok(());
        }
    };

    let damage_handler = meter_core::decryption::DamageEncryptionHandler::new();
    let damage_handler = damage_handler.start()?;

    let mut last_update = Instant::now();
    let mut duration = Duration::from_millis(500);
    let mut last_party_update = Instant::now();
    let party_duration = Duration::from_millis(2000);
    let mut raid_end_cd = Instant::now();

    if settings.general.boss_only_damage {
        flags.set_boss_only_damage();
        info!("boss only damage enabled")
    }
    if settings.general.low_performance_mode {
        duration = Duration::from_millis(1500);
        info!("low performance mode enabled")
    }

    let mut local_info: LocalInfo = get_local_info(local_player_path, &mut stats_api)?;

    get_and_set_region(region_file_path, &mut state);

    while let Ok((op_code, data)) = rx.recv() {

        if flags.is_reset_invoked() {
            state.soft_reset(true);
            flags.clear_reset();
        }

        if flags.is_pause_invoked() {
            continue;
        }

        if flags.is_save_invoked() {
            flags.clear_save();
            state.party_info = update_party(&party_tracker, &entity_tracker);
            state.save_to_db(&stats_api, true, event_emitter.clone(), connection_factory.clone(), &version);
            state.saved = true;
            state.resetting = true;
        }

        if flags.is_boss_only_damage_invoked() {
            state.boss_only_damage = true;
        } else {
            state.boss_only_damage = false;
            state.encounter.boss_only_damage = false;
        }

        let mut status_tracker: std::cell::RefMut<'_, StatusTracker> = status_tracker.borrow_mut();

        handle_packet(
            op_code,
            data,
            &mut state,
            party_tracker.clone(),
            &mut entity_tracker,
            &mut status_tracker,
            id_tracker.clone(),
            &mut party_cache,
            &mut party_map_cache,
            &mut local_info,
            &mut stats_api,
            region_file_path.as_ref(),
            &mut party_freeze,
            local_player_path,
            &mut raid_end_cd,
            &flags,
            event_emitter.clone(),
            connection_factory.clone(),
            &damage_handler,
            &version
        )?;

        let can_send_to_ui = last_update.elapsed() >= duration || state.resetting || state.boss_dead_update;

        if can_send_to_ui {
            let is_boss_dead = state.boss_dead_update;
            let is_damage_valid = state.damage_is_valid;

            if state.boss_dead_update {
                state.boss_dead_update = false;
            }

            let mut encounter = state.encounter.clone();

            let party_info = get_party_info(
                &mut last_party_update,
                party_duration,
                &mut party_cache,
                &mut party_map_cache,
                party_freeze,
                &party_tracker,
                &entity_tracker
            );

            let entities = &mut encounter.entities;
            let current_boss_name = &mut encounter.current_boss_name;

            if !current_boss_name.is_empty() {
                let current_boss = entities.get(current_boss_name).cloned();
                if let Some(mut current_boss) = current_boss {
                    if is_boss_dead {
                        current_boss.is_dead = true;
                        current_boss.current_hp = 0;
                    }
                    encounter.current_boss = Some(current_boss);
                } else {
                    *current_boss_name = String::new();
                }
            }

            entities.retain(|_, entity| entity.is_combat_participant());

            if !entities.is_empty() {
                send_to_ui(
                    event_emitter.clone(),
                    encounter,
                    is_damage_valid,
                    party_info
                );
            }

            last_update = Instant::now();
        }

        if state.resetting {
            state.soft_reset(true);
            state.resetting = false;
            state.saved = false;
            party_freeze = false;
            party_cache = None;
            party_map_cache = HashMap::new();
        }
    }

    Ok(())
}