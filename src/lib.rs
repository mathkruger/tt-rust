use std::env;
use std::fs;
use json::JsonValue;
use utils::TimeRegister;

use crate::report::*;
use crate::register::*;
use crate::utils::application_banner;

mod report;
mod register;
mod utils;

pub fn run(mode: &str) {    
    match mode {
        "report" => show_report(),
        "mark" => mark_time(false),
        "mark-and-report" => mark_time(true),

        "r" => show_report(),
        "m" => mark_time(false),
        "mr" => mark_time(true),

        _ => show_report()
    }
}

fn mark_time(show_report_after_save: bool) {
    let data = get_records();
    let updated_file = mark(data);
    
    match set_records(updated_file) {
        Ok(_) => {
            if !show_report_after_save {
                println!("Registers updated.");
            } else {
                show_report();
            }
        },
        Err(err) => println!("There was an error to write the file: {}", err.to_string())
    };
}

fn show_report() {
    application_banner();

    let data = get_records();
    get_report(data);
}

fn get_current_path() -> String {
    return match env::current_exe() {
        Err(_) => String::new(),
        Ok(mut path) => {
            path.pop();
            return path.display().to_string()
        }
    };
}

fn get_records() -> Vec<TimeRegister> {
    let file_path = get_current_path() + "/records.json";
    
    let contents = match fs::read_to_string(file_path) {
        Ok(value) => value,
        Err(_) => "[]".to_string()
    };

    let data = match json::parse(&contents) {
        Ok(value) => value,
        Err(_) => JsonValue::Array(Vec::new())
    };

    TimeRegister::to_time_register(&data)
}

fn set_records(value: Vec<TimeRegister>) -> Result<(), std::io::Error> {
    let file_string = json::stringify_pretty(TimeRegister::to_json_array(value), 2);
    let file_path = get_current_path() + "/records.json";
    
    return fs::write(file_path, file_string);
}