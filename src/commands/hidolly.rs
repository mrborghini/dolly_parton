use crate::{_add_context_to_dolly_ai, _get_dolly_context, commands::randomnumber::*};
use serde_json::{json, Value};
use std::{env, error::Error};

async fn get_ai_message(
    message: String,
    base_url: String,
    ollama_model: String,
    ollama_system_message: String,
) -> Result<String, Box<dyn Error>> {
    let mut contexts: Vec<i32> = Vec::new();

    let received_contexts = _get_dolly_context();

    match received_contexts {
        Ok(ctx) => {
            contexts = ctx;
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    let url = format!("{}/api/generate", base_url);
    let request_body = json!({
        "model": ollama_model,
        "prompt": message,
        "system": ollama_system_message,
        "context": contexts
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

        if let Some(content) = response["response"].as_str() {
            content_string.push_str(content);
        }

        if let Some(ctx) = response["context"].as_array() {
            let ctx_i32: Vec<i32> = ctx
                .iter()
                .filter_map(|v| v.as_i64())
                .map(|v| v as i32)
                .collect();

            let ctx_string = ctx_i32
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            let _ = _add_context_to_dolly_ai(format!("[{}]", ctx_string));
        }
    }

    println!("Generated: {}", content_string);
    Ok(content_string)
}

pub async fn run(author: String, message: String) -> String {
    let messages = [
        format!("hi {} how are you?", author),
        format!("hi {} I hope you're having an amazing day :)", author),
        format!("Hello {} my fellow American :flag_us: :eagle:", author),
    ];

    let ollama_url = env::var("OLLAMA_URL");
    let ollama_model = env::var("OLLAMA_MODEL");
    let ollama_system_message = env::var("OLLAMA_SYSTEM_MESSAGE");

    let chosen_model: String;
    let chosen_system_message: String;

    match ollama_model {
        Ok(model) => {
            chosen_model = model;
        }
        Err(_) => {
            chosen_model = "llama3".to_string();
        }
    }

    match ollama_system_message {
        Ok(message) => {
            chosen_system_message = message;
            println!("{}", chosen_system_message);
        }
        Err(e) => {
            eprintln!("{}", e);
            chosen_system_message = "Very useful assistant".to_string();
        }
    }

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

    let get_message = get_ai_message(
        format!("Sent by: ({}) {}", author, message),
        final_url,
        chosen_model,
        chosen_system_message,
    )
    .await;

    match get_message {
        Ok(message) => {
            let trimmed_message = if message.len() > 1950 {
                &message[..2000]
            } else {
                &message
            };
            return trimmed_message.to_string();
        }
        Err(e) => {
            eprintln!("{}", e);
            return messages[random_number(0, messages.len() - 1)].clone();
        }
    }
}
