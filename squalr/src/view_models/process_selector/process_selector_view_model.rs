use crate::view_models::view_model_base::ViewModel;
use crate::view_models::view_model_base::ViewModelBase;
use crate::MainWindowView;
use crate::ProcessSelectorViewModelBindings;
use crate::ProcessViewData;
use slint::ComponentHandle;
use slint::Image;
use slint::SharedPixelBuffer;
use squalr_engine::session_manager::SessionManager;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::logging::logger::Logger;
use squalr_engine_processes::process_info::ProcessIcon;
use squalr_engine_processes::process_info::ProcessInfo;
use squalr_engine_processes::process_query::process_queryer::ProcessQuery;
use squalr_engine_processes::process_query::process_queryer::ProcessQueryOptions;
use sysinfo::Pid;

pub struct ProcessSelectorViewModel {
    view_model_base: ViewModelBase<MainWindowView>,
}

impl ProcessSelectorViewModel {
    pub fn new(view_model_base: ViewModelBase<MainWindowView>) -> Self {
        let view = ProcessSelectorViewModel {
            view_model_base: view_model_base,
        };

        view.create_view_bindings();

        return view;
    }

    fn refresh_process_list(
        view_model_base: ViewModelBase<MainWindowView>,
        refresh_windowed_list: bool,
    ) {
        view_model_base.execute_on_ui_thread(move |main_window_view, _view_model_base| {
            let process_query_options = ProcessQueryOptions {
                require_windowed: refresh_windowed_list,
                required_pid: None,
                search_name: None,
                match_case: false,
                limit: None,
            };

            let start_time = std::time::Instant::now();

            let process_list = ProcessQuery::get_processes(process_query_options);
            let process_selector_view = main_window_view.global::<ProcessSelectorViewModelBindings>();

            Logger::get_instance().log(LogLevel::Info, &format!("Process gathering duration: {:?}", start_time.elapsed()), None);

            let start_time = std::time::Instant::now();

            // Gather process icons.
            let process_data_raw: Vec<(ProcessInfo, ProcessIcon)> = process_list
                .iter()
                .filter_map(|process_info| ProcessQuery::get_icon(&process_info.pid).map(|icon| (process_info.clone(), icon)))
                .collect();

            Logger::get_instance().log(LogLevel::Info, &format!("Process icons gathering duration: {:?}", start_time.elapsed()), None);

            let start_time = std::time::Instant::now();

            // Create shared pixel buffers from the icons.
            let process_data: Vec<ProcessViewData> = process_data_raw
                .into_iter()
                .map(|(process_info, icon)| {
                    let mut icon_data = SharedPixelBuffer::new(icon.width, icon.height);
                    let icon_data_bytes = icon_data.make_mut_bytes();
                    icon_data_bytes.copy_from_slice(&icon.bytes_rgba);

                    ProcessViewData {
                        process_id_str: process_info.pid.to_string().into(),
                        process_id: process_info.pid.as_u32() as i32,
                        name: process_info.name.to_string().into(),
                        icon: Image::from_rgba8(icon_data),
                    }
                })
                .collect();

            Logger::get_instance().log(LogLevel::Info, &format!("Pixel buffers creation duration: {:?}", start_time.elapsed()), None);

            if refresh_windowed_list {
                process_selector_view.set_windowed_processes(process_data.as_slice().into());
            } else {
                process_selector_view.set_processes(process_data.as_slice().into());
            }
        });
    }
}

impl ViewModel for ProcessSelectorViewModel {
    fn create_view_bindings(&self) {
        self.view_model_base
            .execute_on_ui_thread(move |main_window_view, view_model_base| {
                let process_selector_view = main_window_view.global::<ProcessSelectorViewModelBindings>();

                let view_model = view_model_base.clone();
                process_selector_view.on_refresh_full_process_list(move || {
                    Self::refresh_process_list(view_model.clone(), false);
                });

                let view_model = view_model_base.clone();
                process_selector_view.on_refresh_windowed_process_list(move || {
                    Self::refresh_process_list(view_model.clone(), true);
                });

                process_selector_view.on_select_process(|process_entry| {
                    let process_to_open = ProcessInfo {
                        pid: Pid::from_u32(process_entry.process_id as u32),
                        name: "".to_string(),
                    };
                    match ProcessQuery::open_process(&process_to_open) {
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
                });
            });
    }
}
