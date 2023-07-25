use super::primitives::{parse_guid, parse_string, Guid};
use nom::{
  combinator::map,
  error::{context, ContextError, FromExternalError, ParseError},
  multi::length_count,
  number::complete::{le_u16 as u16, le_u32 as u32},
  sequence::tuple,
  IResult,
};
use serde::Serialize;
use std::{
  collections::HashMap,
  string::{FromUtf16Error, FromUtf8Error},
};

#[derive(Debug, Serialize)]
pub struct Metadata {
  pub save_version: u32,
  pub package_version: u32,
  pub engine_major: u16,
  pub engine_minor: u16,
  pub engine_patch: u16,
  pub engine_build: u32,
  pub engine_build_id: String,
  pub custom_format_version: u32,
  pub custom_format_data: HashMap<Guid, u32>,
  pub save_game_type: String,
}

pub fn parse_metadata<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>,
>(
  input: &'a [u8],
) -> IResult<&'a [u8], Metadata, E> {
  context(
    "metadata",
    map(
      tuple((
        u32,
        u32,
        u16,
        u16,
        u16,
        u32,
        parse_string,
        u32,
        parse_custom_format_data,
        parse_string,
      )),
      |(
        save_version,
        package_version,
        engine_major,
        engine_minor,
        engine_patch,
        engine_build,
        engine_build_id,
        custom_format_version,
        custom_format_data,
        save_game_type,
      )| Metadata {
        save_version,
        package_version,
        engine_major,
        engine_minor,
        engine_patch,
        engine_build,
        engine_build_id,
        custom_format_version,
        custom_format_data,
        save_game_type,
      },
    ),
  )(input)
}

fn parse_custom_format_data<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  input: &'a [u8],
) -> IResult<&[u8], HashMap<Guid, u32>, E> {
  context(
    "custom format data",
    map(length_count(u32, tuple((parse_guid, u32))), |v| {
      v.into_iter().collect()
    }),
  )(input)
}
