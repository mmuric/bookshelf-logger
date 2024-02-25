mod logger;
use logger::print_log_entry;

fn main() {
    // Example usage
    let json_payload = serde_json::json!({"key": "value"});
    print_log_entry("1234", "books", "some comment", &json_payload);
    print_log_entry("1234", "order", "some other comment", &json_payload);
}