use nom::{
  combinator::{flat_map, map, map_res, verify},
  error::{context, ContextError, FromExternalError, ParseError},
  multi::count,
  number::complete::{le_i32 as i32, le_u16 as u16, le_u32 as u32, le_u8 as u8},
  IResult,
};
use serde::Serialize;
use std::{
  ops::Range,
  string::{FromUtf16Error, FromUtf8Error},
};

/**
 * The maximum acceptable size of a string is 2^16. String lengths are negative
 * if the string is encoded as a wide string.
 */
const VALID_STRING_LENGTH: Range<i32> = -(1 << 16)..(1 << 16);

/**
 * Strings of length > 0 are laid out in the structure:
 *   [u32 | length][(u8 * (length - 1)) | data][u8 | \0]
 * And empty strings are in the format:
 *   [u32 | 0]
 *
 * Note the lack of null-terminator for strings of length 0, which appears to
 * be an optimization for file size.
 *
 * This will remove the trailing null-byte from the resulting string slice if
 * necessary
 */
pub fn parse_string<
  'a,
  E: ParseError<&'a [u8]>
    + ContextError<&'a [u8]>
    + FromExternalError<&'a [u8], FromUtf16Error>
    + FromExternalError<&'a [u8], FromUtf8Error>,
>(
  input: &'a [u8],
) -> IResult<&[u8], String, E> {
  context(
    "string",
    map(
      flat_map(verify(i32, |l| VALID_STRING_LENGTH.contains(l)), |l| {
        move |input| {
          if l < 0 {
            map_res(count(u16, -l as usize), |v| String::from_utf16(&v))(input)
          } else {
            map_res(count(u8, l as usize), |v| String::from_utf8(v))(input)
          }
        }
      }),
      |s| {
        if s.len() == 0 {
          // Avoid underflow when l == 0
          s
        } else {
          // ignore the last byte of the string (\0)
          s[..s.len() - 1].to_string()
        }
      },
    ),
  )(input)
}

#[derive(PartialEq, Hash, Eq, Debug, Serialize)]
pub struct Guid(Option<String>);

/**
 * GUIDs are 4 4-byte groups of hex values encoded in the save file as ints,
 * meaning that each individual group has little endian encoding. We therefore
 * read the value as those 4 ints to decode endian-ness before reconstructing
 * it into the standard string representation.
 *
 * Example parsed GUID
 *   4C4F1A50-42CB24CC-2A7F28B0-0D12AEF9
 *
 * Note that it's possible for a GUID value to be all 0s, which is a form of
 * "null" state, so we specifically look for that and map into "None".
 */
pub fn parse_guid<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  input: &'a [u8],
) -> IResult<&[u8], Guid, E> {
  context(
    "guid",
    map(count(u32, 4), |v| {
      if v.as_slice().into_iter().all(|i| *i == 0) {
        Guid(None)
      } else {
        Guid(Some(
          v.into_iter()
            .map(|i| format!("{i:08X}"))
            .collect::<Vec<_>>()
            .join("-"),
        ))
      }
    }),
  )(input)
}

/**
 * A boolean value is [uncharastically] wasteful and consume a whole byte,
 * despite never using more than the lowest bit to represnt the value.
 */
pub fn parse_bool<
  'a,
  E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + FromExternalError<&'a [u8], String>,
>(
  input: &'a [u8],
) -> IResult<&[u8], bool, E> {
  context(
    "bool",
    map_res(u8, |i| match i {
      0 => Ok(false),
      1 => Ok(true),
      n => Err(format!("Invalid boolean value {}", n)),
    }),
  )(input)
}
