use crate::commands::randomnumber::*;
use serde_json::{json, Value};
use std::{env, error::Error};

async fn get_ai_message(message: String, base_url: String) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/api/chat", base_url);
    let request_body = json!({
        "model": "llama3",
        "messages": [
            { "role": "user", "content": message }
        ]
    });

    let response_text = reqwest::Client::new()
        .post(url)
        .json(&request_body)
        .send()
        .await?
        .text()
        .await?;

    let mut content_string = String::new();

    for line in response_text.lines() {
        let response: Value = serde_json::from_str(line)?;

        if let Some(content) = response["message"]["content"].as_str() {
            content_string.push_str(content);
        }
    }

    Ok(content_string)
}

pub async fn run(author: String, message: String) -> String {
    let messages = [
        format!("hi {} how are you?", author),
        format!("hi {} I hope you're having an amazing day :)", author),
        format!("Hello {} my fellow American :flag_us: :eagle:", author),
    ];

    let ollama_url = env::var("OLLAMA_URL");

    let final_url: String;

    match ollama_url {
        Ok(url) => {
            final_url = url;
        }
        Err(e) => {
            eprintln!("{}", e);
            return messages[random_number(0, messages.len() - 1)].clone();
        }
    }

    let get_message = get_ai_message(message, final_url).await;

    match get_message {
        Ok(message) => {
            return format!("{} {}", author, message);
        }
        Err(e) => {
            eprintln!("{}", e);
            return messages[random_number(0, messages.len() - 1)].clone();
        }
    }
}
