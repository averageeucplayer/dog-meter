use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum HitOption {
    None = 0,
    BackAttack = 1,
    FrontalAttack = 2,
    FlankAttack = 3,
    Max = 4,
}

impl HitOption {
    pub fn from(value: i32) -> Self {
        // unsafe { std::mem::transmute(value) }

        match value {
            0 => HitOption::None,
            1 => HitOption::BackAttack,
            2 => HitOption::FrontalAttack,
            3 => HitOption::FlankAttack,
            4 => HitOption::Max,
            _ => HitOption::None
        }
    }
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum HitFlag {
    Normal = 0,
    Critical = 1,
    Miss = 2,
    Invincible = 3,
    Dot = 4,
    Immune = 5,
    ImmuneSilenced = 6,
    FontSilenced = 7,
    DotCritical = 8,
    Dodge = 9,
    Reflect = 10,
    DamageShare = 11,
    DodgeHit = 12,
    Max = 13,
    Unknown = 999
}

impl HitFlag {
    pub fn from(value: i32) -> Self {
        // unsafe { std::mem::transmute(value) }
            
        match value {
            0 => HitFlag::Normal,
            1 => HitFlag::Critical,
            2 => HitFlag::Miss,
            3 => HitFlag::Invincible,
            4 => HitFlag::Dot,
            5 => HitFlag::Immune,
            6 => HitFlag::ImmuneSilenced,
            7 => HitFlag::FontSilenced,
            8 => HitFlag::DotCritical,
            9 => HitFlag::Dodge,
            10 => HitFlag::Reflect,
            11 => HitFlag::DamageShare,
            12 => HitFlag::DodgeHit,
            13 => HitFlag::Max,
            _ => HitFlag::Unknown
        }
    }
}

#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterDbInfo {
    pub size: String,
    pub total_encounters: i32,
    pub total_encounters_filtered: i32,
}