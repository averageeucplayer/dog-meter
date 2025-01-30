use std::{fmt::Display, str::FromStr};

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum EntityType {
    #[default]
    Unknown,
    Monster,
    Boss,
    Guardian,
    Player,
    Npc,
    Esther,
    Projectile,
    Summon,
}

impl Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EntityType::Unknown => "UNKNOWN".to_string(),
            EntityType::Monster => "MONSTER".to_string(),
            EntityType::Boss => "BOSS".to_string(),
            EntityType::Guardian => "GUARDIAN".to_string(),
            EntityType::Player => "PLAYER".to_string(),
            EntityType::Npc => "NPC".to_string(),
            EntityType::Esther => "ESTHER".to_string(),
            EntityType::Projectile => "PROJECTILE".to_string(),
            EntityType::Summon => "SUMMON".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for EntityType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNKNOWN" => Ok(EntityType::Unknown),
            "MONSTER" => Ok(EntityType::Monster),
            "BOSS" => Ok(EntityType::Boss),
            "GUARDIAN" => Ok(EntityType::Guardian),
            "PLAYER" => Ok(EntityType::Player),
            "NPC" => Ok(EntityType::Npc),
            "ESTHER" => Ok(EntityType::Esther),
            _ => Ok(EntityType::Unknown),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Entity {
    pub id: u64,
    pub entity_type: EntityType,
    pub name: String,
    pub is_local: bool,
    pub npc_id: u32,
    pub class_id: u32,
    pub gear_level: f32,
    pub character_id: u64,
    pub owner_id: u64,
    pub skill_effect_id: u32,
    pub skill_id: u32,
    pub stats: HashMap<u8, i64>,
    pub stance: u8,
    pub grade: String,
    pub push_immune: bool,
    pub level: u16,
    pub balance_level: u16,
    pub max_hp: i64,
    pub item_set: Option<Vec<PassiveOption>>,
    pub items: Items,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.entity_type {
            EntityType::Unknown => format!("Unknown: id {}", self.id),
            EntityType::Monster => format!("Monster: id {} name {} npc_id {}", self.id, self.name, self.npc_id),
            EntityType::Boss => format!("Boss: id {} name {} npc_id {}", self.id, self.name, self.npc_id),
            EntityType::Guardian => format!("Guardian: id {} name {}", self.id, self.name),
            EntityType::Player => format!("{}Player: id {} name {} {} {} {}", if self.is_local { "Local " } else { "" }, self.id, self.name, self.class_id, self.character_id, self.gear_level),
            EntityType::Npc => format!("Npc: id {} name {} npc_id {}", self.id, self.name, self.npc_id),
            EntityType::Esther => format!("Esther: id {} name {}", self.id, self.name),
            EntityType::Projectile => format!("Projecile: id {} skill_id {} owner_id {}", self.id, self.skill_id, self.owner_id),
            EntityType::Summon => format!("Summon: id {} name {} npc_id {}", self.id, self.name, self.npc_id),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PassiveOption {
    #[serde(rename(deserialize = "type"))]
    pub option_type: String,
    pub key_stat: String,
    pub key_index: i32,
    pub value: i32,
}

#[derive(Debug, Default, Clone)]
pub struct Items {
    pub life_tool_list: Option<Vec<PlayerItemData>>,
    pub equip_list: Option<Vec<PlayerItemData>>,
}

#[derive(Debug, Default, Clone)]
pub struct PlayerItemData {
    pub id: u32,
    pub slot: u16,
}
