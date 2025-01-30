mod class_template;
pub mod utils;

use std::{time::{Duration, Instant}, vec};

use class_template::ClassTemplate;
use hashbrown::HashSet;
use log::info;
use meter_core::packets::{definitions::*, opcodes::Pkt, structures::*};
use utils::*;

use crate::models::{HitFlag, HitOption, Npc, NPC_DATA};

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ZoneLevel {
    Normal = 0,
    Hard = 1,
    Inferno = 2,
    Challenge = 3,
    Solo = 4,
    TheFirst = 5
}

pub struct FightSimulatorOptions<'a> {
    pub min_crit_rate: f32,
    pub max_crit_rate: f32,
    pub sup_crit_rate: f32,
    pub min_id: u64,
    pub max_id: u64,
    pub min_entity_id: u64,
    pub max_entity_id: u64,
    pub min_player_hp: i64,
    pub max_player_hp: i64,
    pub min_dps: i64,
    pub max_dps: i64,
    pub min_awakening_dps: i64,
    pub max_awakening_dps: i64,
    pub min_sup_dps: i64,
    pub max_sup_dps: i64,
    pub min_ilvl: f32,
    pub max_ilvl: f32,
    pub player_names: Vec<&'a str>,
    pub party_count: u8,
    pub local_player_name: &'a str,
    pub boss_name: &'a str,
    pub boss_max_hp: i64,
    pub zone_id: u32,
    pub zone_level: ZoneLevel,
    pub trigger_signal_on_end: u32,
    pub use_hyper_awakening_after: Duration,
    pub perform_counter_every: Option<Duration>,
    pub players_dead_after: Vec<(&'a str, Duration)>
}

pub struct Boss {
    pub id: u64,
    pub name: String,
    pub npc_id: u32,
    pub hp: i64,
    pub max_hp: i64,
    pub stat_pairs: Vec<StatPair>,
    pub status_effect_datas: Vec<StatusEffectData>
}

pub struct Party<'a> {
    pub id: u32,
    pub members: Vec<Player<'a>>
}

#[derive(Debug, Clone)]
pub struct Player<'a> {
    pub id: u64,
    pub is_dead: bool,
    pub name: String,
    pub character_id: u64,
    pub class_id: u32,
    pub gear_level: f32,
    pub stat_pairs: Vec<StatPair>,
    pub status_effect_datas: Vec<StatusEffectData>,
    pub class_template: &'a ClassTemplate<'static>,
    pub min_dps: i64,
    pub max_dps: i64,
    pub crit_rate: f32
}

pub struct FightSimulator<'a> {
    last_counter_on: Option<Instant>,
    fight_started_on: Option<Instant>,
    current_time: Instant,
    options: FightSimulatorOptions<'a>,
    boss: Boss,
    parties: Vec<Party<'a>>,
    players: Vec<Player<'a>>,
    local_player: Player<'a>,
    raid_instance_id: u32,
    dead_player_names: HashSet<String>
}

impl<'a> FightSimulator<'a> {
    pub fn new(options: FightSimulatorOptions<'a>) -> Self {

        let mut dps_class_ids: HashSet<u32> = HashSet::new();
        let mut sup_class_ids: HashSet<u32> = HashSet::new();
        let mut entity_ids: HashSet<u64> = HashSet::new();
        let mut character_ids: HashSet<u64> = HashSet::new();
        let mut party_ids: HashSet<u32> = HashSet::new();
        let mut players = vec![];
        let mut parties = vec![];
        let mut player_names_iter = options.player_names.iter();
        let raid_instance_id = get_random_u32(options.min_id as u32, options.max_id as u32);

        for _ in 1..=options.party_count {

            let mut id = get_random_u32(options.min_id as u32, options.max_id as u32);

            while party_ids.contains(&id) {
                id = get_random_u32(options.min_id as u32, options.max_id as u32);
            }

            party_ids.insert(id);

            let mut party = Party {
                id,
                members: vec![]
            };

            for _ in 1..=3 {
                let name = player_names_iter.next().unwrap();
                let player = Self::get_random_dps(&options, name, &mut entity_ids, &mut character_ids, &mut dps_class_ids);
                players.push(player.clone());
                party.members.push(player);
    
            }

            let name = player_names_iter.next().unwrap();
            let player = Self::get_random_sup(&options, name, &mut entity_ids, &mut character_ids, &mut sup_class_ids);
            players.push(player.clone());
            party.members.push(player);
            
            parties.push(party);
        }

        let boss = Self::get_boss(&options, &mut entity_ids);
        let local_player = players.iter().find(|pr| pr.name == options.local_player_name).unwrap().clone();

        Self {
            current_time: Instant::now(),
            last_counter_on: None,
            fight_started_on: None,
            options,
            boss,
            players,
            parties,
            local_player,
            raid_instance_id,
            dead_player_names: HashSet::new()
        }
    }

    pub fn update_time(&mut self) {
        self.current_time = Instant::now();
    }

    pub fn is_boss_dead(&self) -> bool {
        self.boss.hp == 0
    }
    
    pub fn get_player_packets(&self) -> Vec<(Pkt, Vec<u8>)> {
        let mut player_packets = vec![];

        for player in self.players.iter() {
            let packet = PKTNewPC {
                pc_struct: PKTNewPCInner {
                    player_id: player.id,
                    name: player.name.clone(),
                    character_id: player.character_id,
                    class_id: player.class_id,
                    stat_pairs: player.stat_pairs.clone(),
                    equip_item_datas: vec![],
                    status_effect_datas: player.status_effect_datas.clone(),
                    max_item_level: player.gear_level
                }
            };
            let data = serde_json::to_vec(&packet).unwrap();
            let tuple = (Pkt::NewPC, data);
            player_packets.push(tuple);
        }

        player_packets
    }

    pub fn get_local_player_packet(&self) -> (Pkt, Vec<u8>) {
        let player = &self.local_player;

        let packet = PKTInitPC {
            player_id: player.id,
            character_id: player.character_id,
            class_id: player.class_id,
            gear_level: player.gear_level,
            name: player.name.to_string(),
            stat_pairs: player.stat_pairs.clone(),
            status_effect_datas: player.status_effect_datas.clone()
        };
        
        let data = serde_json::to_vec(&packet).unwrap();

        (Pkt::InitPC, data)
    }

    pub fn get_party_info_packet(&self) -> Vec<(Pkt, Vec<u8>)> {
        let mut parties = vec![];

        for party in self.parties.iter() {
            let mut packet = PKTPartyInfo {
                party_instance_id: party.id,
                raid_instance_id: self.raid_instance_id,
                party_member_datas: vec![]
             };

            for member in party.members.iter() {
                let member_data = PKTPartyInfoInner {
                    name: member.name.clone(),
                    class_id: member.class_id,
                    character_id: member.character_id,
                    gear_level: member.gear_level,
                };
    
                packet.party_member_datas.push(member_data);
            }

            let data = serde_json::to_vec(&packet).unwrap();

            parties.push((Pkt::PartyInfo, data));
        }

        parties
    }

        
    pub(crate) fn get_zone_member_load_packet(&self) -> (Pkt, Vec<u8>) {
        let packet = PKTZoneMemberLoadStatusNotify {
            zone_id: self.options.zone_id,
            zone_level: self.options.zone_level as u32,
        };

        let data = serde_json::to_vec(&packet).unwrap();

        (Pkt::ZoneMemberLoadStatusNotify, data)
    }

    pub(crate) fn get_trigger_start_packet(&self) -> (Pkt, Vec<u8>) {
        let packet = PKTTriggerStartNotify {
            signal: self.options.trigger_signal_on_end
        };

        let data = serde_json::to_vec(&packet).unwrap();

        (Pkt::TriggerStartNotify, data)
    }

    pub fn get_boss_packet(&self) -> (Pkt, Vec<u8>) {
        let packet = PKTNewNpc {
            npc_struct: NpcStruct {
                object_id: self.boss.id,
                type_id: self.boss.npc_id,
                level: 60,
                balance_level: None,
                stat_pairs: self.boss.stat_pairs.clone(),
                status_effect_datas: vec![]
            }
         };
         
        let data = serde_json::to_vec(&packet).unwrap();
        
        (Pkt::NewNpc, data)
    }

    pub fn perform_special_actions(&mut self) -> Vec<(Pkt, Vec<u8>)> {

        let mut packets = vec![];

        for (name, duration) in &self.options.players_dead_after {

            if self.dead_player_names.contains(*name) {
                continue;
            }

            let player = self.players.iter_mut().find(|pr| pr.name == *name).unwrap();

            let should_flag_as_dead = self.fight_started_on.filter(|started_on| self.current_time - *started_on > *duration).is_some();

            if should_flag_as_dead {
                player.is_dead = true;
                self.dead_player_names.insert(name.to_string());

                let packet = PKTDeathNotify {
                    target_id: player.id,
                };
                let data = serde_json::to_vec(&packet).unwrap();
    
                let tuple = (Pkt::DeathNotify, data);
                packets.push(tuple);
            }
        }

        if let (Some(fight_started_on), Some(perform_counter_every)) = (self.fight_started_on, self.options.perform_counter_every) {
            let should_perform_counter = match self.last_counter_on {
                Some(last_counter_on) => fight_started_on - last_counter_on > perform_counter_every,
                None => true,
            };
    
            if should_perform_counter {
                self.last_counter_on.replace(self.current_time);
    
                let player = random_item(&self.players);
                let packet = PKTCounterAttackNotify {
                    source_id: player.id,
                };
                let data = serde_json::to_vec(&packet).unwrap();
    
                let tuple = (Pkt::CounterAttackNotify, data);
                packets.push(tuple);
            }
        }
    
        packets
    }

    pub fn random_player_damage_packet(&mut self) -> (Pkt, Vec<u8>) {

        let mut player = random_item_mut(&mut self.players);

        while player.is_dead {
            player = random_item_mut(&mut self.players);
        }

        let skill_id = random_item(&player.class_template.skill_ids);

        let mut hit_flag = random_hit_flag();

        if should_execute(player.crit_rate) {
            hit_flag = HitFlag::Critical;
        }

        let hit_option = random_hit_option();
        let mut damage = 0;
        let modifier = calculate_modifier(hit_flag, hit_option);
        let cur_hp = &mut self.boss.hp;

        match &hit_flag {
            HitFlag::Normal | HitFlag::Dot => {
                damage = get_random_i64(player.min_dps, player.max_dps);

                if *cur_hp > damage {
                    *cur_hp -= damage;
                }
                else {
                    damage = *cur_hp;
                    *cur_hp = 0;
                }
            },
            HitFlag::Critical | HitFlag::DotCritical => {
                damage = get_random_i64(player.min_dps, player.max_dps);
                damage *= 2;

                if *cur_hp > damage {
                    *cur_hp -= damage;
                }
                else {
                    damage = *cur_hp;
                    *cur_hp = 0;
                }
            },
            _ => {}
        }

        let skill_damage_event = SkillDamageEvent {
            damage,
            damage_attr: None,
            damage_type: 0,
            cur_hp: *cur_hp,
            max_hp: self.boss.max_hp,
            modifier,
            target_id: self.boss.id
        };

        let packet = PKTSkillDamageNotify {
            skill_damage_events: vec![skill_damage_event],
            skill_effect_id: None,
            skill_id,
            source_id: player.id
        };
         
        let data = serde_json::to_vec(&packet).unwrap();
        
        (Pkt::SkillDamageNotify, data)
    }

    pub fn get_npc_with_max_bars_by_name(name: &str) -> &'static Npc {
        let mut candidates = vec![];

        for (_, npc_data) in NPC_DATA.iter() {
            if npc_data.name.as_ref().filter(|npc_name| *npc_name == name).is_some() {
                candidates.push(npc_data);
            }
        }
        
        candidates.sort_unstable_by(|a, b| b.hp_bars.cmp(&a.hp_bars));
        candidates.first().unwrap()
    }

    pub fn get_boss(options: &FightSimulatorOptions, entity_ids: &mut HashSet<u64>) -> Boss {

        let mut id = get_random_u64(options.min_entity_id, options.max_entity_id);

        while entity_ids.contains(&id) {
            id = get_random_u64(options.min_entity_id, options.max_entity_id);
        }

        entity_ids.insert(id);

        let npc = Self::get_npc_with_max_bars_by_name(options.boss_name);

        Boss {
            id,
            name: options.boss_name.to_string(),
            npc_id: npc.id,
            hp: options.boss_max_hp,
            max_hp: options.boss_max_hp,
            stat_pairs: Self::to_hp_stat_pairs(options.boss_max_hp, options.boss_max_hp),
            status_effect_datas: vec![]
        }
    }

    pub fn get_random_sup(
        options: &FightSimulatorOptions, name: &str,
        entity_ids: &mut HashSet<u64>,
        character_ids: &mut HashSet<u64>,
        sup_class_ids: &mut HashSet<u32>) -> Player<'a> {

        let mut id = get_random_u64(options.min_entity_id, options.max_entity_id);

        while entity_ids.contains(&id) {
            id = get_random_u64(options.min_entity_id, options.max_entity_id);
        }

        entity_ids.insert(id);

        let mut character_id = get_random_u64(options.min_entity_id, options.max_entity_id);

        while character_ids.contains(&character_id) {
            character_id = get_random_u64(options.min_entity_id, options.max_entity_id);
        }

        character_ids.insert(id);

        let mut class_template = get_random_sup_class_template();

        while sup_class_ids.contains(&class_template.id) {
            class_template = get_random_sup_class_template();
        }

        sup_class_ids.insert(class_template.id);

        let gear_level = get_random_gear_level(options.min_ilvl, options.max_ilvl);
        let max_hp = get_random_i64(options.min_player_hp, options.max_player_hp);
        let hp = max_hp;

        let player = Player {
            id,
            is_dead: false,
            name: name.to_string(),
            class_id: class_template.id,
            character_id,
            gear_level,
            stat_pairs: Self::to_hp_stat_pairs(hp, max_hp),
            status_effect_datas: vec![],
            class_template,
            min_dps: options.min_sup_dps,
            max_dps: options.max_sup_dps,
            crit_rate: options.sup_crit_rate
        };

        player
    }

    pub fn to_hp_stat_pairs(hp: i64, max_hp: i64) -> Vec<StatPair> {
        vec![
            StatPair {
                stat_type: 1,
                value: hp,
            },
            StatPair {
                stat_type: 27,
                value: max_hp,
            }
        ]
    }

    pub fn get_random_dps(
        options: &FightSimulatorOptions,
        name: &str,
        entity_ids: &mut HashSet<u64>,
        character_ids: &mut HashSet<u64>,
        dps_class_ids: &mut HashSet<u32>) -> Player<'a> {

        let mut id = get_random_u64(options.min_entity_id, options.max_entity_id);

        while entity_ids.contains(&id) {
            id = get_random_u64(options.min_entity_id, options.max_entity_id);
        }

        entity_ids.insert(id);

        let mut character_id = get_random_u64(options.min_entity_id, options.max_entity_id);

        while character_ids.contains(&character_id) {
            character_id = get_random_u64(options.min_entity_id, options.max_entity_id);
        }

        character_ids.insert(id);

        let mut class_template = get_random_dps_class_template();

        while dps_class_ids.contains(&class_template.id) {
            class_template = get_random_dps_class_template();
        }

        dps_class_ids.insert(class_template.id);

        let gear_level = get_random_gear_level(options.min_ilvl, options.max_ilvl);
        let max_hp = get_random_i64(options.min_player_hp, options.max_player_hp);
        let crit_rate = get_random_f32(options.min_crit_rate, options.max_crit_rate);
        let hp = max_hp;

        let player = Player {
            id,
            is_dead: false,
            name: name.to_string(),
            class_id: class_template.id,
            character_id,
            gear_level,
            stat_pairs: Self::to_hp_stat_pairs(hp, max_hp),
            status_effect_datas: vec![],
            class_template,
            min_dps: options.min_dps,
            max_dps: options.max_dps,
            crit_rate
        };

        player
    }

}