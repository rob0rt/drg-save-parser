mod parser;

use nom::bytes::complete::tag;
use parser::metadata::{parse_metadata, Metadata};
use std::collections::HashMap;

struct SaveFile<'a> {
  metadata: Metadata<'a>,
  properties: HashMap<String, Property>,
}

pub fn parse_save_data(input: &[u8]) -> Result<SaveFile, ParseError> {
  let (input, _) = tag(b"GVAS")(input)?;
  let (input, metadata) = parse_metadata(input)?;

  // let mut cursor = Cursor::new(data.to_vec());
  // if validate_save_file_header(&mut cursor).is_err() {
  //   return Err(ParseError::new("Invalid save file".to_string()));
  // };
  // let _metadata = SaveFileMetadata::new(&mut cursor)?;

  // let mut properties = HashMap::new();
  // loop {
  //   if char::from_u32(peek(&mut cursor)?).is_none() {
  //     break;
  //   }
  //   let name = cursor.read_string()?;
  //   if name == "None" {
  //     break;
  //   }
  //   let data_type = cursor.read_string()?;
  //   let _length = cursor.read_i64::<LittleEndian>()?;
  //   let property = Property::new(data_type.as_str(), &mut cursor)?;
  //   properties.insert(name, property);
  // }

  // Ok(properties)
}
