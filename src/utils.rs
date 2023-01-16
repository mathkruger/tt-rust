use json::{JsonValue, object};
use owo_colors::OwoColorize;

pub struct TimeRegister {
    pub date: String,
    pub start_time: String,
    pub lunch_start_time: String,
    pub lunch_end_time: String,
    pub end_time: String
}

impl TimeRegister {
    pub fn new(item: &JsonValue) -> TimeRegister {
        TimeRegister {
            date: item["date"].to_string(),
            start_time: if item["startTime"].is_empty() {
                "-".to_string()
            } else {
                item["startTime"].to_string()
            },
            lunch_start_time: if item["lunchStartTime"].is_empty() {
                "-".to_string()
            } else {
                item["lunchStartTime"].to_string()
            },
            lunch_end_time: if item["lunchEndTime"].is_empty() {
                "-".to_string()
            } else {
                item["lunchEndTime"].to_string()
            },
            end_time: if item["endTime"].is_empty() {
                "-".to_string()
            } else {
                item["endTime"].to_string()
            }
        }
    }

    pub fn to_time_register(json: &JsonValue) -> Vec<TimeRegister> {
        let mut list: Vec<TimeRegister> = Vec::new();
    
        for item in json.members() {
            list.push(TimeRegister::new(item));
        }
    
        list
    }

    pub fn to_json_array(list: Vec<TimeRegister>) -> JsonValue {
        let mut json = JsonValue::new_array();

        for item in list {
            let object = object!{
                "date": item.date,
                "startTime": item.start_time,
                "lunchStartTime": item.lunch_start_time,
                "lunchEndTime": item.lunch_end_time,
                "endTime": item.end_time,
            };

            match json.push(object) {
                Ok(_) => (),
                Err(err) => println!("Error at oushing to json array: {:?}", err),
            };
        }

        json
    }
}

pub fn format_date(year: u32, month: u32, day: u32) -> String {
    let separator = "-".to_string();
    let items = vec![year, month, day];
    let date_formatted = format(items, separator);
    
    date_formatted
}

pub fn format_time(hour: u32, minutes: u32, seconds: u32) -> String {
    let separator = ":".to_string();
    let items = vec![hour, minutes, seconds];
    let time_formatted = format(items, separator);
    
    time_formatted
}

pub fn application_banner() {
    pub const PROGRAM_HEADER: &str = r#"
████████        ████████ ██████   █████   ██████ ██   ██ ███████ ██████  
   ██              ██    ██   ██ ██   ██ ██      ██  ██  ██      ██   ██ 
   ██              ██    ██████  ███████ ██      █████   █████   ██████  
   ██              ██    ██   ██ ██   ██ ██      ██  ██  ██      ██   ██ 
   ██    ██        ██    ██   ██ ██   ██  ██████ ██   ██ ███████ ██   ██"#;

    clear_screen();
    println!("{}\n", PROGRAM_HEADER.fg_rgb::<0x65, 0xB1, 0x22>().bold());
}

fn format(items: Vec<u32>, separator: String) -> String {
    let mut strings: Vec<String> = Vec::new();
    
    for el in items {
        let parsed = if el < 10 {
            "0".to_string() + &el.to_string()
        } else {
            el.to_string()
        };
        
        strings.push(parsed);
    }
    
    return strings.join(&separator).to_string()
}

fn clear_screen() {
    assert!(std::process::Command::new("cls")
        .status()
        .or_else(|_| std::process::Command::new("clear").status())
        .unwrap()
        .success());
}
