use std::env;
use std::fs;
use crate::helpers::validator::check_prj_code;
use crate::constants;

pub fn get_file_name() -> Vec<String> {
    let allowed_file_type: Vec<&str> = vec!["docx", "dwg", "rtd", "xlsx", "txt"];
    let mut list_file_name: Vec<String> = Vec::new();

    let path: std::path::PathBuf = env::current_dir().unwrap_or_else(|_| ".".into());

    let entries: fs::ReadDir = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error: {}", e);
            return vec![];
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if !path.is_file() { continue; }

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };
        
        if file_name.len() < 5 { continue; }

        let parts: Vec<&str> = file_name.split(constants::DELIMITER).collect();
        if parts.is_empty() { continue; }
        
        if !check_prj_code(parts[0].to_string()) {
            continue;
        }
        
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if allowed_file_type.contains(&ext) {
                list_file_name.push(file_name);
            }
        }
    }

    list_file_name
}