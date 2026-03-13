use crate::helpers::{
    validator::check_prj_code, 
    get_user_input::get_string,
    display::print_warning,
};

pub fn get_prj_code() -> String {
    loop {
        let output: String = match get_string("Nouveau numero d'affaire: ") {
            Ok(out) => out,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };
        
        if check_prj_code(output.clone()) {
            break output;
        }

        print_warning("Format: AB-CDE");
    }
}