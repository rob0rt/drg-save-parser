use super::{parse_property_map, Property};
use crate::parser::{
  primitives::{parse_guid, parse_string},
  properties::StructPropertyValue,
};
use nom::{
  bytes::complete::take,
  combinator::map,
  error::{context, ContextError, FromExternalError, ParseError},
  number::complete::le_i64 as i64,
  IResult,
};
use std::string::{FromUtf16Error, FromUtf8Error};

pub fn parse_struct_property<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>
    + FromExternalError<&'a [u8], String>,
>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context("struct property", |input| {
    let (input, struct_type) = parse_string(input)?;
    let (input, _) = take(17u8)(input)?;
    let (input, struct_value) = parse_struct_value(struct_type)(input)?;
    Ok((input, Property::Struct(struct_value)))
  })(input)
}

pub fn parse_struct_value<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>
    + FromExternalError<&'a [u8], String>,
>(
  struct_type: String,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], StructPropertyValue, E> {
  move |input| match struct_type.as_str() {
    "Guid" => map(parse_guid, StructPropertyValue::Guid)(input),
    "DateTime" => map(i64, StructPropertyValue::DateTime)(input),
    _ => map(parse_property_map, |value| StructPropertyValue::Foreign {
      name: struct_type.to_owned(),
      value,
    })(input),
  }
}
