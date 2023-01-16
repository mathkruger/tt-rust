use chrono::{DateTime, Datelike, Local, Timelike};
use dateparser::parse;
use json::{object};

use crate::utils::{format_date, format_time, TimeRegister};

pub fn mark(mut data: Vec<TimeRegister>) -> Vec<TimeRegister> {
    let current_time = Local::now();

    let default_register: TimeRegister = TimeRegister::new(&object! {
        "date": format_date(
            current_time.year() as u32,
            current_time.month(), 
            current_time.day() - 1
        )
    });

    let last_register = match data.last() {
        Some(value) => TimeRegister {
            date: value.date.clone(),
            start_time: value.start_time.clone(),
            lunch_start_time: value.lunch_start_time.clone(),
            lunch_end_time: value.lunch_end_time.clone(),
            end_time: value.end_time.clone()
        },
        None => default_register
    };

    let last_date_on_file = parse(&last_register.date).unwrap();

    if last_date_on_file.day() != current_time.day()
        || last_date_on_file.month() != current_time.month()
        || last_date_on_file.year() != current_time.year()
    {
        data = handle_new_day(data, &current_time);
    } else {
        data = handle_same_day(data, last_register, &current_time);
    }

    data
}

fn handle_new_day(mut data: Vec<TimeRegister>, current_time: &DateTime<Local>) -> Vec<TimeRegister> {
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

    let new_day = TimeRegister::new(&object!{
        "date": formated_date,
        "startTime": formated_time,
        "lunchStartTime": "",
        "lunchEndTime": "",
        "endTime": "",
    });

    data.push(new_day);
    data
}

fn handle_same_day(
    mut data: Vec<TimeRegister>,
    mut last_register: TimeRegister,
    current_time: &DateTime<Local>,
) -> Vec<TimeRegister> {
    let formated_time = format_time(
        current_time.hour(), 
        current_time.minute(),
        0
    );

    let mut save = true;

    if last_register.lunch_start_time == "-" {
        println!("Logando inicio da hora de almoço...");
        last_register.lunch_start_time = formated_time;
    }
    else if last_register.lunch_end_time == "-" {
        println!("Logando fim da hora de almoço...");
        last_register.lunch_end_time = formated_time;
    }
    else if last_register.end_time == "-" {
        println!("Lili cantou!!! Logando fim do expediente!");
        last_register.end_time = formated_time;
    }
    else {
        println!("Você já parou de trabalhar, amigo. Vai jogar alguma coisa!");
        save = false;
    }
    
    if save {
        let index = data.len() - 1;
        data[index as usize] = last_register;

        data
    } else {
        data
    }
}
