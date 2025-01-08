use crate::MainWindowView;
use crate::ProcessSelectorViewModelBindings;
use crate::ProcessViewData;
use crate::mvvm::view_binding::ViewBinding;
use crate::mvvm::view_data_converter::ViewDataConverter;
use crate::mvvm::view_model_collection::ViewModelCollection;
use crate::view_models::process_selector::process_info_converter::ProcessInfoConverter;
use slint::ComponentHandle;
use slint_mvvm_macros::create_view_bindings;
use squalr_engine::session_manager::SessionManager;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::logging::logger::Logger;
use squalr_engine_processes::process_info::ProcessInfo;
use squalr_engine_processes::process_query::process_queryer::ProcessQuery;
use squalr_engine_processes::process_query::process_queryer::ProcessQueryOptions;
use sysinfo::Pid;

pub struct ProcessSelectorViewModel {
    _view_binding: ViewBinding<MainWindowView>,
    _processes: ViewModelCollection<ProcessViewData, ProcessInfo, MainWindowView>,
    _windowed_processes: ViewModelCollection<ProcessViewData, ProcessInfo, MainWindowView>,
}

impl ProcessSelectorViewModel {
    pub fn new(view_binding: ViewBinding<MainWindowView>) -> Self {
        let processes = view_binding.create_collection(
            |process: ProcessInfo| ProcessInfoConverter.convert(process),
            |view: &MainWindowView, model| {
                view.global::<ProcessSelectorViewModelBindings>()
                    .set_processes(model)
            },
        );

        let windowed_processes = view_binding.create_collection(
            |process: ProcessInfo| ProcessInfoConverter.convert(process),
            |view: &MainWindowView, model| {
                view.global::<ProcessSelectorViewModelBindings>()
                    .set_windowed_processes(model)
            },
        );

        let view = ProcessSelectorViewModel {
            _view_binding: view_binding.clone(),
            _processes: processes.clone(),
            _windowed_processes: windowed_processes.clone(),
        };

        create_view_bindings!(view_binding, {
            ProcessSelectorViewModelBindings => {
                on_refresh_full_process_list() -> [processes] -> Self::on_refresh_full_process_list
                on_refresh_windowed_process_list() -> [windowed_processes] -> Self::on_refresh_windowed_process_list
                on_select_process(process_entry: ProcessViewData) -> [] -> Self::on_select_process
            }
        });

        view
    }

    fn get_process_query_options(
        required_pid: Option<Pid>,
        require_windowed: bool,
        limit: Option<u64>,
    ) -> ProcessQueryOptions {
        ProcessQueryOptions {
            required_pid: required_pid,
            search_name: None,
            require_windowed: require_windowed,
            match_case: false,
            fetch_icons: true,
            limit: limit,
        }
    }

    fn on_refresh_full_process_list(process_info_converter: ViewModelCollection<ProcessViewData, ProcessInfo, MainWindowView>) {
        let process_query_options = Self::get_process_query_options(None, false, None);
        let processes = ProcessQuery::get_processes(process_query_options);
        process_info_converter.update_from_source(processes);
    }

    fn on_refresh_windowed_process_list(windowed_process_info_converter: ViewModelCollection<ProcessViewData, ProcessInfo, MainWindowView>) {
        let process_query_options = Self::get_process_query_options(None, true, None);
        let processes = ProcessQuery::get_processes(process_query_options);
        windowed_process_info_converter.update_from_source(processes);
    }

    fn on_select_process(process_entry: ProcessViewData) {
        let process_query_options = Self::get_process_query_options(Some(Pid::from_u32(process_entry.process_id as u32)), true, Some(1));
        let processes = ProcessQuery::get_processes(process_query_options);

        if let Some(process_to_open) = processes.first() {
            match ProcessQuery::open_process(process_to_open) {
                Ok(opened_process) => {
                    if let Ok(mut session_manager) = SessionManager::get_instance().write() {
                        session_manager.set_opened_process(opened_process);
                    } else {
                        Logger::get_instance().log(LogLevel::Warn, "Failed to open process.", None);
                    }
                }
                Err(err) => {
                    Logger::get_instance().log(LogLevel::Error, &format!("Failed to open process {}: {}", process_to_open.pid, err), None);
                }
            }
        }
    }
}
