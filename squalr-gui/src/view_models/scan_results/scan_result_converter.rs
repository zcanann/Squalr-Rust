use crate::{DataTypeView, ScanResultViewData};
use slint_mvvm::view_data_converter::ViewDataConverter;
use squalr_engine_common::{structures::scan_results::scan_result::ScanResult, values::data_type::DataType};

pub struct ScanResultConverter;

impl ScanResultConverter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ViewDataConverter<ScanResult, ScanResultViewData> for ScanResultConverter {
    fn convert_collection(
        &self,
        scan_compare_type_list: &Vec<ScanResult>,
    ) -> Vec<ScanResultViewData> {
        return scan_compare_type_list
            .into_iter()
            .map(|item| self.convert_to_view_data(item))
            .collect();
    }

    fn convert_to_view_data(
        &self,
        scan_result: &ScanResult,
    ) -> ScanResultViewData {
        let address = scan_result.get_address();

        let address_string = if scan_result.is_module() {
            format!("{}+{:X}", scan_result.get_module(), scan_result.get_module_offset())
        } else if address <= u32::MAX as u64 {
            format!("{:08X}", address)
        } else {
            format!("{:016X}", address)
        };

        let data_type = match scan_result.get_data_type() {
            DataType::U8 { .. } => DataTypeView::U8,
            DataType::U16 { .. } => DataTypeView::U16,
            DataType::U32 { .. } => DataTypeView::U32,
            DataType::U64 { .. } => DataTypeView::U64,
            DataType::I8 { .. } => DataTypeView::I8,
            DataType::I16 { .. } => DataTypeView::I16,
            DataType::I32 { .. } => DataTypeView::I32,
            DataType::I64 { .. } => DataTypeView::I64,
            DataType::F32 { .. } => DataTypeView::F32,
            DataType::F64 { .. } => DataTypeView::F64,
            DataType::String { .. } => DataTypeView::String,
            DataType::Bytes { .. } => DataTypeView::Bytes,
            DataType::BitField { .. } => DataTypeView::Bitfield,
        };

        ScanResultViewData {
            address: address_string.into(),
            data_type,
            current_value: scan_result.get_current_value().to_value_string().into(),
            previous_value: scan_result.get_previous_value().to_value_string().into(),
        }
    }

    fn convert_from_view_data(
        &self,
        _scan_result_view_data: &ScanResultViewData,
    ) -> ScanResult {
        panic!("Not implemented.")
    }
}
