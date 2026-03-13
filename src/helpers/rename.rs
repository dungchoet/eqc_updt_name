use std::fs;
use std::path::Path;

pub fn rename_file(input_task: Vec<(String, String)>) {
    for (old, new) in input_task {
        if old == new {
            continue;
        }

        if Path::new(&new).exists() {
            continue;
        }

        match fs::rename(&old, &new) {
            Ok(_) => (),
            Err(e) => eprintln!("Error {} : {}", old, e),
        }
    }
}