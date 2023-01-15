use chrono::{DateTime, Utc};
use dateparser::parse;
use json::{self, JsonValue};
use owo_colors::{OwoColorize, Style};

use crate::utils::format_time;

pub fn get_report(data: &JsonValue) {
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

    for item in data.members() {
        let worked_seconds = get_worked_hours_from_day(
            match item["startTime"].as_str() {
                Some(value) => value,
                None => ""
            },
            match item["lunchStartTime"].as_str() {
                Some(value) => value,
                None => ""
            },
            match item["lunchEndTime"].as_str() {
                Some(value) => value,
                None => ""
            },
            match item["endTime"].as_str() {
                Some(value) => value,
                None => ""
            },
        );

        let day_hour_bank = worked_seconds - 28800;
        let (hours, minutes, seconds) = seconds_to_hour(worked_seconds.abs());
        let (hour_bank_formatted, hour_bank_color) = get_formatted_bank_time(day_hour_bank);

        println!(
            "{0: <10} | {1: <10} | {2: <15} | {3: <15} | {4: <10} | {5: <18} | {6: <10}",
            item["date"],
            if item["startTime"].is_empty() {
                "00:00:00"
            } else {
                item["startTime"].as_str().unwrap()
            },
            if item["lunchStartTime"].is_empty() {
                "00:00:00"
            } else {
                item["lunchStartTime"].as_str().unwrap()
            },
            if item["lunchEndTime"].is_empty() {
                "00:00:00"
            } else {
                item["lunchEndTime"].as_str().unwrap()
            },
            if item["endTime"].is_empty() {
                "00:00:00"
            } else {
                item["endTime"].as_str().unwrap()
            },
            format_time(hours as u32, minutes as u32, seconds as u32),
            hour_bank_formatted.style(hour_bank_color),
        );

        total_hour_bank += day_hour_bank;
    }

    let separator = "-";

    let (total_hour_bank_formatted, style) = get_formatted_bank_time(total_hour_bank);

    println!(
        "{0: <72}---{1: <18} | {2: <10}",
        separator.repeat(72),
        "-------Saldo total",
        total_hour_bank_formatted.style(style)
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
    let seconds = raw_seconds % 60;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    (hours, minutes, seconds)
}

fn get_formatted_bank_time(raw_seconds: i64) -> (std::string::String, Style) {
    let (bank_hours, bank_minutes, bank_seconds) = seconds_to_hour(raw_seconds.abs());

    let hour_bank_type: &str = if raw_seconds < 0 {
        "-"
    } else {
        " "
    };

    let hour_bank_formatted = hour_bank_type.to_owned() +
        &format_time(bank_hours as u32, bank_minutes as u32, bank_seconds as u32);

    let hour_bank_color = if hour_bank_type == "-" {
        Style::new().red().bold()
    } else {
        Style::new().green().bold()
    };

    (hour_bank_formatted, hour_bank_color)
}
