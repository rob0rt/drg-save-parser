mod parser;

use nom::Finish;
use parser::root;

use parser::{SaveFile, SaveFileParseError};

pub fn parse_save_data(input: &[u8]) -> Result<SaveFile, SaveFileParseError> {
  let (_, save_file) = root(input).finish()?;
  Ok(save_file)
}

#[cfg(test)]
mod tests {
  use std::{fs, path::Path};

  #[test]
  fn it_successfully_parses_save_1() {
    let sav = fs::read(Path::new(concat!(
      env!("CARGO_MANIFEST_DIR"),
      "/",
      "test",
      "/",
      "save_1.sav"
    )))
    .expect("Failed to read save file");

    assert!(matches!(crate::parse_save_data(&sav), Ok(_)));
  }

  #[test]
  fn it_successfully_parses_save_2() {
    let sav = fs::read(Path::new(concat!(
      env!("CARGO_MANIFEST_DIR"),
      "/",
      "test",
      "/",
      "save_2.sav"
    )))
    .expect("Failed to read save file");

    assert!(matches!(crate::parse_save_data(&sav), Ok(_)));
  }

  #[test]
  fn it_successfully_parses_save_3() {
    let sav = fs::read(Path::new(concat!(
      env!("CARGO_MANIFEST_DIR"),
      "/",
      "test",
      "/",
      "save_3.sav"
    )))
    .expect("Failed to read save file");

    assert!(matches!(crate::parse_save_data(&sav), Ok(_)));
  }
}
