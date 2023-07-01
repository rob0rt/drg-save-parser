use super::Property;
use nom::{number::complete::le_i16 as i16, IResult};

pub fn parse_bool_property(input: &[u8]) -> IResult<&[u8], Property> {
  let (input, i) = i16(input)?;
  Ok((input, Property::Bool(i != 0)))
}
