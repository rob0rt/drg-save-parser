use std::collections::HashMap;
use nom::{IResult, number::complete::le_i32 as i32, error::context, sequence::tuple};
use super::{guid::{Guid, parse_guid}, engine_version::{EngineVersion, parse_engine_version}, string::parse_string};

#[derive(Debug)]
pub struct Metadata<'a> {
  save_version: i32,
  package_version: i32,
  engine_version: EngineVersion<'a>,
  custom_format_version: i32,
  custom_format_data: HashMap<Guid, i32>,
  save_game_type: &'a str,
}

fn parse_custom_format_data(input: &[u8]) -> IResult<&[u8], HashMap<Guid, i32>> {
  let (input, length) = i32(input)?;
  let mut custom_format_data = HashMap::new();
  for _ in 0..length {
    let (input, key) = parse_guid(input)?;
    let (input, value) = i32(input)?;
    custom_format_data.insert(key, value);
  }
  Ok((input, custom_format_data))
}

pub fn parse_metadata(input: &[u8]) -> IResult<&[u8], Metadata> {
  let (
    input,
    (
      save_version,
      package_version,
      engine_version,
      custom_format_version,
      custom_format_data,
      save_game_type,
    ),
  ) = context(
    "metadata",
    tuple((
      i32,
      i32,
      parse_engine_version,
      i32,
      parse_custom_format_data,
      parse_string,
    )),
  )(input)?;
  Ok((
    input,
    Metadata {
      save_version,
      package_version,
      engine_version,
      custom_format_version,
      custom_format_data,
      save_game_type,
    },
  ))
}
