use crate::view_models::view_model::ViewModel;
use crate::MainWindowView;
use crate::ScanSettingsViewModelBindings;
use slint::ComponentHandle;
use std::sync::Arc;

pub struct ScanSettingsViewModel {
    view_handle: Arc<MainWindowView>,
}

impl ScanSettingsViewModel {
    pub fn new(view_handle: Arc<MainWindowView>) -> Self {
        let view = ScanSettingsViewModel {
            view_handle: view_handle.clone(),
        };

        view.create_bindings();

        return view;
    }
}

impl ViewModel for ScanSettingsViewModel {
    fn create_bindings(&self) {
        let _ = self.view_handle.global::<ScanSettingsViewModelBindings>();
    }
}
