use chrono::prelude::*;

#[no_mangle]
pub extern fn print_log_entry(t: &str, comment: &str, payload: &serde_json::Value) {
  let current_time = Local::now();
  let formatted_time = current_time.format("%Y-%m-%d %H:%M").to_string();
  
  println!("[\x1b[1m{}\x1b[0m]: \x1b[1;34m{}\x1b[0m - {} => {}", formatted_time, t, comment, payload);
}