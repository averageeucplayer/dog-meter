use flexi_logger::{
    Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, LoggerHandle, Naming, WriteMode,
};

use log::{error, info, warn, Record};

use std::sync::{Mutex, OnceLock};

fn get_app_state() -> &'static Mutex<AppState> {
    static APP_STATE: OnceLock<Mutex<AppState>> = OnceLock::new();
    APP_STATE.get_or_init(|| Mutex::new(AppState::new()))
}

struct AppState {
    pub logger_handle: Option<LoggerHandle>,
}

pub fn debug_print(args: std::fmt::Arguments<'_>) {
    #[cfg(debug_assertions)]
    {
        info!("{}", args);
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            logger_handle: None,
        }
    }

    pub fn init_logger(&mut self) {
        if self.logger_handle.is_some() {
            error!("AppState logger already inited");
            return;
        }

        let mut resource_directory =
            std::env::current_exe().expect("Can't find path to executable");
        resource_directory.pop();

        let mut logger = if cfg!(debug_assertions) {
            Logger::try_with_str("info, tao=off").unwrap()
        } else {
            Logger::try_with_str("warn, tao=off").unwrap()
        };

        logger = logger
            .log_to_file(
                FileSpec::default()
                    .suppress_timestamp()
                    .basename("loa_logs")
                    .directory(resource_directory),
            )
            .use_utc()
            .write_mode(WriteMode::BufferAndFlush)
            .append()
            .format(AppState::default_format_with_time)
            .rotate(
                Criterion::Size(5_000_000),
                Naming::Timestamps,
                Cleanup::KeepLogFiles(2),
            );

        #[cfg(debug_assertions)]
        {
            logger = logger.duplicate_to_stdout(Duplicate::All);
        }

        self.logger_handle = logger.start().ok();
    }

    pub fn get_logger(&self) -> Option<LoggerHandle> {
        self.logger_handle.clone()
    }

    fn default_format_with_time(
        w: &mut dyn std::io::Write,
        now: &mut DeferredNow,
        record: &Record,
    ) -> Result<(), std::io::Error> {
        write!(
            w,
            "[{}] {} [{}] {}",
            now.format("%Y-%m-%dT%H:%M:%S%.6fZ"),
            record.level(),
            record.module_path().unwrap_or("<unnamed>"),
            record.args()
        )
    }
}

pub fn init() {
    get_app_state().lock().unwrap().init_logger();
}

pub fn get_logger() -> Result<LoggerHandle, String> {
    if let Some(logger_handle) = get_app_state().lock().unwrap().get_logger() {
        return Ok(logger_handle);
    }
    Err("AppState logger not present".to_string())
}
