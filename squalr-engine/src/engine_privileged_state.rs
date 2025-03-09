use crate::engine_bindings::interprocess::interprocess_privileged_shell::InterprocessPrivilegedShell;
use crate::engine_bindings::{engine_priviliged_bindings::EnginePrivilegedBindings, standalone::standalone_privileged_engine::StandalonePrivilegedEngine};
use crate::engine_mode::EngineMode;
use crate::tasks::trackable_task_manager::TrackableTaskManager;
use crossbeam_channel::Receiver;
use squalr_engine_api::events::engine_event::EngineEvent;
use squalr_engine_api::events::process::process_changed_event::ProcessChangedEvent;
use squalr_engine_api::structures::processes::process_info::OpenedProcessInfo;
use squalr_engine_api::structures::tasks::engine_trackable_task_handle::EngineTrackableTaskHandle;
use squalr_engine_processes::process_query::process_queryer::ProcessQuery;
use squalr_engine_scanning::snapshots::snapshot::Snapshot;
use std::sync::{Arc, RwLock};

/// Tracks critical engine state for internal use. This includes executing engine tasks, commands, and events.
pub struct EnginePrivilegedState {
    /// The process to which Squalr is attached.
    opened_process: RwLock<Option<OpenedProcessInfo>>,

    /// The current snapshot of process memory, including any scan results.
    snapshot: Arc<RwLock<Snapshot>>,

    /// The manager that tracks all running engine tasks.
    task_manager: TrackableTaskManager,

    /// Defines functionality that can be invoked by the engine for the GUI or CLI to handle.
    engine_bindings: Arc<RwLock<dyn EnginePrivilegedBindings>>,
}

impl EnginePrivilegedState {
    pub fn new(engine_mode: EngineMode) -> Arc<Self> {
        let engine_bindings: Arc<RwLock<dyn EnginePrivilegedBindings>> = match engine_mode {
            EngineMode::Standalone => Arc::new(RwLock::new(StandalonePrivilegedEngine::new())),
            EngineMode::PrivilegedShell => Arc::new(RwLock::new(InterprocessPrivilegedShell::new())),
            EngineMode::UnprivilegedHost => unreachable!("Privileged state should never be created on an unprivileged host."),
        };

        let execution_context = Arc::new(EnginePrivilegedState {
            opened_process: RwLock::new(None),
            snapshot: Arc::new(RwLock::new(Snapshot::new())),
            task_manager: TrackableTaskManager::new(),
            engine_bindings,
        });

        execution_context
    }

    pub fn initialize(
        &self,
        engine_privileged_state: &Option<Arc<EnginePrivilegedState>>,
    ) {
        match self.engine_bindings.write() {
            Ok(mut engine_bindings) => {
                if let Err(err) = engine_bindings.initialize(engine_privileged_state) {
                    log::error!("Error initializing privileged engine bindings: {}", err);
                }
            }
            Err(err) => {
                log::error!("Failed to acquire privileged engine bindings write lock: {}", err);
            }
        }

        if let Err(err) = ProcessQuery::start_monitoring() {
            log::error!("Failed to monitor system processes: {}", err);
        }
    }

    /// Sets the process to which we are currently attached.
    pub fn set_opened_process(
        &self,
        process_info: OpenedProcessInfo,
    ) {
        if let Ok(mut process) = self.opened_process.write() {
            log::info!("Opened process: {}, pid: {}", process_info.name, process_info.process_id);
            *process = Some(process_info.clone());

            self.emit_event(EngineEvent::Process(ProcessChangedEvent {
                process_info: Some(process_info),
            }));
        }
    }

    /// Clears the process to which we are currently attached.
    pub fn clear_opened_process(&self) {
        if let Ok(mut process) = self.opened_process.write() {
            *process = None;
            log::info!("Process closed");
            self.emit_event(EngineEvent::Process(ProcessChangedEvent { process_info: None }));
        }
    }

    /// Gets the process to which we are currently attached, if any.
    pub fn get_opened_process(&self) -> Option<OpenedProcessInfo> {
        match self.opened_process.read() {
            Ok(opened_process) => opened_process.clone(),
            Err(err) => {
                log::error!("Failed to access opened process: {}", err);
                None
            }
        }
    }

    /// Gets the current snapshot, which contains all captured memory and scan results.
    pub fn get_snapshot(&self) -> Arc<RwLock<Snapshot>> {
        self.snapshot.clone()
    }

    /// Dispatches an event from the engine.
    pub fn emit_event(
        &self,
        event: EngineEvent,
    ) {
        match self.engine_bindings.read() {
            Ok(engine_bindings) => {
                if let Err(err) = engine_bindings.emit_event(event) {
                    log::error!("Error dispatching engine event: {}", err);
                }
            }
            Err(err) => {
                log::error!("Failed to acquire privileged engine bindings read lock: {}", err);
            }
        }
    }

    /// Dispatches an event from the engine.
    pub fn subscribe_to_engine_events(&self) -> Result<Receiver<EngineEvent>, String> {
        match self.engine_bindings.read() {
            Ok(engine_bindings) => engine_bindings.subscribe_to_engine_events(),
            Err(err) => Err(format!("Failed to acquire privileged engine bindings read lock: {}", err)),
        }
    }

    /// Registers a task handle to be tracked by the engine task manager.
    pub fn register_task(
        &self,
        trackable_task_handle: EngineTrackableTaskHandle,
    ) {
        self.task_manager.register_task(trackable_task_handle);
    }

    /// Unregisters a task handle, after which the task manager no longer tracks it.
    pub fn unregister_task(
        &self,
        task_identifier: &String,
    ) {
        self.task_manager.unregister_task(task_identifier);
    }
}
