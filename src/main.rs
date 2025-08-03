use getlibrs::*;
use chrono::{DateTime, Local};
use std::fs;
use std::env;
use std::io;

const DELIMITER: &str = " - ";
const PRJ_CODE_POS: usize = 0;
const VERSION_POS: usize = 2;
const PRJ_NAME_POS: usize = 3;

fn current_date() -> String {
    let current_time: DateTime<Local> = Local::now();
    current_time.format("%d-%m-%Y").to_string()
} // complete

fn print_program_intro() {
    let msg = "
        1: Nouveau projet
        2: Mise a jour
        3: Change numero d'affaire
        4: Change indice
        5: Change nom projet
    ";
    println!("{}", msg);
} // complete

fn check_prj_code(input: i32) -> bool {
    input >= 10000 && input <= 99999
} // complete

fn print_warning(input: &str) {
    let warning_color = "\x1b[31m"; //red
    println!("{}{}\x1b[0m", warning_color, input);
} // complete

fn get_prj_code() -> String {
    let new_prj_code_i32: i32 = loop {
        let output = get_int_i32("Nouveau numero d'affaire: ");
        
        if check_prj_code(output) {
            break output;
        }
        else {
            print_warning("Seulement 5 chiffres acceptés");
        }
    };

    new_prj_code_i32.to_string()
} // complete

fn get_version() -> String {
    let new_version_str = loop {
        let output = get_string("Nouveau indice: ");
        if output.len() == 1 {
            break output.to_uppercase();
        }
        else {
            print_warning("Seulement 1 caractère accepté");
        }
    };
    new_version_str
} // complete

fn split_file_name(file_name: &str) -> Vec<String> {
    file_name.split(" - ").map(|s| s.to_string()).collect()
} // complete

fn get_file_name() -> Vec<String> {
    #[allow(unused_variables)]
    let file_type = vec!["docx", "dwg", "rtd", "xlsx", "txt"];
    
    let mut list_file_name = Vec::new();

    let path = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            return vec![];
        }
    };

    let entries = match fs::read_dir(&path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Không thể đọc thư mục: {}", e);
            return vec![];
        }
    };

    for entry in entries {
        let file_name = match entry {
            Ok(entry) => entry.file_name(),
            Err(e) => {
                eprintln!("Lỗi file: {}", e);
                continue;
            }
        };

        let file_name = match file_name.to_str() {
            Some(name) => name.to_string(),
            None => {
                eprintln!("{:?}", file_name);
                continue;
            }
        };
        
        //Pick file name len > 5
        if file_name.len() < 5 {
            continue;
        }

        // Pick file name with correct prj_code
        let part_name: Vec<&str> = file_name.split(DELIMITER).collect();
        
        let prj_code = match part_name[0].parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if !check_prj_code(prj_code) {
            continue;
        }

        // Pick only .docx .dwg .rtd .xlsx .txt
        if let Some(type_tmp) = part_name.last().and_then(|s| s.split('.').last()) {
            if !file_type.contains(&type_tmp) {
                continue;
            }
        }

        list_file_name.push(file_name);
    }

    list_file_name
} // complete

fn rename_file(old_path: &str, new_path: &str) -> io::Result<()> {
    match fs::rename(old_path, new_path) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Failed to rename file: {}", e);
            Err(e)
        }
    }
} // complete

fn new_project() {
    let new_prj_code = get_prj_code();
    let new_version_str = get_version();
    let new_prj_name_str = get_string("Nouveau nom du projet: ");
    let new_date = current_date();

    let mut file_name_list = get_file_name();

    for file_name in &mut file_name_list {
        let mut file_name_tmp_list = split_file_name(file_name);
        file_name_tmp_list[PRJ_CODE_POS] = new_prj_code.clone();
        file_name_tmp_list[VERSION_POS] = new_version_str.clone();
        file_name_tmp_list[PRJ_NAME_POS] = new_prj_name_str.clone();
        
        // Change date
        if let Some(last) = file_name_tmp_list.last_mut() {
            if let Some((_, ext)) = last.rsplit_once('.') {
                *last = format!("{}.{}", new_date, ext);
            }
        }

        let new_name = file_name_tmp_list.join(" - ");

        match rename_file(&file_name, &new_name) {
            Ok(()) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn update_date() {
    let current_time = current_date();
    
    let mut file_name_list = get_file_name();

    for file_name in &mut file_name_list {
        let mut file_name_tmp_list = split_file_name(file_name);
        
        // Change date
        if let Some(last) = file_name_tmp_list.last_mut() {
            if let Some((_, ext)) = last.rsplit_once('.') {
                *last = format!("{}.{}", current_time, ext);
            }
        }

        let new_name = file_name_tmp_list.join(" - ");

        match rename_file(&file_name, &new_name) {
            Ok(()) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn update_prj_code() {
    let new_prj_code = get_prj_code();
    
    let mut file_name_list = get_file_name();
    
    for file_name in &mut file_name_list {
        let mut file_name_tmp_list = split_file_name(file_name);
        file_name_tmp_list[PRJ_CODE_POS] = new_prj_code.clone();

        let new_name = file_name_tmp_list.join(" - ");

        match rename_file(&file_name, &new_name) {
            Ok(()) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn update_version() {
    let new_version_str = get_version();

    let mut file_name_list = get_file_name();

    for file_name in &mut file_name_list {
        let mut file_name_tmp_list = split_file_name(file_name);
        file_name_tmp_list[VERSION_POS] = new_version_str.clone();

        let new_name = file_name_tmp_list.join(" - ");

        match rename_file(&file_name, &new_name) {
            Ok(()) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn update_prj_name() {
    let new_prj_name_str = get_string("Nouveau nom du projet: ");

    let mut file_name_list = get_file_name();
    
    for file_name in &mut file_name_list {
        let mut file_name_tmp_list = split_file_name(file_name);
        file_name_tmp_list[PRJ_NAME_POS] = new_prj_name_str.clone();

        let new_name = file_name_tmp_list.join(" - ");

        match rename_file(&file_name, &new_name) {
            Ok(()) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }    
}

fn make_choice() {
    loop {
        let user_choice = get_string("Choix: (q=quit) ");

        match user_choice.as_str() {
            "1" => new_project(),
            "2" => update_date(),
            "3" => update_prj_code(),
            "4" => update_version(),
            "5" => update_prj_name(),
            "q" => break,
            _ => println!("Invalide")
        }
    }
} // complete

fn main() {
    print_program_intro();
    make_choice();
}