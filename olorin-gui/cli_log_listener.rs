use olorin_engine_common::logging::log_level::LogLevel;
use olorin_engine_common::logging::logger_observer::LoggerObserver;
use std::sync::Arc;

pub struct CliLogListener;

impl LoggerObserver for CliLogListener {
    fn on_log_event(
        &self,
        log_level: LogLevel,
        message: &str,
        inner_message: Option<&str>,
    ) {
        match inner_message {
            Some(inner) => println!("[{:?}] {} - {}", log_level, message, inner),
            None => println!("[{:?}] {}", log_level, message),
        }
    }
}

impl CliLogListener {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}
