use crate::MainWindowView;
use crate::ProcessSelectorViewModelBindings;
use crate::ProcessViewData;
use crate::view_models::process_selector::process_info_comparer::ProcessInfoComparer;
use crate::view_models::process_selector::process_info_converter::ProcessInfoConverter;
use slint::ComponentHandle;
use slint_mvvm::view_binding::ViewBinding;
use slint_mvvm::view_collection_binding::ViewCollectionBinding;
use slint_mvvm_macros::create_view_bindings;
use slint_mvvm_macros::create_view_model_collection;
use squalr_engine::commands::engine_request::EngineRequest;
use squalr_engine::commands::process::list::process_list_request::ProcessListRequest;
use squalr_engine::commands::process::open::process_open_request::ProcessOpenRequest;
use squalr_engine_processes::process_info::ProcessInfo;

pub struct ProcessSelectorViewModel {
    _view_binding: ViewBinding<MainWindowView>,
    _full_process_list_collection: ViewCollectionBinding<ProcessViewData, ProcessInfo, MainWindowView>,
    _windowed_process_list_collection: ViewCollectionBinding<ProcessViewData, ProcessInfo, MainWindowView>,
}

impl ProcessSelectorViewModel {
    pub fn new(view_binding: ViewBinding<MainWindowView>) -> Self {
        // Create a binding that allows us to easily update the view's process list.
        let full_process_list_collection = create_view_model_collection!(
            view_binding -> MainWindowView,
            ProcessSelectorViewModelBindings -> { set_processes, get_processes },
            ProcessInfoConverter -> [],
            ProcessInfoComparer -> [],
        );

        // Create a binding that allows us to easily update the view's windowed process list.
        let windowed_process_list_collection = create_view_model_collection!(
            view_binding -> MainWindowView,
            ProcessSelectorViewModelBindings -> { set_windowed_processes, get_windowed_processes },
            ProcessInfoConverter -> [],
            ProcessInfoComparer -> [],
        );

        let view = ProcessSelectorViewModel {
            _view_binding: view_binding.clone(),
            _full_process_list_collection: full_process_list_collection.clone(),
            _windowed_process_list_collection: windowed_process_list_collection.clone(),
        };

        // Route all view bindings to Rust.
        create_view_bindings!(view_binding, {
            ProcessSelectorViewModelBindings => {
                on_refresh_full_process_list() -> [full_process_list_collection] -> Self::on_refresh_full_process_list
                on_refresh_windowed_process_list() -> [windowed_process_list_collection] -> Self::on_refresh_windowed_process_list
                on_select_process(process_entry: ProcessViewData) -> [] -> Self::on_select_process
            }
        });

        view.listen_for_process_change(view_binding.clone());

        view
    }

    fn listen_for_process_change(
        &self,
        view_binding: ViewBinding<MainWindowView>,
    ) {
    }

    fn on_refresh_full_process_list(full_process_list_collection: ViewCollectionBinding<ProcessViewData, ProcessInfo, MainWindowView>) {
        let list_all_processes_request = ProcessListRequest {
            require_windowed: false,
            search_name: None,
            match_case: false,
            limit: None,
            fetch_icons: true,
        };

        list_all_processes_request.send(move |process_list_response| {
            full_process_list_collection.update_from_source(process_list_response.processes);
        });
    }

    fn on_refresh_windowed_process_list(windowed_process_list_collection: ViewCollectionBinding<ProcessViewData, ProcessInfo, MainWindowView>) {
        let list_windowed_processes_request = ProcessListRequest {
            require_windowed: true,
            search_name: None,
            match_case: false,
            limit: None,
            fetch_icons: true,
        };

        list_windowed_processes_request.send(move |process_list_response| {
            windowed_process_list_collection.update_from_source(process_list_response.processes);
        });
    }

    fn on_select_process(process_entry: ProcessViewData) {
        let open_process_command = ProcessOpenRequest {
            process_id: Some(process_entry.process_id as u32),
            search_name: None,
            match_case: false,
        };

        open_process_command.send(|_process_open_response| {});
    }
}
