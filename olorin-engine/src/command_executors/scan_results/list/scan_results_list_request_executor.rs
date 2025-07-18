use crate::command_executors::engine_request_executor::EngineCommandRequestExecutor;
use crate::engine_privileged_state::EnginePrivilegedState;
use olorin_engine_api::commands::scan_results::list::scan_results_list_request::ScanResultsListRequest;
use olorin_engine_api::commands::scan_results::list::scan_results_list_response::ScanResultsListResponse;
use olorin_engine_api::structures::scan_results::scan_result::ScanResult;
use olorin_engine_memory::memory_queryer::memory_queryer::MemoryQueryer;
use olorin_engine_memory::memory_queryer::memory_queryer_trait::IMemoryQueryer;
use olorin_engine_memory::memory_reader::MemoryReader;
use olorin_engine_memory::memory_reader::memory_reader_trait::IMemoryReader;
use olorin_engine_scanning::scan_settings_config::ScanSettingsConfig;
use std::sync::Arc;

impl EngineCommandRequestExecutor for ScanResultsListRequest {
    type ResponseType = ScanResultsListResponse;

    fn execute(
        &self,
        engine_privileged_state: &Arc<EnginePrivilegedState>,
    ) -> <Self as EngineCommandRequestExecutor>::ResponseType {
        let results_page_size = (ScanSettingsConfig::get_results_page_size() as u64).max(1);
        let mut scan_results_list = vec![];
        let mut last_page_index = 0;
        let mut result_count = 0;
        let mut total_size_in_bytes = 0;

        // Collect modules if possible so that we can resolve whether individual addresses are static later.
        let modules = if let Some(opened_process_info) = engine_privileged_state
            .get_process_manager()
            .get_opened_process()
        {
            MemoryQueryer::get_instance().get_modules(&opened_process_info)
        } else {
            vec![]
        };

        if let Ok(snapshot) = engine_privileged_state.get_snapshot().read() {
            result_count = snapshot.get_number_of_results();
            last_page_index = result_count.saturating_sub(1) / results_page_size;
            total_size_in_bytes = snapshot.get_byte_count();

            // Get the range of indicies for the elements of this page.
            let index_of_first_page_entry = self.page_index.clamp(0, last_page_index) * results_page_size;
            let index_of_last_page_entry = index_of_first_page_entry
                .saturating_add(results_page_size)
                .min(result_count);

            for result_index in index_of_first_page_entry..index_of_last_page_entry {
                let scan_result_base = match snapshot.get_scan_result(result_index) {
                    None => break,
                    Some(scan_result_base) => scan_result_base,
                };

                let mut recently_read_value = None;
                let mut module_name = String::default();
                let address = scan_result_base.get_address();
                let mut module_offset = address;

                // Best-effort attempt to read the values for this scan result.
                if let Some(opened_process_info) = engine_privileged_state
                    .get_process_manager()
                    .get_opened_process()
                {
                    if let Some(mut data_value) = scan_result_base.get_data_type().get_default_value() {
                        if MemoryReader::get_instance().read(&opened_process_info, address, &mut data_value) {
                            recently_read_value = Some(data_value);
                        }
                    }
                }

                // Check whether this scan result belongs to a module (ie check if the address is static).
                if let Some((found_module_name, address)) = MemoryQueryer::get_instance().address_to_module(address, &modules) {
                    module_name = found_module_name;
                    module_offset = address;
                }

                let is_frozen = if let Ok(snapshot_scan_result_freeze_list) = engine_privileged_state
                    .get_snapshot_scan_result_freeze_list()
                    .read()
                {
                    snapshot_scan_result_freeze_list.is_address_frozen(address)
                } else {
                    false
                };

                scan_results_list.push(ScanResult::new(scan_result_base, module_name, module_offset, recently_read_value, is_frozen));
            }
        }

        ScanResultsListResponse {
            scan_results: scan_results_list,
            page_index: self.page_index,
            page_size: results_page_size,
            last_page_index,
            result_count,
            total_size_in_bytes,
        }
    }
}
