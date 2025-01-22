use crate::command_handlers::memory;
use crate::command_handlers::process;
use crate::command_handlers::project;
use crate::command_handlers::results;
use crate::command_handlers::scan;
use crate::command_handlers::settings;
use crate::engine_command::EngineCommand;
use squalr_engine_architecture::vectors::vectors;
use squalr_engine_common::logging::{log_level::LogLevel, logger::Logger};
use squalr_engine_processes::{process_info::OpenedProcessInfo, process_query::process_queryer::ProcessQuery};
use squalr_engine_scanning::snapshots::snapshot::Snapshot;
use std::sync::{Arc, Once, RwLock};

pub struct SqualrEngine {
    opened_process: RwLock<Option<OpenedProcessInfo>>,
    snapshot: Arc<RwLock<Snapshot>>,
}

impl SqualrEngine {
    fn new() -> Self {
        SqualrEngine {
            opened_process: RwLock::new(None),
            snapshot: Arc::new(RwLock::new(Snapshot::new(vec![]))),
        }
    }

    fn get_instance() -> &'static SqualrEngine {
        static mut INSTANCE: Option<SqualrEngine> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                INSTANCE = Some(SqualrEngine::new());
            });

            #[allow(static_mut_refs)]
            INSTANCE.as_ref().unwrap_unchecked()
        }
    }

    pub fn initialize(ipc_mode: bool) {
        Logger::get_instance().log(LogLevel::Info, "Squalr started", None);
        vectors::log_vector_architecture();
        if let Err(err) = ProcessQuery::start_monitoring() {
            Logger::get_instance().log(LogLevel::Error, &format!("Failed to monitor system processes: {}", err), None);
        }
    }

    pub fn dispatch_command(command: &mut EngineCommand) {
        match command {
            EngineCommand::Memory(cmd) => memory::handle_memory_command(cmd),
            EngineCommand::Process(cmd) => process::handle_process_command(cmd),
            EngineCommand::Project(cmd) => project::handle_project_command(cmd),
            EngineCommand::Results(cmd) => results::handle_results_command(cmd),
            EngineCommand::Scan(cmd) => scan::handle_scan_command(cmd),
            EngineCommand::Settings(cmd) => settings::handle_settings_command(cmd),
        }
    }

    pub fn set_opened_process(process_info: OpenedProcessInfo) {
        let instance = Self::get_instance();
        if let Ok(mut process) = instance.opened_process.write() {
            Logger::get_instance().log(
                LogLevel::Info,
                &format!("Opened process: {}, pid: {}", process_info.name, process_info.pid),
                None,
            );
            *process = Some(process_info);
        }
    }

    pub fn clear_opened_process() {
        let instance = Self::get_instance();
        if let Ok(mut process) = instance.opened_process.write() {
            *process = None;
            Logger::get_instance().log(LogLevel::Info, "Process closed", None);
        }
    }

    pub fn get_opened_process() -> Option<OpenedProcessInfo> {
        let instance = Self::get_instance();
        instance
            .opened_process
            .read()
            .ok()
            .and_then(|guard| guard.clone())
    }

    pub fn get_snapshot() -> Arc<RwLock<Snapshot>> {
        let instance = Self::get_instance();
        instance.snapshot.clone()
    }
}
