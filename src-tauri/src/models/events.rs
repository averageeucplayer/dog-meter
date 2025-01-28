use hashbrown::HashMap;
use serde::Serialize;

use crate::abstractions::AppEvent;

use super::Encounter;

// TO-DO Use macro

#[derive(Debug, Serialize, Clone)]
pub struct IdentityUpdate {
    pub gauge1: u32,
    pub gauge2: u32,
    pub gauge3: u32,
}

impl AppEvent<IdentityUpdate> for IdentityUpdate {
    fn name(&self) -> &'static str {
        "identity-update"
    }
    
    fn payload(&self) -> IdentityUpdate {
       self.clone()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PhaseTransition {
    pub phase_code: i32
}

impl AppEvent<i32> for PhaseTransition {
    fn name(&self) -> &'static str {
        "phase-transition"
    }

    fn payload(&self) -> i32 {
        self.phase_code
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct RaidStart {
    pub timestamp: i64
}

impl AppEvent<i64> for RaidStart {
    fn name(&self) -> &'static str {
        "raid-start"
    }

    fn payload(&self) -> i64 {
        self.timestamp
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ClearEncounter {
    pub encounter_id: i64
}

impl AppEvent<i64> for ClearEncounter {
    fn name(&self) -> &'static str {
        "clear-encounter"
    }

    fn payload(&self) -> i64 {
        self.encounter_id
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ZoneChange {}

impl AppEvent<String> for ZoneChange {
    fn name(&self) -> &'static str {
        "zone-change"
    }
    
    fn payload(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct EncounterUpdate {
    pub encounter: Encounter
}

impl AppEvent<String> for EncounterUpdate {
    fn name(&self) -> &'static str {
        "encounter-update"
    }
    
    fn payload(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct InvalidDamage {}

impl AppEvent<String> for InvalidDamage {
    fn name(&self) -> &'static str {
        "invalid-damage"
    }
    
    fn payload(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PartyUpdate {
    pub party_info: HashMap<i32, Vec<String>>
}

impl AppEvent<String> for PartyUpdate {
    fn name(&self) -> &'static str {
        "party-update"
    }
    
    fn payload(&self) -> String {
        "".to_string()
    }
}