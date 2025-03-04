use squalr_engine_common::structures::{
    data_types::data_type::DataType,
    data_values::{anonymous_value::AnonymousValue, data_value::DataValue},
    scanning::scan_compare_type::ScanCompareType,
};

#[derive(Debug, Clone)]
pub struct ScanParameters {
    compare_type: ScanCompareType,
    compare_immediate: Option<AnonymousValue>,
}

impl ScanParameters {
    pub fn new(
        compare_type: ScanCompareType,
        value: Option<AnonymousValue>,
    ) -> Self {
        Self {
            compare_type,
            compare_immediate: value,
        }
    }

    pub fn get_compare_type(&self) -> ScanCompareType {
        self.compare_type.clone()
    }

    pub fn deanonymize_type(
        &self,
        data_type: &Box<dyn DataType>,
    ) -> Box<dyn DataValue> {
        self.compare_immediate
            .as_ref()
            .and_then(|value| value.deanonymize_type(data_type).ok())
            .unwrap_or_else(|| panic!("Invalid type"))
    }

    pub fn get_compare_immediate(&self) -> Option<&AnonymousValue> {
        match self.get_compare_type() {
            ScanCompareType::Immediate(_) => self.compare_immediate.as_ref(),
            ScanCompareType::Relative(_) => None,
            ScanCompareType::Delta(_) => None,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self.get_compare_type() {
            ScanCompareType::Immediate(_) => self.compare_immediate.is_some(),
            ScanCompareType::Relative(_) => true,
            ScanCompareType::Delta(_) => true,
        }
    }

    pub fn clone(&self) -> Self {
        ScanParameters {
            compare_type: self.compare_type.clone(),
            compare_immediate: self.compare_immediate.clone(),
        }
    }
}
