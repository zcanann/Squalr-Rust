use serde::{Deserialize, Serialize};
use squalr_engine_common::structures::processes::process_info::OpenedProcessInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessChangedEvent {
    pub process_info: Option<OpenedProcessInfo>,
}
