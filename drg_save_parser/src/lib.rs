use gloo_utils::format::JsValueSerdeExt;
use gvas_save_parser::parse_save_data;
use js_sys::{Promise, Uint8Array};
use std::str;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, FileReader, ProgressEvent};
extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
  ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub async fn parse_save_file(file: File) -> Result<JsValue, JsValue> {
  console_error_panic_hook::set_once();

  // The FileReader API is callback-based, so we call it inside of a Promise
  // which we convert to a Rust future for usgae with async.
  let p = Promise::new(&mut |resolve, reject| {
    let Ok(reader) = FileReader::new() else {
      let _ = reject.call1(&JsValue::undefined(), &JsValue::from("Failed to create file reader"));
      return
    };
    reader.set_onloadend(Some(&resolve));
    let Ok(_) = reader.read_as_array_buffer(&file) else {
      let _ = reject.call1(&JsValue::undefined(), &JsValue::from("file not readable"));
      return
    };
  });

  // web_sys and js_sys give us these untyped "JSValue"s and functions that
  // return "Option" so we need a lot of casting and chaining to get the value
  // actually out of the fututre.
  // In JS this would look like `e.target().result()`
  let file_bytes = match JsCast::dyn_ref::<ProgressEvent>(&JsFuture::from(p).await?)
    .map(|progress_event| progress_event.target())
    .flatten()
    .as_deref()
    .map(|target| JsCast::dyn_ref::<FileReader>(target))
    .flatten()
    .map(|file_reader| file_reader.result())
    .map(|result| result.map(|v| Uint8Array::new(&v).to_vec()))
  {
    Some(e) => e,
    _ => Err(JsValue::from("Error getting data from file reader promise")),
  }?;

  match parse_save_data(&file_bytes) {
    Ok(s) => Ok(<JsValue as JsValueSerdeExt>::from_serde(&s).unwrap()),
    Err(e) => Err(JsValue::from(e.to_string())),
  }
}
