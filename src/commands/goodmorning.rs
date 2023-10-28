use crate::commands::randomnumber::*;

pub fn run(author: String) -> String {
    let morningmessages = [
        format!(
            "Good morning, {}! May your day be filled with joy and success.",
            author
        ),
        format!("Rise and shine, {}! It's a brand new day.", author),
        format!(
            "Hey {}! Hope you had a restful night and are ready to conquer the day!",
            author
        ),
        format!(
            "Hello there, {}! Wishing you a fantastic morning and a productive day ahead.",
            author
        ),
        format!(
            "Greetings, {}! Let the morning sun energize you for the challenges ahead.",
            author
        ),
        format!(
            "Good morning, {}! Remember to take some time for yourself today.",
            author
        ),
        format!(
            "Morning, {}! Embrace the opportunities this day brings and make the most of them.",
            author
        ),
        format!("Good morning sleepy {}!", author),
        format!("Hi {} I hope you have a wonderful morning :)", author),
        format!(
            "Good morning {}! It's been a year {} I've really really missed you.",
            author, author
        ),
    ];
    morningmessages[random_number(0, morningmessages.len() - 1)].to_string()
}
