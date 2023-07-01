mod array_property;
mod bool_property;
mod enum_property;
mod float_property;
mod guid_property;
mod int_property;
mod map_property;
mod multicast_inline_delegate_property;
mod object_property;
mod set_property;
mod string_property;
mod struct_property;

use crate::utils::error::ParseError;
use enum_dispatch::enum_dispatch;
use serde::Serialize;
use std::io::Cursor;

use array_property::ArrayProperty;
use bool_property::BoolProperty;
use enum_property::EnumProperty;
use float_property::FloatProperty;
use guid_property::GuidProperty;
use int_property::IntProperty;
use map_property::MapProperty;
use multicast_inline_delegate_property::MulticastInlineDelegateProperty;
use object_property::ObjectProperty;
use set_property::SetProperty;
use string_property::StringProperty;
use struct_property::StructProperty;

#[enum_dispatch]
trait PropertyTrait {}

#[enum_dispatch(PropertyTrait)]
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Property {
  StructProperty,
  IntProperty,
  BoolProperty,
  ArrayProperty,
  GuidProperty,
  FloatProperty,
  SetProperty,
  StringProperty,
  EnumProperty,
  MapProperty,
  ObjectProperty,
  MulticastInlineDelegateProperty,
}

impl Property {
  pub fn new(property_type: &str, reader: &mut Cursor<Vec<u8>>) -> Result<Self, ParseError> {
    match property_type {
      "IntProperty" | "UInt32Property" => IntProperty::new(reader),
      "BoolProperty" => BoolProperty::new(reader),
      "StructProperty" => StructProperty::new(reader),
      "ArrayProperty" => ArrayProperty::new(reader),
      "FloatProperty" => FloatProperty::new(reader),
      "SetProperty" => SetProperty::new(reader),
      "StrProperty" => StringProperty::new(reader),
      "EnumProperty" => EnumProperty::new(reader),
      "MapProperty" => MapProperty::new(reader),
      "ObjectProperty" => ObjectProperty::new(reader),
      "MulticastInlineDelegateProperty" => MulticastInlineDelegateProperty::new(reader),
      _ => {
        return Err(ParseError::new(format!(
          "Unhandled property data type {}",
          property_type
        )))
      }
    }
  }
}
