use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use log::info;
use tauri::{Manager, Window, Wry};


pub struct Flags {
    reset: Arc<AtomicBool>,
    pause: Arc<AtomicBool>,
    save: Arc<AtomicBool>,
    boss_only_damage: Arc<AtomicBool>,
    emit_details: Arc<AtomicBool>,
}

impl Flags {
    pub fn new(event_listener: Window<Wry>) -> Self {
        let reset = Arc::new(AtomicBool::new(false));
        let pause = Arc::new(AtomicBool::new(false));
        let save = Arc::new(AtomicBool::new(false));
        let boss_only_damage = Arc::new(AtomicBool::new(false));
        let emit_details: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        let meter_window_clone = event_listener.clone();
        event_listener.listen_global("reset-request", {
            let reset_clone = reset.clone();
            let meter_window_clone = meter_window_clone.clone();
            move |_event| {
                reset_clone.store(true, Ordering::Relaxed);
                info!("resetting meter");
                meter_window_clone.emit("reset-encounter", "").ok();
            }
        });

        event_listener.listen_global("save-request", {
            let save_clone = save.clone();
            let meter_window_clone = meter_window_clone.clone();
            move |_event| {
                save_clone.store(true, Ordering::Relaxed);
                info!("manual saving encounter");
                meter_window_clone.emit("save-encounter", "").ok();
            }
        });
    
        event_listener.listen_global("pause-request", {
            let pause_clone = pause.clone();
            let meter_window_clone = meter_window_clone.clone();
            move |_event| {
                let prev = pause_clone.fetch_xor(true, Ordering::Relaxed);
                if prev {
                    info!("unpausing meter");
                } else {
                    info!("pausing meter");
                }
                meter_window_clone.emit("pause-encounter", "").ok();
            }
        });
    
        event_listener.listen_global("boss-only-damage-request", {
            let boss_only_damage = boss_only_damage.clone();
            move |event| {
                if let Some(bod) = event.payload() {
                    if bod == "true" {
                        boss_only_damage.store(true, Ordering::Relaxed);
                        info!("boss only damage enabled")
                    } else {
                        boss_only_damage.store(false, Ordering::Relaxed);
                        info!("boss only damage disabled")
                    }
                }
            }
        });
    
        event_listener.listen_global("emit-details-request", {
            let emit_clone = emit_details.clone();
            move |_event| {
                let prev = emit_clone.fetch_xor(true, Ordering::Relaxed);
                if prev {
                    info!("stopped sending details");
                } else {
                    info!("sending details");
                }
            }
        });

        Self {
            pause,
            reset,
            boss_only_damage,
            save,
            emit_details
        }
    }

    pub fn is_reset_invoked(&self) -> bool {
        self.reset.load(Ordering::Relaxed)
    }

    pub fn is_pause_invoked(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }

    pub fn is_save_invoked(&self) -> bool {
        self.save.load(Ordering::Relaxed)
    }

    pub fn clear_reset(&mut self) {
        self.reset.store(false, Ordering::Relaxed);
    }

    pub fn clear_save(&mut self) {
        self.save.store(false, Ordering::Relaxed);
    }

    pub fn is_boss_only_damage_invoked(&self) -> bool {
        self.boss_only_damage.load(Ordering::Relaxed)
    }

    pub fn set_boss_only_damage(&mut self) {
        self.boss_only_damage.store(true, Ordering::Relaxed);
    }

    pub fn can_emit_details(&self) -> bool {
        self.emit_details.load(Ordering::Relaxed)
    }
}
