use std::{env, time::Duration};

use serde::Serialize;
use serenity::async_trait;

use crate::components::{Logger, types::Severity};

use super::{LlmMessage, LlmProvider, LlmResponse, MessageRequest};

#[derive(Serialize)]
struct OllamaBody {
    model: String,
    messages: Vec<LlmMessage>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    num_ctx: i32,
}

pub struct Ollama;

#[async_trait]
impl LlmProvider for Ollama {
    async fn get_message(request: MessageRequest, logger: Logger) -> LlmResponse {
        let function_name = "get_message";

        match request {
            MessageRequest::WithUrl { llm_body, url } => {
                // Check if ollama is online by using the / path
                let ping_url = format!("{}/", url.clone());
                let ping_client = reqwest::Client::builder()
                    .timeout(Duration::from_millis(500)) // Applies to the entire request
                    .build()
                    .unwrap();
                let ping_response = ping_client.get(ping_url).send().await;
                match ping_response {
                    Ok(_) => {}
                    Err(_) => {
                        return LlmResponse {
                            message: LlmMessage {
                                content: String::new(),
                                role: String::new(),
                            },
                        };
                    }
                }

                let num_ctx: i32 = env::var("NUM_CTX").unwrap().parse().unwrap_or(2048);

                logger.debug(
                    format!("Using {} context window", num_ctx).as_str(),
                    function_name,
                );

                let ollama_body = OllamaBody {
                    model: llm_body.model,
                    messages: llm_body.messages,
                    stream: false,
                    options: OllamaOptions { num_ctx },
                };

                let request_url = format!("{}/api/chat", url);
                let request_body = serde_json::to_string(&ollama_body).unwrap();
                let response = reqwest::Client::new()
                    .post(request_url)
                    .body(request_body)
                    .send()
                    .await;

                match response {
                    Ok(response) => {
                        let response_json = response.json::<LlmResponse>().await;

                        match response_json {
                            Ok(response) => response,
                            Err(why) => {
                                logger.error(
                                    format!("Could not get Ollama response: {}", why).as_str(),
                                    function_name,
                                    Severity::High,
                                );
                                LlmResponse {
                                    message: LlmMessage {
                                        content: String::new(),
                                        role: String::new(),
                                    },
                                }
                            }
                        }
                    }
                    Err(_) => LlmResponse {
                        message: LlmMessage {
                            content: String::new(),
                            role: String::new(),
                        },
                    },
                }
            }
            MessageRequest::WithToken {
                llm_body: _,
                token: _,
            } => LlmResponse {
                message: LlmMessage {
                    content: String::new(),
                    role: String::new(),
                },
            },
        }
    }
}
