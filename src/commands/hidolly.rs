use crate::commands::randomnumber::*;

pub fn run(author: String) -> String {
    let messages = [
        format!("hi {} how are you?", author),
        format!("hi {} I hope you're having an amazing day :)", author),
        format!("Hello {} my fellow American :flag_us: :eagle:", author),
    ];
    messages[random_number(0, messages.len() - 1)].clone()
}
