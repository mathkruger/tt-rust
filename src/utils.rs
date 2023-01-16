use owo_colors::OwoColorize;

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

fn clear_screen() {
    assert!(std::process::Command::new("cls")
        .status()
        .or_else(|_| std::process::Command::new("clear").status())
        .unwrap()
        .success());
}
