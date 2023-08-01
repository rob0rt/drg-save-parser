use super::{r#struct::parse_struct_value, ArrayPropertyValue, Property};
use crate::parser::primitives::parse_string;
use nom::{
  bytes::complete::take,
  combinator::{map, map_res},
  error::{context, ContextError, ErrorKind, FromExternalError, ParseError},
  multi::count,
  number::complete::le_i32 as i32,
  number::complete::le_u32 as u32,
  number::complete::le_u64 as u64,
  Err, IResult, ToUsize,
};
use std::string::{FromUtf16Error, FromUtf8Error};

pub fn parse_array_property<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>
    + FromExternalError<&'a [u8], String>,
>(
  input: &'a [u8],
) -> IResult<&'a [u8], Property, E> {
  context("array property", |input| {
    let (input, property_type) = parse_string(input)?;
    let (input, _) = take(1u8)(input)?;
    let (input, property_count) = u32(input)?;

    let (input, values) = match property_type.as_str() {
      "StructProperty" => parse_struct_array(property_count)(input)?,
      "IntProperty" => parse_int_array(property_count)(input)?,
      "ObjectProperty" => parse_object_array(property_count)(input)?,
      e => {
        return Err(Err::Error(E::from_external_error(
          input,
          ErrorKind::Fail,
          format!("Unhandled array property type {}", e),
        )))
      }
    };
    Ok((input, Property::Array(values)))
  })(input)
}

fn parse_struct_array<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>
    + FromExternalError<&'a [u8], String>,
>(
  property_count: u32,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<ArrayPropertyValue>, E> {
  move |input| {
    context("struct array", |input| {
      let (input, _name) = parse_string(input)?;
      let (input, _property_type) =
        map_res(parse_string, |property_type| match property_type.as_str() {
          "StructProperty" => Ok(property_type),
          p => Err(format!("Unhandled struct array property type {}", p)),
        })(input)?;
      let (input, _length) = u64(input)?;

      let (input, struct_inner_property_type) = parse_string(input)?;
      let (input, _) = take(17u8)(input)?;

      count(
        map(
          parse_struct_value(struct_inner_property_type),
          ArrayPropertyValue::Struct,
        ),
        property_count.to_usize(),
      )(input)
    })(input)
  }
}

fn parse_int_array<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  property_count: u32,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<ArrayPropertyValue>, E> {
  move |input| {
    context("int array", |input| {
      count(map(i32, ArrayPropertyValue::Int), property_count.to_usize())(input)
    })(input)
  }
}

fn parse_object_array<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>,
>(
  property_count: u32,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<ArrayPropertyValue>, E> {
  move |input| {
    context(
      "object array",
      count(
        map(
           parse_string,
          ArrayPropertyValue::Object,
        ),
        property_count.to_usize(),
      ),
    )(input)
  }
}
