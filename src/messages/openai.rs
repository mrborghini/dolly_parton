use serde::Deserialize;
use serenity::async_trait;

use crate::components::{types::Severity, Logger};

use super::{LlmMessage, LlmProvider, LlmResponse, MessageRequest};

#[derive(Debug, Clone, Deserialize)]
struct OpenAIResponse {
    choices: Vec<LlmResponse>,
}

pub struct OpenAI;

#[async_trait]
impl LlmProvider for OpenAI {
    async fn get_message(request: MessageRequest, logger: Logger) -> LlmResponse {
        let function_name = "get_message";
        match request {
            MessageRequest::WithUrl {
                llm_body: _,
                url: _,
            } => LlmResponse {
                message: LlmMessage {
                    content: String::new(),
                    role: String::new(),
                },
            },
            MessageRequest::WithToken { llm_body, token } => {
                let request_url = "https://api.cohere.com/v2/chat".to_string();

                let request_body = serde_json::to_string(&llm_body).unwrap();

                let response = reqwest::Client::new()
                    .post(request_url)
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Content-Type", "application/json")
                    .body(request_body)
                    .send()
                    .await;

                match response {
                    Ok(response) => {
                        let openai_response = response.json::<OpenAIResponse>().await;

                        match openai_response {
                            Ok(response) => {
                                if response.choices.len() == 0 {
                                    panic!();
                                }

                                let choice = response.choices[0].clone();

                                choice
                            }
                            Err(why) => {
                                logger.error(
                                    format!("Could not get OpenAi message: {}", why).as_str(),
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
        }
    }
}
