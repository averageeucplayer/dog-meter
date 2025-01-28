use std::error::Error;

use serde::Serialize;
use tauri::{Window, Wry};

pub trait AppEvent<T: Serialize + Clone> {
    fn name(&self) -> &'static str;
    fn payload(&self) -> T;
}

pub trait EventEmitter: Send + Sync + 'static {
    fn emit<T: Serialize + Clone>(&self, event: impl AppEvent<T>) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct DefaultEventEmitter {
    emitter: Window<Wry>
}

impl EventEmitter for DefaultEventEmitter {
    fn emit<T: Serialize + Clone>(&self, event: impl AppEvent<T>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let name = event.name().to_string();
        self.emitter.emit(&name, event.payload())?;
        Ok(())
    }
}

impl DefaultEventEmitter {
    pub fn new(emitter: Window<Wry>) -> Self {
        Self { emitter }
    }
}

