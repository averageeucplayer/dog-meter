use meter_core::packets::{definitions::*, opcodes::Pkt, structures::*};

pub struct FightSimulatorOptions {
    pub min_dps: i64,
    pub max_dps: i64,
    pub min_sup_dps: i64,
    pub max_sup_dps: i64,
}

pub struct FightSimulator {
    options: FightSimulatorOptions,
    boss: Option<String>
}

impl FightSimulator {
    pub fn new(options: FightSimulatorOptions) -> Self {
        Self {
            options,
            boss: None
        }
    }

    pub fn set_player(&mut self, name: &str, hp: i64) {

    }

    pub fn set_boss(&mut self, name: &str, hp: i64) {

    }
    
    pub fn get_player_packets(&self) -> Vec<(Pkt, Vec<u8>)> {
        vec![]
    }

    pub fn get_local_player_packet(&self) -> (Pkt, Vec<u8>) {
        let packet = PKTInitPC {
            player_id: 0,
            character_id: 0,
            class_id: 0,
            gear_level: 0.0,
            name: "".into(),
            stat_pairs: vec![],
            status_effect_datas: vec![]
        };
        
        let data = serde_json::to_vec(&packet).unwrap();

        (Pkt::InitPC, data)
    }

    pub fn get_party_info_packet(&self) -> (Pkt, Vec<u8>) {
        let packet = PKTPartyInfo {
           party_instance_id: 0,
           raid_instance_id: 0,
           party_member_datas: vec![]
        };
        
        let data = serde_json::to_vec(&packet).unwrap();

        (Pkt::PartyInfo, data)
    }

    pub fn get_boss_packet(&self) -> (Pkt, Vec<u8>) {
        let packet = PKTNewNpc {
            npc_struct: NpcStruct {
                object_id: 0,
                type_id: 0,
                level: 0,
                balance_level: None,
                stat_pairs: vec![],
                status_effect_datas: vec![]
            }
         };
         
         let data = serde_json::to_vec(&packet).unwrap();
        
        (Pkt::NewNpc, data)
    }

    pub fn get_damage_packet(&self) -> (Pkt, Vec<u8>) {

        let packet = PKTSkillDamageNotify {
            skill_damage_events: vec![],
            skill_effect_id: None,
            skill_id: 0,
            source_id: 0
        };
         
         let data = serde_json::to_vec(&packet).unwrap();
        
        (Pkt::SkillDamageNotify, data)
    }
}