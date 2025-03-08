use crate::command_executors::engine_request_executor::EngineRequestExecutor;
use crate::engine_privileged_state::EnginePrivilegedState;
use squalr_engine_api::commands::project::list::project_list_request::ProjectListRequest;
use squalr_engine_api::commands::project::list::project_list_response::ProjectListResponse;
use std::sync::Arc;

impl EngineRequestExecutor for ProjectListRequest {
    type ResponseType = ProjectListResponse;

    fn execute(
        &self,
        _execution_context: &Arc<EnginePrivilegedState>,
    ) -> <Self as EngineRequestExecutor>::ResponseType {
        // JIRA: Implement me
        ProjectListResponse {}
    }
}
