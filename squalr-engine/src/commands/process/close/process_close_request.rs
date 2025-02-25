use std::sync::Arc;

use crate::commands::engine_command::EngineCommand;
use crate::commands::engine_request::EngineRequest;
use crate::commands::process::close::process_close_response::ProcessCloseResponse;
use crate::commands::process::process_command::ProcessCommand;
use crate::commands::process::process_response::ProcessResponse;
use crate::engine_execution_context::EngineExecutionContext;
use serde::{Deserialize, Serialize};
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::logging::logger::Logger;
use squalr_engine_processes::process_query::process_queryer::ProcessQuery;
use structopt::StructOpt;

#[derive(Clone, StructOpt, Debug, Serialize, Deserialize)]
pub struct ProcessCloseRequest {}

impl EngineRequest for ProcessCloseRequest {
    type ResponseType = ProcessCloseResponse;

    fn execute(
        &self,
        execution_context: &Arc<EngineExecutionContext>,
    ) -> Self::ResponseType {
        if let Some(process_info) = execution_context.get_opened_process() {
            Logger::log(
                LogLevel::Info,
                &format!("Closing process {} with handle {}", process_info.process_id, process_info.handle),
                None,
            );

            match ProcessQuery::close_process(process_info.handle) {
                Ok(_) => {
                    execution_context.clear_opened_process();
                }
                Err(err) => {
                    Logger::log(
                        LogLevel::Error,
                        &format!("Failed to close process handle {}: {}", process_info.handle, err),
                        None,
                    );
                }
            }

            ProcessCloseResponse {
                process_info: Some(process_info),
            }
        } else {
            Logger::log(LogLevel::Info, "No process to close", None);
            ProcessCloseResponse { process_info: None }
        }
    }

    fn to_engine_command(&self) -> EngineCommand {
        EngineCommand::Process(ProcessCommand::Close {
            process_close_request: self.clone(),
        })
    }
}

impl From<ProcessCloseResponse> for ProcessResponse {
    fn from(process_close_response: ProcessCloseResponse) -> Self {
        ProcessResponse::Close { process_close_response }
    }
}
