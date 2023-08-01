use std::fmt::Display;

use nom::error::{ContextError, ErrorKind, FromExternalError, ParseError};

#[derive(Debug)]
enum ParseErrorKind {
  Parser(ErrorKind),
  #[allow(dead_code)]
  External { parser: ErrorKind, message: String },
}

#[derive(Debug)]
pub struct SaveFileParseError<'a> {
  input: &'a [u8],
  context: Vec<&'static str>,
  error_stack: Vec<ParseErrorKind>,
}

impl<'a> ParseError<&'a [u8]> for SaveFileParseError<'a> {
  fn from_error_kind(input: &'a [u8], kind: ErrorKind) -> Self {
    SaveFileParseError {
      input,
      context: vec![],
      error_stack: vec![ParseErrorKind::Parser(kind)],
    }
  }

  fn append(_input: &[u8], kind: ErrorKind, mut other: Self) -> Self {
    if !other.context.is_empty() {
      other.error_stack.push(ParseErrorKind::Parser(kind))
    }
    other
  }
}

impl<'a> ContextError<&'a [u8]> for SaveFileParseError<'a> {
  fn add_context(_input: &[u8], ctx: &'static str, mut other: Self) -> Self {
    other.context.push(ctx);
    other
  }
}

impl<'a, T: Display> FromExternalError<&'a [u8], T> for SaveFileParseError<'a> {
  fn from_external_error(input: &'a [u8], kind: ErrorKind, e: T) -> Self {
    SaveFileParseError {
      input,
      context: vec![],
      error_stack: vec![ParseErrorKind::External {
        parser: kind,
        message: format!("{}", e),
      }],
    }
  }
}

impl<'a> Display for SaveFileParseError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Context\n\t{}\nError Stack\n\t{:?}\nNext 16 Bytes\n\t{}\n",
      self
        .context
        .clone()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(" -> "),
      self.error_stack,
      String::from_utf8_lossy(&self.input[..16])
    )
  }
}
