use crate::MainWindowView;
use crate::WindowViewModelBindings;
use crate::view_models::docking::dock_root_view_model::DockRootViewModel;
use slint::ComponentHandle;
use slint_mvvm::view_binding::ViewBinding;
use slint_mvvm_macros::create_view_bindings;
use squalr_engine::engine_execution_context::EngineExecutionContext;
use squalr_engine_common::logging::file_system_logger::FileSystemLogger;
use std::sync::Arc;

pub struct MainWindowViewModel {
    _view: MainWindowView,
    view_binding: ViewBinding<MainWindowView>,
    _dock_root_view_model: Arc<DockRootViewModel>,
    _engine_execution_context: Arc<EngineExecutionContext>,
}

impl MainWindowViewModel {
    pub fn new(
        engine_execution_context: &Arc<EngineExecutionContext>,
        file_system_logger: &Arc<FileSystemLogger>,
    ) -> Self {
        let view = MainWindowView::new().unwrap();
        let view_binding = ViewBinding::new(ComponentHandle::as_weak(&view));
        let dock_root_view_model = Arc::new(DockRootViewModel::new(
            view_binding.clone(),
            engine_execution_context.clone(),
            file_system_logger.clone(),
        ));

        let view: MainWindowViewModel = MainWindowViewModel {
            _view: view,
            view_binding: view_binding.clone(),
            _dock_root_view_model: dock_root_view_model.clone(),
            _engine_execution_context: engine_execution_context.clone(),
        };

        // Logger::subscribe(dock_root_view_model.get_output_view_model().clone());

        create_view_bindings!(view_binding, {
            WindowViewModelBindings => {
                on_minimize() -> [view_binding] -> Self::on_minimize,
                on_maximize() -> [view_binding] -> Self::on_maximize,
                on_close() -> [] -> Self::on_close,
                on_double_clicked() -> [view_binding] -> Self::on_double_clicked,
                on_drag(delta_x: i32, delta_y: i32) -> [view_binding] -> Self::on_drag
            }
        });

        view.show();

        view
    }

    pub fn initialize(&self) {
        self.show();
    }

    pub fn show(&self) {
        if let Ok(handle) = self.view_binding.get_view_handle().lock() {
            if let Some(view) = handle.upgrade() {
                if let Err(err) = view.show() {
                    log::error!("Error showing the main window: {err}");
                }
            }
        }
    }

    pub fn hide(&self) {
        if let Ok(handle) = self.view_binding.get_view_handle().lock() {
            if let Some(view) = handle.upgrade() {
                if let Err(err) = view.hide() {
                    log::error!("Error hiding the main window: {err}");
                }
            }
        }
    }

    fn on_minimize(view_binding: ViewBinding<MainWindowView>) {
        view_binding.execute_on_ui_thread(move |main_window_view, _view_binding| {
            let window = main_window_view.window();
            window.set_minimized(true);
        });
    }

    fn on_maximize(view_binding: ViewBinding<MainWindowView>) {
        view_binding.execute_on_ui_thread(move |main_window_view, _view_binding| {
            let window = main_window_view.window();
            window.set_maximized(!window.is_maximized());
        });
    }

    fn on_close() {
        if let Err(err) = slint::quit_event_loop() {
            log::error!("Failed to quit event loop: {}", err);
        }
    }

    fn on_double_clicked(view_binding: ViewBinding<MainWindowView>) {
        view_binding.execute_on_ui_thread(move |main_window_view, _view_binding| {
            let window = main_window_view.window();
            window.set_maximized(!window.is_maximized());
        });
    }

    fn on_drag(
        view_binding: ViewBinding<MainWindowView>,
        delta_x: i32,
        delta_y: i32,
    ) {
        view_binding.execute_on_ui_thread(move |main_window_view, _view_binding| {
            let window = main_window_view.window();
            let mut position = window.position();
            position.x += delta_x;
            position.y += delta_y;
            window.set_position(position);
        });
    }
}
