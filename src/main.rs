mod constants;
mod utils;
mod helpers;
mod features;

use crate::features::{
    make_choice::make_choice,
    introduction::print_program_intro,
};

fn main() {
    print_program_intro();
    make_choice();
}