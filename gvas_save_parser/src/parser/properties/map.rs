use super::{parse_property_map, MapPropertyKey, MapPropertyValue, Property};
use crate::parser::primitives::{parse_guid, parse_string};
use nom::{
  bytes::complete::take,
  combinator::map,
  error::{context, ContextError, ErrorKind, FromExternalError, ParseError},
  multi::length_count,
  number::complete::{le_f32 as f32, le_i32 as i32, le_i8 as i8, le_u32 as u32},
  sequence::tuple,
  Err, IResult,
};
use std::string::{FromUtf16Error, FromUtf8Error};

pub fn parse_map_property<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>
    + FromExternalError<&'a [u8], String>,
>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context("map property", |input| {
    let (input, (key_type, value_type)) = tuple((parse_string, parse_string))(input)?;
    let (input, _) = take(5u8)(input)?;

    map(
      length_count(u32, move |input| {
        let (input, key) = context("map property key", |input| match key_type.as_str() {
          "StructProperty" => map(parse_guid, MapPropertyKey::Struct)(input),
          "IntProperty" => map(i32, MapPropertyKey::Int)(input),
          e => Err(Err::Error(E::from_external_error(
            input,
            ErrorKind::Fail,
            format!("Unhandled map property key type {}", e),
          ))),
        })(input)?;

        let (input, value) = context("map property value", |input| match value_type.as_str() {
          "StructProperty" => map(parse_property_map, MapPropertyValue::Struct)(input),
          "IntProperty" => map(i32, MapPropertyValue::Int)(input),
          "FloatProperty" => map(f32, MapPropertyValue::Float)(input),
          "BoolProperty" => map(parse_map_bool_property, MapPropertyValue::Bool)(input),
          e => Err(Err::Error(E::from_external_error(
            input,
            ErrorKind::Fail,
            format!("Unhandled map property value type {}", e),
          ))),
        })(input)?;

        Ok((input, (key, value)))
      }),
      |d| Property::Map(d.into_iter().collect()),
    )(input)
  })(input)
}

fn parse_map_bool_property<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  input: &'a [u8],
) -> IResult<&'a [u8], bool, E> {
  context("map bool property", map(i8, |i| i != 0))(input)
}
