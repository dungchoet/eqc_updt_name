use std::io;
use std::io::{Write};

fn read_input() -> io::Result<String> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn get_string(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?; 

    let input = read_input()?;
    Ok(input.trim().to_string())
}