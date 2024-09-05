use crate::ui;
use slint::*;
use std::sync::Arc;

pub struct TitleBarView {
    view_handle: Arc<ui::MainWindow>,
}

/// Wraps the slint main window to internally manage and track the view handle for later use, as well as setting up
/// view code bindings to the corresponding slint UI.
impl TitleBarView {
    pub fn new(view_handle: Arc<ui::MainWindow>) -> Self {
        let view = TitleBarView {
            view_handle: view_handle.clone(),
        };

        view.bind_view_to_ui();

        return view;
    }

    fn bind_view_to_ui(&self) {
        let title_bar_adapter = self.view_handle.global::<ui::TitleBarAdapter>();

        let view_handle = self.view_handle.clone();
        title_bar_adapter.on_minimize(move || {
            view_handle.window().set_minimized(true);
        });

        let view_handle = self.view_handle.clone();
        title_bar_adapter.on_maximize(move || {
            view_handle
                .window()
                .set_maximized(!view_handle.window().is_maximized());
        });

        title_bar_adapter.on_close(move || {
            let _ = slint::quit_event_loop();
        });
    }
}
