use nom::{combinator::map_res, multi::length_data, number::complete::le_u32 as u32, IResult};
use std::str::from_utf8;

pub fn parse_string(input: &[u8]) -> IResult<&[u8], &str> {
  map_res(length_data(u32), from_utf8)(input)
}
