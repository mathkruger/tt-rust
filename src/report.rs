use dateparser::parse;
use json::{self, JsonValue};

pub fn get_report(data: &JsonValue) {
    for item in data["data"].members() {
        let worked_seconds = get_worked_hours_from_day(
            item["startTime"].as_str().expect("Not a string"),
            item["lunchStartTime"].as_str().expect("Not a string"),
            item["lunchEndTime"].as_str().expect("Not a string"),
            item["endTime"].as_str().expect("Not a string")
        );

        let total_minutes = worked_seconds / 60;

        let seconds = worked_seconds % 60;
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

        print!(
            "{}: {} - {} - {} - {} = {}:{}:{} \n",
            item["date"],
            item["startTime"],
            item["lunchStartTime"],
            item["lunchEndTime"],
            item["endTime"],
            hours,
            minutes,
            seconds
        );
    }
}

pub fn get_worked_hours_from_day(
    start_time: &str,
    lunch_start_time: &str,
    lunch_end_time: &str,
    end_time: &str,
) -> i64 {
    let format_error = "Time malformated";
    let start_time_parsed = parse(start_time).expect(format_error);
    let lunch_start_time_parsed = parse(lunch_start_time).expect(format_error);
    let lunch_end_time_parsed = parse(lunch_end_time).expect(format_error);
    let end_time_parsed = parse(end_time).expect(format_error);

    let worked_without_lunch = end_time_parsed.time() - start_time_parsed.time();
    let lunch_duration = lunch_end_time_parsed.time() - lunch_start_time_parsed.time();

    return worked_without_lunch.num_seconds() - lunch_duration.num_seconds();
}