mod array;
mod map;
mod set;
mod r#struct;

use array::parse_array_property;
use map::parse_map_property;
use r#struct::parse_struct_property;
use serde::{ser::SerializeMap, Serialize, Serializer};
use set::parse_set_property;

use super::primitives::{parse_bool, parse_string, Guid};
use nom::{
  bytes::complete::take,
  combinator::{cut, fail, map, verify},
  error::{context, ContextError, FromExternalError, ParseError},
  multi::many_till,
  number::complete::{le_f32 as f32, le_i32 as i32, le_u32 as u32},
  sequence::{preceded, tuple},
  IResult,
};
use std::{
  collections::{HashMap, HashSet},
  string::{FromUtf16Error, FromUtf8Error},
};

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum MapPropertyKey {
  Struct(Guid),
  Int(i32),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum MapPropertyValue {
  Struct(HashMap<String, Property>),
  Int(i32),
  Float(f32),
  Bool(bool),
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum SetPropertyValue {
  Struct(Guid),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ArrayPropertyValue {
  Int(i32),
  Object(String),
  Struct(StructPropertyValue),
}

#[derive(Debug)]
pub enum StructPropertyValue {
  DateTime(i64),
  Guid(Guid),
  Foreign {
    name: String,
    value: HashMap<String, Property>,
  },
}

impl Serialize for StructPropertyValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      StructPropertyValue::DateTime(v) => serializer.serialize_i64(*v),
      StructPropertyValue::Guid(v) => v.serialize(serializer),
      StructPropertyValue::Foreign { name, value } => {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(name, value)?;
        map.end()
      }
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Property {
  Int(i32),
  UInt32(u32),
  Bool(bool),
  Struct(StructPropertyValue),
  Array(Vec<ArrayPropertyValue>),
  Float(f32),
  MulticastInlineDelegate {
    object_path: String,
    function_name: String,
  },
  Str(String),
  Map(HashMap<MapPropertyKey, MapPropertyValue>),
  Set(HashSet<SetPropertyValue>),
  Object(String),
  Enum {
    name: String,
    value: String,
  },
  Name(String),
}

pub fn parse_property_map<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>
    + FromExternalError<&'a [u8], String>,
>(
  input: &'a [u8],
) -> IResult<&[u8], HashMap<String, Property>, E> {
  context(
    "property map",
    map(
      many_till(
        context("property", |input| {
          let (input, (property_name, property_type, _)) =
            tuple((parse_string, parse_string, take(8usize)))(input)?;
          let (input, property_value) = parse_property_value(property_type)(input)?;
          Ok((input, (property_name, property_value)))
        }),
        verify(cut(parse_string), |s: &str| s == "None"),
      ),
      |(v, _)| v.into_iter().collect(),
    ),
  )(input)
}

fn parse_property_value<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>
    + FromExternalError<&'a [u8], String>,
>(
  property_type: String,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Property, E> {
  move |input| match property_type.as_str() {
    "IntProperty" => parse_int_property,
    "UInt32Property" => parse_uint32_property,
    "BoolProperty" => parse_bool_property,
    "StructProperty" => parse_struct_property,
    "ArrayProperty" => parse_array_property,
    "FloatProperty" => parse_float_property,
    "StrProperty" => parse_str_property,
    "MulticastInlineDelegateProperty" => parse_multicast_inline_delegate_property,
    "MapProperty" => parse_map_property,
    "SetProperty" => parse_set_property,
    "ObjectProperty" => parse_object_property,
    "EnumProperty" => parse_enum_property,
    "NameProperty" => parse_name_property,
    _ => fail,
  }(input)
}

fn parse_object_property<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>,
>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context(
    "object property",
    map(preceded(take(1usize), parse_string), Property::Object),
  )(input)
}

fn parse_multicast_inline_delegate_property<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>,
>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context("multicast inline delegate property", |input| {
    // There's 5 bytes of something at the top of this property. From what I've
    // gathered, it may be a bitmask of flags, or other alignment options.
    // Regardless, it doesn't seem to be important to us, so we'll disregard
    // that entire region of the save file
    let (input, _) = take(5u8)(input)?;

    map(
      tuple((parse_string, parse_string)),
      |(object_path, function_name)| Property::MulticastInlineDelegate {
        object_path,
        function_name,
      },
    )(input)
  })(input)
}

fn parse_int_property<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context("int property", map(preceded(take(1u8), i32), Property::Int))(input)
}

fn parse_uint32_property<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context(
    "uint32 property",
    map(preceded(take(1u8), u32), Property::UInt32),
  )(input)
}

fn parse_bool_property<
  'a,
  E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + FromExternalError<&'a [u8], String>,
>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context(
    "bool property",
    map(preceded(take(1u8), parse_bool), Property::Bool),
  )(input)
}

fn parse_float_property<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context(
    "float property",
    map(preceded(take(1usize), f32), Property::Float),
  )(input)
}

fn parse_str_property<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>,
>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context(
    "str property",
    map(preceded(take(1usize), parse_string), Property::Str),
  )(input)
}

fn parse_enum_property<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>,
>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context("enum property", |input| {
    let (input, name) = parse_string(input)?;
    let (input, _) = take(1usize)(input)?;
    let (input, value) = parse_string(input)?;

    Ok((input, Property::Enum { name, value }))
  })(input)
}

fn parse_name_property<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>,
>(
  input: &'a [u8],
) -> IResult<&[u8], Property, E> {
  context(
    "name property",
    map(preceded(take(1usize), parse_string), Property::Name),
  )(input)
}
