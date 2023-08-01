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
  fn it_successfully_parses_saves() {
    let save_files = fs::read_dir(Path::new(concat!(
      env!("CARGO_MANIFEST_DIR"),
      "/",
      "test",
      "/",
    )))
    .unwrap();

    for save_file in save_files {
      let save_file = save_file.unwrap();
      let save_file_path = save_file.path();
      let save_file_name = save_file_path.file_name().unwrap().to_str().unwrap();
      let save_file_data = fs::read(&save_file_path).expect("Failed to read save file");

      println!("Parsing save file {}", save_file_name);
      match crate::parse_save_data(&save_file_data) {
        Ok(_) => (),
        Err(e) => panic!("Failed to parse save file {};\n{}", save_file_name, e),
      }
    }
  }
}
