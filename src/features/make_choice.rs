use crate::features::project_feature::*;
use crate::utils::get_user_choice::get_choice;

pub fn make_choice() {
    loop {
        let user_choice: String = get_choice();

        match user_choice.as_str() {
            "1" => new_project(),
            "2" => rename_date(),
            "3" => rename_prj_code(),
            "4" => rename_version(),
            "5" => rename_prj_name(),
            "q" => break,
            _ => println!("Invalide")
        }
    }
}