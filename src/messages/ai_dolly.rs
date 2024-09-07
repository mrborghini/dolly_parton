use async_trait::async_trait;

use serenity::gateway::client::Context;
use serenity::model::channel::Message;

use crate::components::types::Severity;
use crate::components::Logger;

use super::message_handler::MessageHandler;

pub struct AIDolly {
    logger: Logger,
}

impl AIDolly {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("AIdolly"),
        }
    }
}

#[async_trait]
impl MessageHandler for AIDolly {
    async fn respond(&self, ctx: &Context, msg: &Message) -> bool {
        let function_name = "respond";

        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                self.logger.error(
                    format!("Error sending message: {why:?}").as_str(),
                    function_name,
                    Severity::High,
                );
            }
        }

        return true;
    }
}
