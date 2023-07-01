use nom::{IResult, error::context, sequence::tuple, number::complete::le_u32 as u32};

#[derive(PartialEq, Hash, Eq, Debug)]
pub struct Guid(String);

pub fn parse_guid(input: &[u8]) -> IResult<&[u8], Guid> {
  let (input, (group1, group2, group3, group4)) =
    context("guid", tuple((u32, u32, u32, u32)))(input)?;
  Ok((
    input,
    Guid(format!("{group1:X}-{group2:X}-{group3:X}-{group4:X}")),
  ))
}
