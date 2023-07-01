use crate::parser::{property_map::parse_property_map, string::parse_string};
use nom::{
  bytes::complete::take,
  combinator::{flat_map, map, map_parser},
  error::context,
  number::complete::le_i64 as i64,
  sequence::{terminated, tuple},
  IResult, Parser,
};

use super::{guid::parse_guid_property, Property};

pub fn parse_struct_property(input: &[u8]) -> IResult<&[u8], Property> {
  context(
    "struct",
    flat_map(
      terminated(parse_string, take(17usize)),
      |struct_type| -> &'static dyn Fn(&[u8]) -> IResult<&[u8], Property> {
        match struct_type {
          "Guid" => &parse_guid_property,
          "DateTime" => map(i64, |timestamp| Property::DateTime(timestamp)),
          _ => Box::new(map(parse_property_map, |value| Property::Struct {
            name: struct_type,
            value: Box::new(value),
          })),
        }
      },
    ),
  )(input)
}
