use std::process::Command;

use log::{error, info, warn};
use sysinfo::System;

use crate::{logger, constants::{DELETE_WINDIVERT_SERVICE_COMMAND, STEAM_GAME_COMMAND, STOP_WINDIVERT_COMMAND, WINDOWS_SERVICE_CONTROL}};

pub fn unload_driver() {
    let mut command = Command::new(WINDOWS_SERVICE_CONTROL);
    command.args(STOP_WINDIVERT_COMMAND);
    let output = command.output();

    match output {
        Ok(output) => {
            if output.status.success() {
                info!("stopped driver");
            }
        }
        Err(_) => {
            warn!("could not execute command to stop driver");
        }
    }
}

pub fn remove_driver() {
    let mut command = Command::new(WINDOWS_SERVICE_CONTROL);
    command.args(DELETE_WINDIVERT_SERVICE_COMMAND);
    command.output().expect("unable to delete driver");
}

pub fn start_loa_process() {
    if !check_loa_running() {
        info!("starting lost ark process...");
        let mut command = Command::new("cmd");

        command
            .args(STEAM_GAME_COMMAND)
            .spawn()
            .map_err(|e| error!("could not open lost ark: {}", e))
            .ok();
    } else {
        info!("lost ark already running")
    }
}

pub fn check_loa_running() -> bool {
    let system = System::new_all();
    let process_name = "lostark.exe";

    // Iterate through all running processes
    for process in system.processes().values() {
        if process.name().to_string_lossy().to_ascii_lowercase() == process_name {
            return true;
        }
    }
    false
}

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic message".to_string()
        };

        let location = info.location().map_or("unknown location".to_string(), |location| {
            format!("{}:{}", location.file(), location.line())
        });

        error!("Panicked at '{}', {}", message, location);

        logger::get_logger().unwrap().flush();
    }));
}