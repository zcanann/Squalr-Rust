use crate::structures::data_types::built_in_types::primitive_data_type::PrimitiveDataType;
use crate::structures::data_types::data_type_error::DataTypeError;
use crate::structures::data_types::data_type_ref::DataTypeRef;
use crate::structures::data_values::anonymous_value_container::AnonymousValueContainer;
use crate::structures::data_values::display_value_type::DisplayValueType;
use crate::structures::data_values::display_values::DisplayValues;
use crate::structures::memory::endian::Endian;
use crate::structures::{data_types::data_type::DataType, data_values::data_value::DataValue};
use serde::{Deserialize, Serialize};

// For a bool8 the underlying primitive is a u8.
type ExposedType = bool;
type PrimitiveType = u8;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataTypeBool8 {}

impl DataTypeBool8 {
    pub const DATA_TYPE_ID: &str = "bool8";

    pub fn get_data_type_id() -> &'static str {
        Self::DATA_TYPE_ID
    }

    fn to_vec(value: ExposedType) -> Vec<u8> {
        (value as PrimitiveType).to_le_bytes().to_vec()
    }

    pub fn get_value_from_primitive(value: ExposedType) -> DataValue {
        let value_bytes = PrimitiveType::to_le_bytes(value as PrimitiveType);

        DataValue::new(DataTypeRef::new(Self::get_data_type_id()), value_bytes.to_vec())
    }
}

impl DataType for DataTypeBool8 {
    fn get_data_type_id(&self) -> &str {
        Self::get_data_type_id()
    }

    fn get_icon_id(&self) -> &str {
        "bool"
    }

    fn get_unit_size_in_bytes(&self) -> u64 {
        size_of::<PrimitiveType>() as u64
    }

    fn validate_value(
        &self,
        anonymous_value_container: &AnonymousValueContainer,
    ) -> bool {
        match PrimitiveDataType::deanonymize_bool::<PrimitiveType>(anonymous_value_container, false) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn deanonymize_value(
        &self,
        anonymous_value_container: &AnonymousValueContainer,
    ) -> Result<DataValue, DataTypeError> {
        let value_bytes = PrimitiveDataType::deanonymize_bool::<PrimitiveType>(anonymous_value_container, false)?;

        Ok(DataValue::new(DataTypeRef::new(Self::get_data_type_id()), value_bytes))
    }

    fn create_display_values(
        &self,
        value_bytes: &[u8],
    ) -> Result<DisplayValues, DataTypeError> {
        PrimitiveDataType::create_display_values_bool(value_bytes, self.get_unit_size_in_bytes())
    }

    fn get_supported_display_types(&self) -> Vec<DisplayValueType> {
        vec![DisplayValueType::Bool]
    }

    fn get_default_display_type(&self) -> DisplayValueType {
        DisplayValueType::Bool
    }

    fn get_endian(&self) -> Endian {
        Endian::Little
    }

    fn is_floating_point(&self) -> bool {
        false
    }

    fn is_signed(&self) -> bool {
        false
    }

    fn get_default_value(
        &self,
        data_type_ref: DataTypeRef,
    ) -> DataValue {
        DataValue::new(data_type_ref, Self::to_vec(false))
    }
}
