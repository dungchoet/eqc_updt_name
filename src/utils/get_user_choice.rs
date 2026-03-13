use crate::helpers::get_user_input::get_string;

pub fn get_choice() -> String {
    loop {
        let output: String = match get_string("Choix: (q=quit): ") {
            Ok(out) => out,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };
        
        if !output.is_empty() {
            break output;
        }
    }
}