use std::{error::Error, sync::Arc};

use log::{error, info, warn};
use tauri::{App, AppHandle, Manager};
use tauri_plugin_window_state::WindowExt;
use tokio::task;

use crate::{abstractions::{DefaultEventEmitter, EventEmitter, SqliteConnectionFactory}, constants::{DATABASE_FILE_NAME, LOCAL_PLAYERS_FILE_NAME, LOGS_WINDOW_LABEL, METER_WINDOW_LABEL, WINDOW_STATE_FLAGS}, database::setup_db, flags::Flags, packet_sniffer::{FakePacketSniffer, WindivertPacketSniffer}, parser::{self, ParserOptions}, settings::{create_default_settings, read_settings}, utils::{remove_driver, start_loa_process}};


pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let version = app.package_info().version.to_string();
    info!("starting app v{}", version);

    let resource_path = app
        .path_resolver()
        .resource_dir()
        .expect("could not get resource dir");

    match setup_db(&resource_path) {
        Ok(_) => (),
        Err(e) => {
            warn!("error setting up database: {}", e);
        }
    }

    launch_update(app.handle());

    let settings = read_settings(&resource_path).ok()
        .unwrap_or_else(|| create_default_settings(&resource_path).unwrap());

    let meter_window = app.get_window(METER_WINDOW_LABEL).unwrap();
    meter_window
        .restore_state(WINDOW_STATE_FLAGS)
        .expect("failed to restore window state");
    // #[cfg(debug_assertions)]
    // {
    //     meter_window.open_devtools();
    // }

    let logs_window = app.get_window(LOGS_WINDOW_LABEL).unwrap();
    logs_window
        .restore_state(WINDOW_STATE_FLAGS)
        .expect("failed to restore window state");

    info!("settings loaded");
    if !settings.general.hide_meter_on_start {
        meter_window.show()?;
    }
    
    if !settings.general.hide_logs_on_start {
        logs_window.show()?;
    }

    if !settings.general.always_on_top {
        meter_window.set_always_on_top(false)?;
    }

    let port = settings.general.port;

    if settings.general.start_loa_on_start {
        info!("auto launch game enabled");
        start_loa_process();
    }

    info!("listening on port: {}", port);
    
    let region_file_path = {
        let mut resource_path = resource_path.clone();
        resource_path.push("current_region");
        resource_path.to_string_lossy().to_string()
    };

    let local_player_path = {
        let mut path = resource_path.clone();
        path.push(LOCAL_PLAYERS_FILE_NAME);
        path.to_string_lossy().to_string()
    };

    let mut flags = Flags::new(meter_window.clone());

    let database_path =  {
        let mut path = resource_path.clone();
        path.push(DATABASE_FILE_NAME);
        path.to_string_lossy().to_string()
    };

    remove_driver();

    task::spawn_blocking(move || {
        // let packet_sniffer = WindivertPacketSniffer::new();
        let options = ParserOptions {

        };
        let connection_factory = Arc::new(SqliteConnectionFactory::new(&database_path));
        let packet_sniffer = FakePacketSniffer::new();
        let event_emitter = Arc::new(DefaultEventEmitter::new(meter_window.clone()));

        parser::start(
            options,
            connection_factory,
            packet_sniffer,
            event_emitter,
            port,
            settings,
            &region_file_path,
            &local_player_path,
            &mut flags,
            version
        ).map_err(|err| {
            error!("unexpected error occurred in parser: {}", err);
        })
    });

    // #[cfg(debug_assertions)]
    // {
    //     _logs_window.open_devtools();
    // }

    Ok(())
}

pub fn launch_update(handle: AppHandle) {
    
    tauri::async_runtime::spawn(async move {
        match tauri::updater::builder(handle).check().await {
            Ok(update) => {
                if update.is_update_available() {
                    #[cfg(not(debug_assertions))]
                    {
                        info!(
                            "update available, downloading update: v{}",
                            update.latest_version()
                        );

                        unload_driver();
                        remove_driver();

                        update
                            .download_and_install()
                            .await
                            .map_err(|e| {
                                error!("failed to download update: {}", e);
                            })
                            .ok();
                    }
                } else {
                    info!("no update available");
                }
            }
            Err(e) => {
                warn!("failed to get update: {}", e);
            }
        }
    });

}