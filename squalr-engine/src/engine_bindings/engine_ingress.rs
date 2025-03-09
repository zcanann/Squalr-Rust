use crate::engine_privileged_state::EnginePrivilegedState;
use serde::{Deserialize, Serialize};
use squalr_engine_api::commands::{engine_command::EngineCommand, engine_response::EngineResponse};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EngineIngress {
    EngineCommand(EngineCommand),
}

pub trait ExecutableRequest {
    fn execute(
        &self,
        execution_context: &Arc<EnginePrivilegedState>,
    ) -> EngineResponse;
}
