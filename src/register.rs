use chrono::{DateTime, Datelike, Local, Timelike};
use dateparser::parse;
use json::{object, JsonValue};

use crate::utils::{format_date, format_time};

pub fn mark(mut data: JsonValue) -> String {
    let default_register = JsonValue::Array(Vec::new());

    let last_register = match data.members().last() {
        Some(value) => value,
        None => &default_register
    };

    let current_time = Local::now();
    let default_date_string = format_date(
        current_time.year() as u32,
        current_time.month(), 
        current_time.day() - 1
    );
    let last_date_on_file = parse(match last_register["date"].as_str() {
        Some(value) => value,
        None => &default_date_string
    }).unwrap();

    if last_date_on_file.day() != current_time.day()
        || last_date_on_file.month() != current_time.month()
        || last_date_on_file.year() != current_time.year()
    {
        data = handle_new_day(&data, &current_time);
    } else {
        data = handle_same_day(&data, &last_register, &current_time);
    }

    return json::stringify_pretty(data, 2);
}

fn handle_new_day(data: &JsonValue, current_time: &DateTime<Local>) -> JsonValue {
    let formated_date = format_date(
        current_time.year() as u32,
        current_time.month(),
        current_time.day()
    );

    let formated_time = format_time(
        current_time.hour(), 
        current_time.minute(),
        0
    );

    println!("Logando início do dia...");

    let new_day = object! {
        "date": formated_date,
        "startTime": formated_time,
        "lunchStartTime": "",
        "lunchEndTime": "",
        "endTime": "",
    };

    let mut data_to_save = data.clone();
    data_to_save.push(new_day).unwrap();

    data_to_save
}

fn handle_same_day(
    data: &JsonValue,
    last_register: &JsonValue,
    current_time: &DateTime<Local>,
) -> JsonValue {
    let formated_time = format_time(
        current_time.hour(), 
        current_time.minute(),
        0
    );

    let mut save = true;
    let mut register_to_modify = last_register.clone();

    if register_to_modify["lunchStartTime"].is_empty() {
        println!("Logando inicio da hora de almoço...");
        register_to_modify["lunchStartTime"] = json::JsonValue::String(formated_time);
    }
    else if register_to_modify["lunchEndTime"].is_empty() {
        println!("Logando fim da hora de almoço...");
        register_to_modify["lunchEndTime"] = json::JsonValue::String(formated_time);
    }
    else if register_to_modify["endTime"].is_empty() {
        println!("Lili cantou!!! Logando fim do expediente!");
        register_to_modify["endTime"] = json::JsonValue::String(formated_time);
    }
    else {
        println!("Você já parou de trabalhar, amigo. Vai jogar alguma coisa!");
        save = false;
    }
    
    if save {
        let mut index = 0;
        let mut data_to_save = data.clone();

        for entry in data_to_save.members_mut() {
            if entry["date"] == last_register["date"] {
                break;
            }
    
            index += 1;
        };

        data_to_save[index as usize] = register_to_modify;
        data_to_save
    } else {
        data.clone()
    }
}
