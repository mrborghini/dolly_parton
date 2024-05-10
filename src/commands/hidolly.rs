use crate::{_add_context_to_dolly_ai, _get_dolly_context, commands::randomnumber::*};
use serde_json::{json, Value};
use std::{env, error::Error};

async fn get_ai_message(message: String, base_url: String) -> Result<String, Box<dyn Error>> {
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
        "model": "llama3",
        "prompt": message,
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
            let mut allowed: Vec<i32> = Vec::new();
            let ctx_i32: Vec<i32> = ctx
                .iter()
                .filter_map(|v| v.as_i64())
                .map(|v| v as i32)
                .collect();

            for &item in ctx_i32.iter() {
                if !contexts.contains(&item) && !allowed.contains(&item) {
                    allowed.push(item);
                }
            }

            let _ = _add_context_to_dolly_ai(&allowed);
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
