use crate::commands::engine_command_response::EngineCommandResponse;
use crate::commands::engine_command_response::TypedEngineCommandResponse;
use crate::commands::trackable_tasks::trackable_tasks_response::TrackableTasksResponse;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackableTasksListResponse {}

impl TypedEngineCommandResponse for TrackableTasksListResponse {
    fn to_engine_response(&self) -> EngineCommandResponse {
        EngineCommandResponse::TrackableTasks(TrackableTasksResponse::List {
            trackable_tasks_list_response: self.clone(),
        })
    }

    fn from_engine_response(response: EngineCommandResponse) -> Result<Self, EngineCommandResponse> {
        if let EngineCommandResponse::TrackableTasks(TrackableTasksResponse::List { trackable_tasks_list_response }) = response {
            Ok(trackable_tasks_list_response)
        } else {
            Err(response)
        }
    }
}
