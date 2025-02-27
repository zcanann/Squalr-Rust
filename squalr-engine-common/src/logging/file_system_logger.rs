use crate::logging::log_dispatcher::LogDispatcher;
use crossbeam_channel::{Receiver, Sender};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::{fs, sync::Arc, thread};
use std::{path::PathBuf, sync::RwLock};

pub struct FileSystemLogger {
    log_event_senders: Arc<RwLock<Vec<Sender<String>>>>,
}

impl FileSystemLogger {
    pub fn new() -> Self {
        let file_system_logger = FileSystemLogger {
            log_event_senders: Arc::new(RwLock::new(Vec::new())),
        };

        if let Err(err) = file_system_logger.initialize() {
            log::error!("Failed to initialize file system logging: {err}");
        }

        file_system_logger
    }

    pub fn subscribe_to_logs(&self) -> Result<Receiver<String>, String> {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut sender_lock = self.log_event_senders.write().map_err(|err| err.to_string())?;
        sender_lock.push(sender);
        Ok(receiver)
    }

    fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        let log_root_dir = Self::get_log_root_path();

        if !log_root_dir.exists() {
            std::fs::create_dir_all(&log_root_dir)?;
        }

        let log_file = Self::get_log_path();
        let backup_file = Self::get_log_backup_path();

        // If a log file already exists, rename it as a backup before creating a new log.
        if log_file.exists() {
            fs::rename(&log_file, &backup_file)?;
        }

        let file_appender = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} - {l} - {t} - {m}\n")))
            .build(log_file)?;

        let (log_dispatcher_sender, log_dispatcher_receiver) = crossbeam_channel::unbounded();
        let log_dispatcher = LogDispatcher::new(log_dispatcher_sender);

        let config = Config::builder()
            .appender(Appender::builder().build("file", Box::new(file_appender)))
            .appender(Appender::builder().build("log_events", Box::new(log_dispatcher)))
            .build(
                Root::builder()
                    .appender("file")
                    .appender("log_events")
                    .build(LevelFilter::Debug),
            )?;

        log4rs::init_config(config)?;

        let log_event_senders = self.log_event_senders.clone();

        // Listen for events from the log dispatcher, then re-dispatch them to all listeners.
        // This "daisy chain" of event listeners is done because direct access to LogDispatcher is difficult,
        // due to being abstracted behind a generic Appender. The easiest work-around is just to have
        // LogDispatcher be responsible for emitting a single event, then in FileSystemLogger we can fan it out
        // to multiple listeners, as we do here.
        thread::spawn(move || {
            while let Ok(log_message) = log_dispatcher_receiver.recv() {
                if let Ok(senders) = log_event_senders.read() {
                    for sender in senders.iter() {
                        let _ = sender.send(log_message.clone());
                    }
                }
            }
        });

        Ok(())
    }

    fn get_log_root_path() -> PathBuf {
        match dirs::data_local_dir() {
            Some(mut path) => {
                path.push("Squalr");
                path.push("logs");
                std::fs::create_dir_all(&path).unwrap_or_else(|err| {
                    log::error!("Failed to create logs directory: {err}");
                });
                path
            }
            None => {
                log::error!("Failed to get local app data directory");
                PathBuf::from("logs")
            }
        }
    }

    fn get_log_path() -> PathBuf {
        let mut log_path = Self::get_log_root_path();
        log_path.push("application.log");

        log_path
    }

    fn get_log_backup_path() -> PathBuf {
        let mut log_path = Self::get_log_root_path();
        log_path.push("application.log.bak");

        log_path
    }
}
