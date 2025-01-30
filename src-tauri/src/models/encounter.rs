use hashbrown::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{DamageStats, EntityType, Skill, SkillStats, StatusEffect};

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub last_combat_packet: i64,
    pub fight_start: i64,
    pub local_player: String,
    pub entities: HashMap<String, EncounterEntity>,
    pub current_boss_name: String,
    pub current_boss: Option<EncounterEntity>,
    pub encounter_damage_stats: EncounterDamageStats,
    pub duration: i64,
    pub difficulty: Option<String>,
    pub favorite: bool,
    pub cleared: bool,
    pub boss_only_damage: bool,
    pub sync: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncounterEntity {
    pub id: u64,
    pub character_id: u64,
    pub npc_id: u32,
    pub name: String,
    pub entity_type: EntityType,
    pub class_id: u32,
    pub class: String,
    pub gear_score: f32,
    pub current_hp: i64,
    pub max_hp: i64,
    pub current_shield: u64,
    pub is_dead: bool,
    pub skills: HashMap<u32, Skill>,
    pub damage_stats: DamageStats,
    pub skill_stats: SkillStats,
    pub engraving_data: Option<Vec<String>>,
    pub gear_hash: Option<String>,
    pub ark_passive_active: Option<bool>,
    pub ark_passive_data: Option<ArkPassiveData>,
    pub spec: Option<String>,
}

impl EncounterEntity {
    pub fn is_player_with_stats(&self) -> bool {
        self.entity_type == EntityType::Player && self.damage_stats.damage_dealt > 0
    }

    pub fn is_combat_participant(&self) -> bool {
        let is_valid_type = matches!(
            self.entity_type,
            EntityType::Player | EntityType::Esther | EntityType::Boss
        );

        let is_player_with_valid_class = self.entity_type == EntityType::Player && self.class_id > 0;

        (is_player_with_valid_class || is_valid_type) && self.damage_stats.damage_dealt > 0
    }
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EncounterDamageStats {
    pub total_damage_dealt: i64,
    pub top_damage_dealt: i64,
    pub total_damage_taken: i64,
    pub top_damage_taken: i64,
    pub dps: i64,
    pub most_damage_taken_entity: MostDamageTakenEntity,
    pub buffs: HashMap<u32, StatusEffect>,
    pub debuffs: HashMap<u32, StatusEffect>,
    pub total_shielding: u64,
    pub total_effective_shielding: u64,
    pub applied_shield_buffs: HashMap<u32, StatusEffect>,
    #[serde(skip)]
    pub unknown_buffs: HashSet<u32>,
    #[serde(skip)]
    pub max_stagger: i32,
    #[serde(skip)]
    pub stagger_start: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<EncounterMisc>,
    pub boss_hp_log: HashMap<String, Vec<BossHpLog>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stagger_stats: Option<StaggerStats>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BossHpLog {
    pub time: i32,
    pub hp: i64,
    #[serde(default)]
    pub p: f32,
}


impl BossHpLog {
    pub fn new(time: i32, hp: i64, p: f32) -> Self {
        Self { time, hp, p }
    }
}


#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaggerStats {
    pub average: f64,
    #[serde(default)]
    pub staggers_per_min: f64,
    pub log: Vec<(i32, f32)>,
}


#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MostDamageTakenEntity {
    pub name: String,
    pub damage_taken: i64,
}


#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
#[serde_as]
pub struct EncounterMisc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stagger_stats: Option<StaggerStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boss_hp_log: Option<HashMap<String, Vec<BossHpLog>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_clear: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_info: Option<HashMap<i32, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdps_valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdps_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_fight_start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_save: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncounterPreview {
    pub id: i32,
    pub fight_start: i64,
    pub boss_name: String,
    pub duration: i64,
    pub classes: Vec<i32>,
    pub names: Vec<String>,
    pub difficulty: Option<String>,
    pub local_player: String,
    pub my_dps: i64,
    pub favorite: bool,
    pub cleared: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ArkPassiveData {
    pub evolution: Option<Vec<ArkPassiveNode>>,
    pub enlightenment: Option<Vec<ArkPassiveNode>>,
    pub leap: Option<Vec<ArkPassiveNode>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ArkPassiveNode {
    pub id: u32,
    pub lv: u8,
}