use squalr_engine_common::dynamic_struct::field_value::FieldValue;

pub struct ScanResult {
    address: u64,
    field_value: FieldValue,
}

impl ScanResult {
    pub fn new(
        address:u64,
        field_value: &FieldValue
    ) ->
    Self {
        Self {
            address: address,
            field_value: field_value.clone(),
        }
    }
}
