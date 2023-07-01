use super::{
  properties::{
    bool::parse_bool_property, int::parse_int_property, r#struct::parse_struct_property, Property,
  },
  string::parse_string,
};
use nom::{
  bytes::{streaming::take, complete::tag},
  combinator::{fail, map_parser, flat_map},
  sequence::terminated,
  IResult, branch::alt, Parser,
};
use std::collections::HashMap;

pub fn parse_property_map(input: &[u8]) -> IResult<&[u8], HashMap<&str, Property>> {
  let mut properties = HashMap::new();
  loop {
    let (input, name) = parse_string(input)?;
    if name == "None" {
      break;
    }
    // if char::from_u32(peek(&mut cursor)?).is_none() {
    //   break;
    // }
    // let name = cursor.read_string()?;
    // if name == "None" {
    //   break;
    // }

    let (input, property) =
      flat_map(
        terminated(parse_string, take(8usize)),
        |property_type| match property_type {
          "IntProperty" | "UInt32Property" => parse_int_property,
          "BoolProperty" => parse_bool_property,
          "StructProperty" => parse_struct_property,
          // "ArrayProperty" => ArrayProperty::new(reader),
          // "FloatProperty" => FloatProperty::new(reader),
          // "SetProperty" => SetProperty::new(reader),
          // "StrProperty" => StringProperty::new(reader),
          // "EnumProperty" => EnumProperty::new(reader),
          // "MapProperty" => MapProperty::new(reader),
          // "ObjectProperty" => ObjectProperty::new(reader),
          // "MulticastInlineDelegateProperty" => MulticastInlineDelegateProperty::new(reader),
          _ => fail,
        },
      )(input)?;
    properties.insert(name, property);
  }

  Ok((input, properties))
}
