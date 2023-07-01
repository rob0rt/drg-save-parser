use nom::{IResult, combinator::map};

use crate::parser::guid::parse_guid;

use super::Property;

pub fn parse_guid_property(input: &[u8]) -> IResult<&[u8], Property> {
  map(parse_guid, |guid| Property::Guid(guid))(input)
}
