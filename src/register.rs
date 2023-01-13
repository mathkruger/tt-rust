use std::time::SystemTime;

use json::JsonValue;

pub fn mark(data: JsonValue) -> String {
    let last_register = data["data"].members().last().unwrap();
    let current_date = SystemTime::now();

    return json::stringify(data);
}