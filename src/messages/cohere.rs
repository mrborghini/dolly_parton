use super::{LlmMessage, LlmProvider, LlmResponse, MessageRequest};
use rust_logger::{Logger, Severity};
use serde::Deserialize;
use serenity::async_trait;

pub struct Cohere;

#[derive(Debug, Deserialize)]
struct CohereResponse {
    message: CohereMessage,
}

#[derive(Debug, Deserialize)]
struct CohereMessage {
    role: String,
    content: Vec<CohereText>,
}

#[derive(Debug, Deserialize)]
struct CohereText {
    text: String,
}

#[async_trait]
impl LlmProvider for Cohere {
    async fn get_message(request: MessageRequest, logger: Logger) -> LlmResponse {
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
                        let cohere_response = response.json::<CohereResponse>().await;

                        match cohere_response {
                            Ok(cohere_response) => {
                                let content = cohere_response.message.content;

                                if content.is_empty() {
                                    logger.error("Could not get content", Severity::High);
                                    return LlmResponse {
                                        message: LlmMessage {
                                            content: content[0].text.as_str().to_string(),
                                            role: cohere_response.message.role,
                                        },
                                    };
                                }

                                return LlmResponse {
                                    message: LlmMessage {
                                        content: content[0].text.as_str().to_string(),
                                        role: cohere_response.message.role,
                                    },
                                };
                            }
                            Err(why) => {
                                logger.error(
                                    format!("Could not parse Cohere response: {}", why).as_str(),
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
