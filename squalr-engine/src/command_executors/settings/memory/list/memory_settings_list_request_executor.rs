use crate::command_executors::engine_request_executor::EngineCommandRequestExecutor;
use crate::engine_privileged_state::EnginePrivilegedState;
use squalr_engine_api::commands::settings::memory::list::memory_settings_list_request::MemorySettingsListRequest;
use squalr_engine_api::commands::settings::memory::list::memory_settings_list_response::MemorySettingsListResponse;
use squalr_engine_memory::config::memory_settings_config::MemorySettingsConfig;
use std::sync::Arc;

impl EngineCommandRequestExecutor for MemorySettingsListRequest {
    type ResponseType = MemorySettingsListResponse;

    fn execute(
        &self,
        _engine_privileged_state: &Arc<EnginePrivilegedState>,
    ) -> <Self as EngineCommandRequestExecutor>::ResponseType {
        if let Ok(memory_config) = MemorySettingsConfig::get_full_config().read() {
            log::info!("{:?}", memory_config);
        }

        MemorySettingsListResponse {}
    }
}
