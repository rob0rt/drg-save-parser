use super::Property;
use nom::{bytes::complete::take, number::complete::le_i32 as i32, sequence::preceded, IResult};

pub fn parse_int_property(input: &[u8]) -> IResult<&[u8], Property> {
  let (input, i) = preceded(take(1usize), i32)(input)?;
  Ok((input, Property::Int(i)))
}
