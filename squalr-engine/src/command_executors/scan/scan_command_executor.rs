use crate::command_executors::engine_command_executor::EngineCommandExecutor;
use crate::command_executors::engine_request_executor::EngineRequestExecutor;
use crate::engine_execution_context::EngineExecutionContext;
use squalr_engine_api::commands::engine_response::{EngineResponse, TypedEngineResponse};
use squalr_engine_api::commands::scan::scan_command::ScanCommand;
use std::sync::Arc;

impl EngineCommandExecutor for ScanCommand {
    type ResponseType = EngineResponse;

    fn execute(
        &self,
        execution_context: &Arc<EngineExecutionContext>,
    ) -> <Self as EngineCommandExecutor>::ResponseType {
        match self {
            ScanCommand::CollectValues { scan_value_collector_request } => scan_value_collector_request
                .execute(execution_context)
                .to_engine_response(),
            ScanCommand::Hybrid { scan_hybrid_request } => scan_hybrid_request
                .execute(execution_context)
                .to_engine_response(),
            ScanCommand::New { scan_new_request } => scan_new_request.execute(execution_context).to_engine_response(),
            ScanCommand::Manual { scan_manual_request } => scan_manual_request
                .execute(execution_context)
                .to_engine_response(),
        }
    }
}
