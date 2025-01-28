use serde::{Deserialize, Serialize};
use crate::models::utils::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub general: GeneralSettings,
    pub shortcuts: Shortcuts,
    pub meter: MeterTabs,
    pub logs: LogTabs,
    pub buffs: BuffSettings,
    pub sync: SyncSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Shortcuts {
    pub hide_meter: Shortcut,
    pub show_logs: Shortcut,
    pub show_latest_encounter: Shortcut,
    pub reset_session: Shortcut,
    pub pause_session: Shortcut,
    pub manual_save: Shortcut,
    pub disable_clickthrough: Shortcut,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Shortcut {
    pub modifier: String,
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GeneralSettings {
    pub start_loa_on_start: bool,
    pub low_performance_mode: bool,
    #[serde(default = "default_true")]
    pub show_names: bool,
    pub show_gear_score: bool,
    pub hide_names: bool,
    #[serde(default = "default_true")]
    pub show_esther: bool,
    #[serde(default = "default_true")]
    pub show_date: bool,
    #[serde(default = "default_true")]
    pub show_difficulty: bool,
    pub show_gate: bool,
    #[serde(default = "default_true")]
    pub split_lines: bool,
    pub underline_hovered: bool,
    pub show_details: bool,
    pub show_shields: bool,
    pub show_tanked: bool,
    pub show_bosses: bool,
    pub hide_logo: bool,
    pub accent_color: String,
    pub raw_socket: bool,
    #[serde(default = "default_true")]
    pub auto_iface: bool,
    pub if_desc: String,
    pub ip: String,
    pub port: u16,
    pub blur: bool,
    pub blur_win11: bool,
    pub transparent: bool,
    #[serde(default = "default_scale")]
    pub scale: String,
    #[serde(default = "default_scale")]
    pub log_scale: String,
    #[serde(default = "default_true")]
    pub always_on_top: bool,
    #[serde(default = "default_true")]
    pub boss_only_damage: bool,
    #[serde(default = "default_true")]
    pub keep_favorites: bool,
    pub hide_meter_on_start: bool,
    pub hide_logs_on_start: bool,
    pub constant_local_player_color: bool,
    #[serde(default = "default_true")]
    pub boss_only_damage_default_on: bool,
    pub start_on_boot: bool,
    pub logs_per_page: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct BuffSettings {
    #[serde(default = "default_true")]
    pub default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SyncSettings {
    pub enabled: bool,
    pub access_token: String,
    pub auto: bool,
    pub username: String,
    pub valid_token: bool,
    pub visibility: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MeterTabs {
    pub boss_hp: bool,
    pub boss_hp_bar: bool,
    pub split_boss_hp_bar: bool,
    pub abbreviate_header: bool,
    pub show_time_until_kill: bool,
    pub show_class_colors: bool,
    #[serde(default = "default_true")]
    pub split_party_buffs: bool,
    #[serde(default = "default_true")]
    pub pin_self_party: bool,
    pub damage: bool,
    pub dps: bool,
    pub damage_percent: bool,
    pub death_time: bool,
    pub crit_rate: bool,
    pub crit_dmg: bool,
    pub front_atk: bool,
    pub back_atk: bool,
    pub percent_buff_by_sup: bool,
    pub percent_identity_by_sup: bool,
    #[serde(default = "default_true")]
    pub positional_dmg_percent: bool,
    pub percent_brand: bool,
    pub counters: bool,
    #[serde(default = "default_true")]
    pub rdps_split_party: bool,
    pub rdps_damage_given: bool,
    pub rdps_damage_received: bool,
    pub rdps_contribution: bool,
    pub rdps_s_contribution: bool,
    pub rdps_d_contribution: bool,
    #[serde(default = "default_true")]
    pub rdps_syn: bool,
    #[serde(default = "default_true")]
    pub rdps_s_syn: bool,
    #[serde(default = "default_true")]
    pub rdps_d_syn: bool,
    #[serde(default = "default_true")]
    pub ssyn: bool,
    pub breakdown: BreakdownTabs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LogTabs {
    pub abbreviate_header: bool,
    #[serde(default = "default_true")]
    pub split_party_damage: bool,
    #[serde(default = "default_true")]
    pub split_party_buffs: bool,
    pub damage: bool,
    pub dps: bool,
    pub damage_percent: bool,
    pub death_time: bool,
    pub crit_rate: bool,
    pub crit_dmg: bool,
    pub front_atk: bool,
    pub back_atk: bool,
    pub percent_buff_by_sup: bool,
    pub percent_identity_by_sup: bool,
    #[serde(default = "default_true")]
    pub positional_dmg_percent: bool,
    pub percent_brand: bool,
    pub counters: bool,
    pub min_encounter_duration: i32,
    #[serde(default = "default_true")]
    pub rdps_split_party: bool,
    #[serde(default = "default_true")]
    pub rdps_damage_given: bool,
    #[serde(default = "default_true")]
    pub rdps_damage_received: bool,
    #[serde(default = "default_true")]
    pub rdps_contribution: bool,
    #[serde(default = "default_true")]
    pub rdps_s_contribution: bool,
    #[serde(default = "default_true")]
    pub rdps_d_contribution: bool,
    #[serde(default = "default_true")]
    pub rdps_syn: bool,
    #[serde(default = "default_true")]
    pub rdps_s_syn: bool,
    #[serde(default = "default_true")]
    pub rdps_d_syn: bool,
    #[serde(default = "default_true")]
    pub ssyn: bool,
    pub breakdown: BreakdownTabs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct BreakdownTabs {
    pub damage: bool,
    pub dps: bool,
    pub damage_percent: bool,
    pub crit_rate: bool,
    pub crit_dmg: bool,
    pub front_atk: bool,
    pub back_atk: bool,
    pub percent_buff_by_sup: bool,
    pub percent_identity_by_sup: bool,
    pub percent_brand: bool,
    pub avg_damage: bool,
    pub max_damage: bool,
    pub casts: bool,
    pub cpm: bool,
    pub hits: bool,
    pub hpm: bool,
}
