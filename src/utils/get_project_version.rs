use crate::helpers::{
    get_user_input::get_string,
    display::print_warning,
};

pub fn get_version() -> String {
    loop {
        let output: String = match get_string("Nouveau indice: ") {
            Ok(out) => out,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        if output.len() == 1 {
            break output.to_uppercase();
        }
        
        print_warning("Seulement 1 caractère accepté");
    }
}