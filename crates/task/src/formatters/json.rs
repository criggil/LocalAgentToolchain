use serde::Serialize;

pub fn print_json<T: Serialize>(value: &T) {
    match serde_json::to_string_pretty(value) {
        Ok(json_str) => println!("{}", json_str),
        Err(e) => eprintln!("{{\"error\": \"Failed to serialize to JSON: {}\"}}", e),
    }
}
