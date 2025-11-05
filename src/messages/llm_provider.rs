use super::{LlmBody, LlmResponse};
use rust_logger::Logger;
use serenity::async_trait;

pub enum MessageRequest {
    WithUrl { llm_body: LlmBody, url: String },
    WithToken { llm_body: LlmBody, token: String },
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn get_message(request: MessageRequest, logger: Logger) -> LlmResponse;
}
