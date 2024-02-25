use chrono::prelude::*;

// #[no_mangle]
pub fn print_log_entry(uuid: &str, msrv: &str, comment: &str, payload: &serde_json::Value) {
  let current_time = Local::now();
  let formatted_time = current_time.format("%Y-%m-%d %H:%M").to_string();
  
  println!("[\x1b[1m{}\x1b[0m]: \x1b[0;32m{}\x1b[0m (\x1b[1;34m{}\x1b[0m) - {} => {}", formatted_time, uuid, msrv, comment, payload);
}