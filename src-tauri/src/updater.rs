use log::*;
use tauri::{async_runtime::spawn, updater::builder, AppHandle};

use crate::utils::unload_driver;

pub fn launch_update(handle: AppHandle) {
    
    spawn(async move {
        match builder(handle).check().await {
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