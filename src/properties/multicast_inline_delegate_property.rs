use super::Property;
use crate::utils::{error::ParseError, read_string::ReadString};
use serde::Serialize;
use std::io::{Cursor, Read};

#[derive(Debug, Serialize)]
pub struct MulticastInlineDelegateProperty {
  pub object_path: String,
  pub function_name: String,
}

impl MulticastInlineDelegateProperty {
  pub fn new(reader: &mut Cursor<Vec<u8>>) -> Result<Property, ParseError> {
    // There's 5 bytes of something at the top of this property. From what I've
    // gathered, it may be a bitmask of flags, or other alignment options.
    // Regardless, it doesn't seem to be important to us, so we'll disregard
    // that entire region of the save file
    reader.read_exact(&mut [0u8; 5])?;

    let object_path = reader.read_string()?;
    let function_name = reader.read_string()?;

    Ok(Property::from(MulticastInlineDelegateProperty {
      object_path,
      function_name,
    }))
  }
}
