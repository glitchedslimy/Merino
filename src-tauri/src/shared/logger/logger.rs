use log::{Level, Log, Record};
use chrono::prelude::Local;

/// # MerinoLogger
/// _Custom implementation_ of `Log` facade.
/// 
/// Allows us to use macros such as:
/// * `error!()`: For logging errors
/// * `warn!()`: For logging warns
/// * `debug!()`: For debugging traces
/// * `info!()`: For logging info logs (Used to give info to the user)
/// * `trace!()`: A normal trace on the app
/// 
/// **It will return the following format:** `Time [LogLevel] - Message (File where it happened)`.
pub struct MerinoLogger;

impl Log for MerinoLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now();
            let hour = now.format("%H:%M:%S").to_string();

            let level_str = match record.level() {
                Level::Error => "ERROR",
                Level::Warn => "WARN",
                Level::Debug => "DEBUG",
                Level::Info => "INFO",
                Level::Trace => "TRACE"
            };

            println!("{} [{}] - {} on ({})", hour, level_str, record.args(), record.file().unwrap_or("unknown"))
        }
    }

    fn flush(&self) {}
}