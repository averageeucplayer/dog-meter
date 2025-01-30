pub mod events;
pub mod local;
pub mod skill;
pub mod entity;
pub mod encounter;
pub mod stats;
pub mod settings;
pub mod status_effect;
pub mod rdps;
pub mod misc;
pub mod json;
mod utils;

pub use local::*;
pub use skill::*;
pub use entity::*;
pub use encounter::*;
pub use stats::*;
pub use settings::*;
pub use status_effect::*;
pub use rdps::*;
pub use misc::*;
pub use json::*;

use crate::models::utils::int_or_string_as_string;
use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug)]
pub struct DamageData {
    pub skill_id: Option<u32>,
    pub skill_effect_id: Option<u32>,
    pub damage: i64,
    pub modifier: i32,
    pub target_current_hp: i64,
    pub target_max_hp: i64,
    pub damage_attribute: Option<u8>,
    pub damage_type: u8,
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub gauge1: u32,
    pub gauge2: u32,
    pub gauge3: u32,
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Stagger {
    pub current: u32,
    pub max: u32,
}

pub type IdentityLog = Vec<(i64, (u32, u32, u32))>;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentityArcanist {
    // timestamp, (percentage, card, card)
    pub log: Vec<(i32, (f32, u32, u32))>,
    pub average: f64,
    pub card_draws: HashMap<u32, u32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentityArtistBard {
    // timestamp, (percentage, bubble)
    pub log: Vec<(i32, (f32, u32))>,
    pub average: f64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentityGeneric {
    // timestamp, percentage
    pub log: Vec<(i32, f32)>,
    pub average: f64,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Npc {
    pub id: u32,
    pub name: Option<String>,
    pub grade: String,
    #[serde(rename = "type")]
    pub npc_type: String,
    #[serde(rename = "hpBars")]
    pub hp_bars: u16,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Esther {
    pub name: String,
    pub icon: String,
    pub skills: Vec<i32>,
    #[serde(alias = "npcs")]
    pub npc_ids: Vec<u32>,
}



#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillEffectData {
    pub id: i32,
    pub comment: String,
    #[serde(skip)]
    pub stagger: i32,
    pub source_skills: Option<Vec<u32>>,
    pub directional_mask: Option<i32>,
    pub item_name: Option<String>,
    pub item_desc: Option<String>,
    pub item_type: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillBuffData {
    pub id: i32,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub icon: Option<String>,
    pub icon_show_type: Option<String>,
    pub duration: i32,
    // buff | debuff
    pub category: String,
    #[serde(rename(deserialize = "type"))]
    #[serde(deserialize_with = "int_or_string_as_string")]
    pub buff_type: String,
    pub status_effect_values: Option<Vec<i32>>,
    pub buff_category: Option<String>,
    pub target: String,
    pub unique_group: u32,
    #[serde(rename(deserialize = "overlap"))]
    pub overlap_flag: i32,
    pub passive_options: Vec<PassiveOption>,
    pub source_skills: Option<Vec<u32>>,
    pub set_name: Option<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct CombatEffectData {
    pub effects: Vec<CombatEffectDetail>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct CombatEffectDetail {
    pub ratio: i32,
    pub cooldown: i32,
    pub conditions: Vec<CombatEffectCondition>,
    pub actions: Vec<CombatEffectAction>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct CombatEffectCondition {
    #[serde(rename(deserialize = "type"))]
    pub condition_type: String,
    pub actor_type: String,
    pub arg: i32,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct CombatEffectAction {
    pub action_type: String,
    pub actor_type: String,
    pub args: Vec<i32>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct SkillFeatureLevelData {
    pub tripods: HashMap<u8, Tripod>,
}


#[derive(Debug, Default, Deserialize, Clone)]
pub struct ItemSet {
    #[serde(rename(deserialize = "itemids"))]
    pub item_ids: Vec<u32>,
    pub value: HashMap<u8, ItemSetDetails>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ItemSetDetails {
    pub desc: String,
    pub options: Vec<PassiveOption>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ItemSetShort {
    pub set_name: String,
    pub level: u8,
}

pub type ItemSetLevel = HashMap<u8, ItemSetCount>;
pub type ItemSetCount = HashMap<u8, ItemSetDetails>;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct EngravingData {
    pub id: u32,
    pub name: Option<String>,
    pub icon: Option<String>,
}



#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncountersOverview {
    pub encounters: Vec<EncounterPreview>,
    pub total_encounters: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SearchFilter {
    pub bosses: Vec<String>,
    pub min_duration: i32,
    pub max_duration: i32,
    pub cleared: bool,
    pub favorite: bool,
    pub difficulty: String,
    pub boss_only_damage: bool,
    pub sort: String,
    pub order: u8,
}


pub struct CombatEffectConditionData<'a> {
    pub self_entity: &'a Entity,
    pub target_entity: &'a Entity,
    pub caster_entity: &'a Entity,
    pub skill: Option<&'a SkillData>,
    pub hit_option: i32,
    pub target_count: i32,
}

pub struct ItemSetInfo {
    pub item_ids: HashMap<u32, ItemSetShort>,
    pub set_names: HashMap<String, ItemSetLevel>,
}
