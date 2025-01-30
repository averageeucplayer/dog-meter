use crate::parser::encounter_state::EncounterState;
use crate::models::{ArkPassiveData, Entity, EntityType};
use hashbrown::HashMap;
use log::{info, warn};
use md5::compute;
use moka::sync::Cache;
use reqwest::Client;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use std::fmt;
use std::time::Duration;

pub const API_URL: &str = "https://inspect.fau.dev";
pub const INSPECT_API_URL: &str = "https://api.snow.xyz";

#[derive(Clone)]
pub struct StatsApi {
    pub client_id: String,
    client: Client,
    pub valid_zone: bool,
    stats_cache: Cache<String, PlayerStats>,
    request_cache: Cache<String, PlayerStats>,
    inflight_cache: Cache<String, u8>,
    cancel_queue: Cache<String, String>,

    region_file_path: String,

    pub region: String,
}

impl StatsApi {
    pub fn new(region_file_path: String) -> Self {
        Self {
            client_id: String::new(),
            client: Client::new(),
            valid_zone: false,
            stats_cache: Cache::builder().max_capacity(64).build(),
            request_cache: Cache::builder().max_capacity(64).build(),
            inflight_cache: Cache::builder().max_capacity(32).build(),
            cancel_queue: Cache::builder()
                .max_capacity(16)
                .time_to_live(Duration::from_secs(15))
                .build(),
            region_file_path,

            region: "".to_string(),
        }
    }

    pub fn get_hash(&self, player: &Entity) -> Option<String> {
        if player.gear_level < 0.0
            || player.character_id == 0
            || player.class_id == 0
            || player.name == "You"
            || !player
                .name
                .chars()
                .next()
                .unwrap_or_default()
                .is_uppercase()
        {
            return None;
        }

        let mut equip_data: [u32; 32] = [0; 32];
        if let Some(equip_list) = player.items.equip_list.as_ref() {
            for item in equip_list.iter() {
                if item.slot >= 32 {
                    continue;
                }
                equip_data[item.slot as usize] = item.id;
            }
        }

        if equip_data[..26].iter().all(|&x| x == 0) {
            warn!("missing equipment data for {:?}", player);
            return Some("".to_string());
        }

        // {player_name}{xxxx.xx}{xxx}{character_id}{equip_data}
        let data = format!(
            "{}{:.02}{}{}{}",
            player.name,
            player.gear_level,
            player.class_id,
            player.character_id,
            equip_data.iter().map(|x| x.to_string()).collect::<String>()
        );

        Some(format!("{:x}", compute(data)))
    }

    pub fn get_stats(&mut self, state: &EncounterState) -> Option<Cache<String, PlayerStats>> {
        if !self.valid_difficulty(&state.raid_difficulty) {
            return None;
        }

        Some(self.stats_cache.clone())
    }

    fn valid_difficulty(&self, difficulty: &str) -> bool {
        self.valid_zone
            && (difficulty == "Normal"
                || difficulty == "Hard"
                || difficulty == "The First"
                || difficulty == "Trial")
    }

    pub fn send_raid_info(&mut self, version: &str, state: &EncounterState) {
        if !((self.valid_zone
            && (state.raid_difficulty == "Normal" || state.raid_difficulty == "Hard"))
            || (state.raid_difficulty == "Inferno"
                || state.raid_difficulty == "Trial"
                || state.raid_difficulty == "The First"))
        {
            info!("not valid for raid info");
            return;
        }

        let players: HashMap<String, u64> = state
            .encounter
            .entities
            .iter()
            .filter_map(|(_, e)| {
                if e.entity_type == EntityType::Player {
                    Some((e.name.clone(), e.character_id))
                } else {
                    None
                }
            })
            .collect();

        if players.len() > 16 {
            warn!("invalid zone. num players: {}", players.len());
            return;
        }

        let client = self.client.clone();
        let client_id = self.client_id.clone();
        let region = self.region.clone();
        let boss_name = state.encounter.current_boss_name.clone();
        let difficulty = state.raid_difficulty.clone();
        let cleared = state.raid_clear;
        let version = version.to_string();

        tokio::task::spawn(async move {
            let request_body = json!({
                "id": client_id,
                "version": version,
                "region": region,
                "boss": boss_name,
                "difficulty": difficulty,
                "characters": players,
                "cleared": cleared,
            });

            match client
                .post(format!("{API_URL}/raid"))
                .json(&request_body)
                .send()
                .await
            {
                Ok(_) => {
                    info!("sent raid info");
                }
                Err(e) => {
                    warn!("failed to send raid info: {:?}", e);
                }
            }
        });
    }

    pub async fn get_character_info(&self,
        version: &str,
        boss_name: &str,
        players: Vec<String>,
        region: Option<String>) -> Option<HashMap<String, PlayerStats>> {
        if region.is_none() {
            warn!("region is not set");
            return None;
        }
        
        let request_body = json!({
                "clientId": self.client_id,
                "version": version.to_string(),
                "region": region.unwrap(),
                "boss": boss_name,
                "characters": players,
            });
        
        match self.client
            .post(format!("{INSPECT_API_URL}/inspect"))
            .json(&request_body)
            .send()
            .await
        {
            Ok(res) => {
                match res.json::<HashMap<String, PlayerStats>>().await {
                    Ok(data) => {
                        info!("received player stats");
                        Some(data)
                    }
                    Err(e) => {
                        warn!("failed to parse player stats: {:?}", e);
                        None
                    }
                }
            }
            Err(e) => {
                warn!("failed to get inspect data: {:?}", e);
                None
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub crit: u32,
    pub spec: u32,
    pub swift: u32,
    pub exp: u32,
    pub atk_power: u32,
    pub add_dmg: u32,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PlayerStats {
    pub ark_passive_enabled: bool,
    pub ark_passive_data: Option<ArkPassiveData>,
    pub engravings: Option<Vec<u32>>,
    pub gems: Option<Vec<GemData>>,

}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ElixirData {
    pub slot: u8,
    pub entries: Vec<ElixirEntry>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ElixirEntry {
    pub id: u32,
    pub level: u8,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GemData {
    pub tier: u8,
    pub skill_id: u32,
    pub gem_type: u8,
    pub value: u32,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Engraving {
    pub id: u32,
    pub level: u8,
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PlayerHash {
    pub name: String,
    pub hash: String,
    pub id: u64,
}

struct StatsVisitor;

impl<'de> Visitor<'de> for StatsVisitor {
    type Value = Stats;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map with integer keys")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut stats = Stats::default();
        while let Some((key, value)) = map.next_entry::<usize, u32>()? {
            if key == 0 {
                stats.crit = value;
            } else if key == 1 {
                stats.spec = value;
            } else if key == 2 {
                stats.swift = value;
            } else if key == 3 {
                stats.exp = value;
            } else if key == 4 {
                stats.atk_power = value;
            } else if key == 5 {
                stats.add_dmg = value;
            }
        }
        Ok(stats)
    }
}

impl<'de> Deserialize<'de> for Stats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(StatsVisitor)
    }
}
