use crate::helpers::{
    get_user_input::get_string,
};

pub fn get_project_name() -> String {
    loop {
        let output: String = match get_string("Nouveau nom du projet: ") {
            Ok(out) => out,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };
        
        if !output.is_empty() {
            break output.to_uppercase();
        }

        println!("Le nom du projet nul.");
    }
}