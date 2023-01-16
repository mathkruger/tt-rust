use chrono::{DateTime, Utc};
use dateparser::parse;
use owo_colors::{OwoColorize, Style};

use crate::utils::{format_time, TimeRegister};

pub fn get_report(data: Vec<TimeRegister>) {
    if data.len() == 0 {
        println!("Sem registros para reportar, aponte algum horário.");
        return;
    }
    
    println!(
        "{0: <10} | {1: <10} | {2: <15} | {3: <15} | {4: <10} | {5: <18} | {6: <10}",
        "Data".bold(),
        "Início".bold(),
        "Início - almoço".bold(),
        "Fim - almoço".bold(),
        "Fim".bold(),
        "Horas trabalhadas".bold(),
        "Saldo".bold()
    );

    let mut total_hour_bank = 0;

    for item in data {
        let worked_seconds = get_worked_hours_from_day(
            &item.start_time,
            &item.lunch_start_time,
            &item.lunch_end_time,
            &item.end_time
        );

        let day_hour_bank = worked_seconds - 28800;
        let (hours, minutes, seconds) = seconds_to_hour(worked_seconds.abs());
        let (hour_bank_formatted, hour_bank_color) = get_formatted_bank_time(day_hour_bank);

        println!(
            "{0: <10} | {1: <10} | {2: <15} | {3: <15} | {4: <10} | {5: <18} | {6: <10}",
            item.date,
            item.start_time,
            item.lunch_start_time,
            item.lunch_end_time,
            item.end_time,
            format_time(hours as u32, minutes as u32, seconds as u32),
            hour_bank_formatted.style(hour_bank_color),
        );

        total_hour_bank += day_hour_bank;
    }

    let (total_hour_bank_formatted, style) = get_formatted_bank_time(total_hour_bank);
    
    println!(
        "{0: <72}---{1: <18} | {2: <10}",
        "-".repeat(72).bold(),
        "------ Saldo total".bold(),
        total_hour_bank_formatted.style(style).bold()
    );
}

pub fn get_worked_hours_from_day(
    start_time: &str,
    lunch_start_time: &str,
    lunch_end_time: &str,
    end_time: &str,
) -> i64 {
    let start_time_parsed = get_time_or_last(start_time, &Utc::now());
    let lunch_start_time_parsed = get_time_or_last(lunch_start_time, &start_time_parsed);
    let lunch_end_time_parsed = get_time_or_last(lunch_end_time, &lunch_start_time_parsed);
    let end_time_parsed = get_time_or_last(end_time, &lunch_end_time_parsed);

    let worked_without_lunch = end_time_parsed.time() - start_time_parsed.time();
    let lunch_duration = lunch_end_time_parsed.time() - lunch_start_time_parsed.time();

    return worked_without_lunch.num_seconds() - lunch_duration.num_seconds()
}

fn get_time_or_last(time_to_parse: &str, last_time: &DateTime<Utc>) -> DateTime<Utc> {
    let parsed = match parse(time_to_parse) {
        Ok(value) => value,
        Err(_) => last_time.clone()
    };

    parsed
}

fn seconds_to_hour(raw_seconds: i64) -> (i64, i64, i64) {
    if raw_seconds == 0 {
        return (0, 0, 0)
    }

    let total_minutes = raw_seconds / 60;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    (hours, minutes, 0)
}

fn get_formatted_bank_time(raw_seconds: i64) -> (std::string::String, Style) {
    let (bank_hours, bank_minutes, bank_seconds) = seconds_to_hour(raw_seconds.abs());

    let hour_bank_type: &str = if raw_seconds < 0 {
        "-"
    } else {
        "+"
    };

    let hour_bank_formatted = hour_bank_type.to_owned() +
        &format_time(bank_hours as u32, bank_minutes as u32, bank_seconds as u32);

    let hour_bank_color = if hour_bank_type == "-" {
        Style::new().red()
    } else {
        Style::new().green()
    };

    (hour_bank_formatted, hour_bank_color)
}
