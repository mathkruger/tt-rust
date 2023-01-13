use std::fs;

use json::JsonValue;

use crate::report::*;
use crate::register::*;

mod report;
mod register;

pub fn run(mode: &str) {
    match mode {
        "report" => show_report(),
        "mark" => mark_time(),
        _ => println!("Command not found")
    }
}

fn mark_time() {
    let data = get_records();
    let updated_file = mark(data);
    set_records(&updated_file).expect("Error at writing the file again");
}

fn show_report() {
    let data = get_records();
    get_report(&data);
}

fn get_records() -> JsonValue {
    let file_path = "records.json";
    let contents = fs::read_to_string(file_path).expect("File not found!");
    let data = json::parse(&contents).expect("The json is invalid!");

    data
}

fn set_records(value: &str) -> Result<(), std::io::Error> {
    let file_path = "records.json";
    return fs::write(file_path, value);
}