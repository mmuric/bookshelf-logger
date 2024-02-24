mod logger;
use logger::print_log_entry;

fn main() {
    // Example usage
    let json_payload = serde_json::json!({"key": "value"});
    print_log_entry("INFO", "Processing data", &json_payload);
}