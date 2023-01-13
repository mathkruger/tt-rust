use chrono::{DateTime, Utc};
use dateparser::parse;
use json::{self, JsonValue};

use crate::utils::format_time;

pub fn get_report(data: &JsonValue) {
    if data.len() == 0 {
        println!("Sem registros para reportar, aponte algum horário.");
        return;
    }
    
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

        let total_minutes = worked_seconds / 60;

        let seconds = worked_seconds % 60;
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

        print!(
            "{}: {} - {} - {} - {} = {} \n",
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
        );
    }
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

    return worked_without_lunch.num_seconds() - lunch_duration.num_seconds();
}

fn get_time_or_last(time_to_parse: &str, last_time: &DateTime<Utc>) -> DateTime<Utc> {
    let parsed = match parse(time_to_parse) {
        Ok(value) => value,
        Err(_) => last_time.clone(),
    };

    parsed
}
