use serenity::async_trait;
use serenity::gateway::client::Context;
use serenity::model::channel::Message;

use crate::components::types::Severity;
use crate::components::Logger;

use super::message_handler::MessageHandler;

pub struct Insult {
    logger: Logger,
}

impl Insult {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("Insult"),
        }
    }
}

#[async_trait]
impl MessageHandler for Insult {
    async fn respond(&self, ctx: &Context, msg: &Message) -> bool {
        let function_name = "respond";

        if msg.content == "!ping" {
            match msg.channel_id.say(&ctx.http, "Pong!").await {
                Ok(_) => return true,
                Err(why) => {
                    self.logger.error(
                        format!("Error sending message: {why:?}").as_str(),
                        function_name,
                        Severity::High,
                    );
                    return false;
                }
            };
        }
        return false;
    }
}
