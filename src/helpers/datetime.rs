use chrono::{DateTime, Local};

pub fn current_date() -> String {
    let current_time: DateTime<Local> = Local::now();
    current_time.format("%d-%m-%Y").to_string()
}

pub fn replace_date(parts: &mut [String], new_prj_date: &str) {
    if let Some(last_part) = parts.last_mut() {
        if last_part.len() >= 10 {
            last_part.replace_range(0..10, new_prj_date);
        }
    }
}