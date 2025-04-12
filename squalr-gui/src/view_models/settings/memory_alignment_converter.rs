use slint_mvvm::view_data_converter::ViewDataConverter;
use squalr_engine_api::structures::memory::memory_alignment::MemoryAlignment;

use crate::MemoryAlignmentView;

pub struct MemoryAlignmentConverter {}

impl MemoryAlignmentConverter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ViewDataConverter<MemoryAlignment, MemoryAlignmentView> for MemoryAlignmentConverter {
    fn convert_collection(
        &self,
        floating_point_tolerance_list: &Vec<MemoryAlignment>,
    ) -> Vec<MemoryAlignmentView> {
        floating_point_tolerance_list
            .into_iter()
            .map(|item| self.convert_to_view_data(item))
            .collect()
    }

    fn convert_to_view_data(
        &self,
        floating_point_tolerance: &MemoryAlignment,
    ) -> MemoryAlignmentView {
        match floating_point_tolerance {
            MemoryAlignment::Alignment1 => MemoryAlignmentView::Alignment1,
            MemoryAlignment::Alignment2 => MemoryAlignmentView::Alignment2,
            MemoryAlignment::Alignment4 => MemoryAlignmentView::Alignment4,
            MemoryAlignment::Alignment8 => MemoryAlignmentView::Alignment8,
        }
    }

    fn convert_from_view_data(
        &self,
        floating_point_tolerance_view: &MemoryAlignmentView,
    ) -> MemoryAlignment {
        match floating_point_tolerance_view {
            MemoryAlignmentView::Alignment1 => MemoryAlignment::Alignment1,
            MemoryAlignmentView::Alignment2 => MemoryAlignment::Alignment2,
            MemoryAlignmentView::Alignment4 => MemoryAlignment::Alignment4,
            MemoryAlignmentView::Alignment8 => MemoryAlignment::Alignment8,
        }
    }
}
