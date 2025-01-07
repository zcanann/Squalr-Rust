use squalr_engine_common::logging::{log_level::LogLevel, logger::Logger};
use squalr_engine_processes::{process_info::OpenedProcessInfo, process_query::process_queryer::ProcessQuery};
use squalr_engine_scanning::snapshots::snapshot::Snapshot;
use std::sync::{Arc, Once, RwLock};

pub struct SessionManager {
    opened_process: Option<OpenedProcessInfo>,
    snapshot: Arc<RwLock<Snapshot>>,
}

impl SessionManager {
    fn new() -> Self {
        SessionManager {
            opened_process: None,
            snapshot: Arc::new(RwLock::new(Snapshot::new(vec![]))),
        }
    }

    pub fn get_instance() -> Arc<RwLock<SessionManager>> {
        static mut INSTANCE: Option<Arc<RwLock<SessionManager>>> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = Arc::new(RwLock::new(SessionManager::new()));
                INSTANCE = Some(instance);
            });

            #[allow(static_mut_refs)]
            return INSTANCE.as_ref().unwrap_unchecked().clone();
        }
    }

    pub fn initialize(&self) {
        if let Err(err) = ProcessQuery::start_monitoring() {
            Logger::get_instance().log(LogLevel::Error, &format!("Failed to monitor system processes: {}", err), None);
        }
    }

    pub fn set_opened_process(
        &mut self,
        process_info: OpenedProcessInfo,
    ) {
        Logger::get_instance().log(
            LogLevel::Info,
            &format!("Opened process: {}, pid: {}", process_info.name, process_info.pid),
            None,
        );
        self.opened_process = Some(process_info);
    }

    pub fn clear_opened_process(&mut self) {
        self.opened_process = None;
        Logger::get_instance().log(LogLevel::Info, "Process closed", None);
    }

    pub fn get_opened_process(&self) -> Option<&OpenedProcessInfo> {
        self.opened_process.as_ref()
    }

    pub fn get_snapshot(&self) -> Arc<RwLock<Snapshot>> {
        return self.snapshot.clone();
    }
}
