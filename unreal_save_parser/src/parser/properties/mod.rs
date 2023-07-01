use std::collections::HashMap;

use super::guid::Guid;

pub mod bool;
pub mod int;
pub mod r#struct;
pub mod guid;

pub enum Property<'a> {
  Int(i32),
  Bool(bool),
  Struct {
    name: &'a str,
    value: Box<HashMap<&'a str, Property<'a>>>,
  },
  Guid(Guid),
  DateTime(i64),
}
