use super::string::parse_string;
use nom::{
  error::context,
  number::complete::{le_u16 as u16, le_u32 as u32},
  sequence::tuple,
  IResult,
};

#[derive(Debug)]
pub struct EngineVersion<'a> {
  major: u16,
  minor: u16,
  patch: u16,
  build: u32,
  build_id: &'a str,
}

pub fn parse_engine_version(input: &[u8]) -> IResult<&[u8], EngineVersion> {
  let (input, (major, minor, patch, build, build_id)) =
    context("engine version", tuple((u16, u16, u16, u32, parse_string)))(input)?;
  Ok((
    input,
    EngineVersion {
      major,
      minor,
      patch,
      build,
      build_id,
    },
  ))
}
