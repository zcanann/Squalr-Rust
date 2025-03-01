use squalr_engine_common::structures::scan_compare_type::ScanCompareType;
use squalr_engine_common::structures::scan_filter_parameters::ScanFilterParameters;
use squalr_engine_common::values::anonymous_value::AnonymousValue;
use squalr_engine_common::values::data_type::DataType;
use squalr_engine_common::values::data_value::DataValue;

#[derive(Debug, Clone)]
pub struct ScanParameters {
    compare_type: ScanCompareType,
    compare_immediate: Option<AnonymousValue>,
    scan_filter_parameters: Vec<ScanFilterParameters>,
}

impl ScanParameters {
    pub fn new() -> Self {
        Self {
            compare_type: ScanCompareType::Changed,
            compare_immediate: None,
            scan_filter_parameters: vec![],
        }
    }

    pub fn new_with_value(
        compare_type: ScanCompareType,
        value: Option<AnonymousValue>,
    ) -> Self {
        Self {
            compare_type,
            compare_immediate: value,
            scan_filter_parameters: vec![],
        }
    }

    pub fn get_compare_type(&self) -> ScanCompareType {
        self.compare_type.clone()
    }

    pub fn get_scan_filter_parameters(&self) -> &Vec<ScanFilterParameters> {
        &self.scan_filter_parameters
    }

    pub fn deanonymize_type(
        &self,
        data_type: &DataType,
    ) -> DataValue {
        self.compare_immediate
            .as_ref()
            .and_then(|value| value.deanonymize_type(data_type).ok())
            .unwrap_or_else(|| panic!("Invalid type"))
    }

    pub fn get_compare_immediate(&self) -> Option<&AnonymousValue> {
        if self.is_immediate_comparison() {
            self.compare_immediate.as_ref()
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_immediate_comparison() {
            true
        } else {
            self.compare_immediate.is_some()
        }
    }

    pub fn is_relative_delta_comparison(&self) -> bool {
        match self.compare_type {
            ScanCompareType::IncreasedByX | ScanCompareType::DecreasedByX => true,
            _ => false,
        }
    }

    pub fn is_relative_comparison(&self) -> bool {
        match self.compare_type {
            ScanCompareType::Changed | ScanCompareType::Unchanged | ScanCompareType::Increased | ScanCompareType::Decreased => true,
            _ => false,
        }
    }

    pub fn is_immediate_comparison(&self) -> bool {
        match self.compare_type {
            ScanCompareType::Equal
            | ScanCompareType::NotEqual
            | ScanCompareType::GreaterThan
            | ScanCompareType::GreaterThanOrEqual
            | ScanCompareType::LessThan
            | ScanCompareType::LessThanOrEqual
            | ScanCompareType::IncreasedByX
            | ScanCompareType::DecreasedByX => true,
            _ => false,
        }
    }

    pub fn clone(&self) -> Self {
        ScanParameters {
            compare_type: self.compare_type.clone(),
            compare_immediate: self.compare_immediate.clone(),
            scan_filter_parameters: self.scan_filter_parameters.clone(),
        }
    }

    pub fn conflicts_with(
        &self,
        other: &ScanParameters,
    ) -> bool {
        if self.compare_type == other.compare_type {
            true
        } else if !self.is_immediate_comparison() && !other.is_immediate_comparison() {
            true
        } else if self.is_immediate_comparison() && other.is_immediate_comparison() {
            if (matches!(
                self.compare_type,
                ScanCompareType::LessThan | ScanCompareType::LessThanOrEqual | ScanCompareType::NotEqual
            ) && matches!(
                other.compare_type,
                ScanCompareType::GreaterThan | ScanCompareType::GreaterThanOrEqual | ScanCompareType::NotEqual
            )) || (matches!(
                self.compare_type,
                ScanCompareType::GreaterThan | ScanCompareType::GreaterThanOrEqual | ScanCompareType::NotEqual
            ) && matches!(
                other.compare_type,
                ScanCompareType::LessThan | ScanCompareType::LessThanOrEqual | ScanCompareType::NotEqual
            )) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
