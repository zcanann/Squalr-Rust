use crate::{engine_execution_context::EngineExecutionContext, engine_privileged_state::EnginePrivilegedState};
use crossbeam_channel::Receiver;
use squalr_engine_api::{
    commands::{engine_command::EngineCommand, engine_response::EngineResponse},
    events::engine_event::EngineEvent,
};
use std::sync::Arc;

/// Defines the functionality that is invoked from a GUI, CLI, etc. and handled by the engine.
pub trait EngineUnprivilegedBindings: Send + Sync {
    fn initialize(
        &mut self,
        engine_privileged_state: &Option<Arc<EnginePrivilegedState>>,
        engine_execution_context: &Option<Arc<EngineExecutionContext>>,
    ) -> Result<(), String>;

    /// Dispatches an engine command to the engine to handle.
    fn dispatch_command(
        &self,
        command: EngineCommand,
        callback: Box<dyn FnOnce(EngineResponse) + Send + Sync + 'static>,
    ) -> Result<(), String>;

    /// Requests to listen to all engine events.
    fn subscribe_to_engine_events(&self) -> Result<Receiver<EngineEvent>, String>;
}
