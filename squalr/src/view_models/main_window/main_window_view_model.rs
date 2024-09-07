use crate::view_models::view_model::ViewModel;
use crate::MainWindowView;
use crate::WindowViewModelBindings;
use slint::ComponentHandle;
use std::sync::Arc;

pub struct MainWindowViewModel {
    view_handle: Arc<MainWindowView>,
    // manual_scan_view: Arc<ManualScanViewModel>,
}

/// Wraps the slint main window to internally manage and track the view handle for later use, as well as setting up
/// view code bindings to the corresponding slint UI.
impl MainWindowViewModel {
    pub fn new() -> Self {
        let view_handle = Arc::new(MainWindowView::new().unwrap());
        let view = MainWindowViewModel {
            view_handle: view_handle.clone(),
            // manual_scan_view: Arc::new(ManualScanViewModel::new(view_handle.clone())),
        };

        view.create_bindings();

        return view;
    }

    pub fn run_event_loop(&self) {
        return self.view_handle.run().unwrap();
    }

    /*
    pub fn get_manual_scan_view(&self) -> &Arc<ManualScanViewModel> {
        return &self.manual_scan_view;
    } */
}

impl ViewModel for MainWindowViewModel {
    fn create_bindings(&self) {
        let view = self.view_handle.global::<WindowViewModelBindings>();

        let view_handle = self.view_handle.clone();
        view.on_minimize(move || {
            view_handle.window().set_minimized(true);
        });

        let view_handle = self.view_handle.clone();
        view.on_maximize(move || {
            view_handle
                .window()
                .set_maximized(!view_handle.window().is_maximized());
        });

        view.on_close(move || {
            /*
            let _ = slint::invoke_from_event_loop(|| {
                PanelWindowViewModel::new().show();
            }); */

            let _ = slint::quit_event_loop();
        });

        let view_handle = self.view_handle.clone();
        view.on_double_clicked(move || {
            view_handle
                .window()
                .set_maximized(!view_handle.window().is_maximized());
        });

        let view_handle = self.view_handle.clone();
        view.on_drag(move |delta_x, delta_y| {
            let mut position = view_handle.window().position();
            position.x = position.x + delta_x;
            position.y = position.y + delta_y;
            view_handle.window().set_position(position);
        });
    }
}
