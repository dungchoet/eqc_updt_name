pub fn print_warning(input: &str) {
    let warning_color: &str = "\x1b[31m";
    println!("{}{}\x1b[0m", warning_color, input);
}