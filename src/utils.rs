use owo_colors::OwoColorize;

pub fn format_date(year: u32, month: u32, day: u32) -> String {
    let separator = "-".to_string();
    let year = year.to_string();

    let month = if month < 10 {
        "0".to_string() + &month.to_string()
    } else {
        month.to_string()
    };

    let day = if day < 10 {
        "0".to_string() + &day.to_string()
    } else {
        day.to_string()
    };

    return year + &separator + &month + &separator + &day;
}

pub fn format_time(hour: u32, minutes: u32, seconds: u32) -> String {
    let separator = ":".to_string();

    let hour = if hour < 10 {
        "0".to_string() + &hour.to_string()
    } else {
        hour.to_string()
    };

    let minutes = if minutes < 10 {
        "0".to_string() + &minutes.to_string()
    } else {
        minutes.to_string()
    };

    let seconds = if seconds < 10 {
        "0".to_string() + &seconds.to_string()
    } else {
        seconds.to_string()
    };

    return hour + &separator + &minutes + &separator + &seconds;
}

pub fn application_banner() {
    pub const PROGRAM_HEADER: &str = r#"
████████        ████████ ██████   █████   ██████ ██   ██ ███████ ██████  
   ██              ██    ██   ██ ██   ██ ██      ██  ██  ██      ██   ██ 
   ██              ██    ██████  ███████ ██      █████   █████   ██████  
   ██              ██    ██   ██ ██   ██ ██      ██  ██  ██      ██   ██ 
   ██    ██        ██    ██   ██ ██   ██  ██████ ██   ██ ███████ ██   ██"#;

    println!("{}", PROGRAM_HEADER.fg_rgb::<0x65, 0xB1, 0x22>().bold());
}