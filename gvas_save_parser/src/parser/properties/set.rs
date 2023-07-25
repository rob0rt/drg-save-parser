use super::{Property, SetPropertyValue};
use crate::parser::primitives::{parse_guid, parse_string};
use nom::{
  bytes::complete::take,
  combinator::map,
  error::{context, ContextError, ErrorKind, FromExternalError, ParseError},
  multi::count,
  number::complete::le_u32 as u32,
  Err, IResult, ToUsize,
};
use std::string::{FromUtf16Error, FromUtf8Error};

pub fn parse_set_property<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>
    + FromExternalError<&'a [u8], String>,
>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context("set property", |input| {
    let (input, property_type) = parse_string(input)?;
    let (input, _) = take(5u8)(input)?;
    let (input, num_properties) = u32(input)?;

    map(
      count(
        move |input| match property_type.as_str() {
          "StructProperty" => map(parse_guid, SetPropertyValue::Struct)(input),
          e => Err(Err::Error(E::from_external_error(
            input,
            ErrorKind::Fail,
            format!("Unhandled map property value type {}", e),
          ))),
        },
        num_properties.to_usize(),
      ),
      |v| Property::Set(v.into_iter().collect()),
    )(input)
  })(input)
}
