use tauri_plugin_window_state::StateFlags;

pub const DB_VERSION: i32 = 5;
pub const WINDOWS_SERVICE_CONTROL: &str = "sc";
pub const DELETE_WINDIVERT_SERVICE_COMMAND: [&str; 2] = ["delete", "windivert"];
pub const STOP_WINDIVERT_COMMAND: [&str; 2] = ["stop", "windivert"];
pub const STEAM_GAME_COMMAND: [&str; 3] = ["/C", "start", "steam://rungameid/1599340"];
pub const LOCAL_PLAYERS_FILE_NAME: &str = "local_players.json";
pub const SETTINGS_FILE_NAME: &str = "settings.json";
pub const DATABASE_FILE_NAME: &str = "encounters.db";
pub const METER_WINDOW_LABEL: &str = "main";
pub const LOGS_WINDOW_LABEL: &str = "logs";
pub const WINDOW_STATE_FLAGS: StateFlags = StateFlags::from_bits_truncate(
    StateFlags::FULLSCREEN.bits()
        | StateFlags::MAXIMIZED.bits()
        | StateFlags::POSITION.bits()
        | StateFlags::SIZE.bits()
        | StateFlags::VISIBLE.bits(),
);
