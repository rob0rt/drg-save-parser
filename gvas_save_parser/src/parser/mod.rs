mod error;
mod metadata;
mod primitives;
mod properties;

use nom::{
  bytes::complete::tag,
  combinator::map,
  error::{ContextError, FromExternalError, ParseError},
  sequence::{preceded, tuple},
  IResult,
};
use serde::Serialize;
use std::{
  collections::HashMap,
  string::{FromUtf16Error, FromUtf8Error},
};

pub use error::SaveFileParseError;
pub use metadata::*;
pub use properties::*;

#[derive(Debug, Serialize)]
pub struct SaveFile {
  pub metadata: Metadata,
  pub properties: HashMap<String, Property>,
}

pub fn root<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>
    + FromExternalError<&'a [u8], String>,
>(
  input: &'a [u8],
) -> IResult<&[u8], SaveFile, E> {
  map(
    preceded(tag(b"GVAS"), tuple((parse_metadata, parse_property_map))),
    |(metadata, properties)| SaveFile {
      metadata,
      properties,
    },
  )(input)
}
