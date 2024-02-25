use chrono::prelude::*;
use neon::prelude::*;


pub fn print_log_entry(uuid: &str, msrv: &str, comment: &str, payload: &serde_json::Value) {
  let current_time = Local::now();
  let formatted_time = current_time.format("%Y-%m-%d %H:%M").to_string();
  
  println!("[\x1b[1m{}\x1b[0m]: \x1b[0;32m{}\x1b[0m (\x1b[1;34m{}\x1b[0m) - {} => {}", formatted_time, uuid, msrv, comment, payload);
}

pub fn print_log_entry_js(cx: FunctionContext, uuid_js: JsString, msrv_js: JsString, payload_js: JsObject) {
  // let uuid = uuid_js.
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // cx.export_function("hello", hello)?;
    cx.export_function("print_log_entry_js", print_log_entry_js)?;
    Ok(())
}