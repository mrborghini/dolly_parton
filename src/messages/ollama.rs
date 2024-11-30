use std::time::Duration;

use serenity::async_trait;

use crate::components::{types::Severity, Logger};

use super::{LlmMessage, LlmProvider, LlmResponse, MessageRequest};

pub struct Ollama;

#[async_trait]
impl LlmProvider for Ollama {
    async fn get_message(request: MessageRequest, logger: Logger) -> LlmResponse {
        let function_name = "get_message";

        match request {
            MessageRequest::WithUrl { llm_body, url } => {
                // Check if ollama is online by using the / path
                let ping_url = format!("{}/", url.clone());
                let ping_response = reqwest::Client::new()
                    .get(ping_url)
                    .timeout(Duration::from_millis(500))
                    .send()
                    .await;

                match ping_response {
                    Ok(_) => {}
                    Err(_) => {
                        return LlmResponse {
                            message: LlmMessage {
                                content: String::new(),
                                role: String::new(),
                            },
                        }
                    }
                }

                let request_url = format!("{}/api/chat", url);
                let request_body = serde_json::to_string(&llm_body).unwrap();
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
