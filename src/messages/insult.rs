use super::message_handler::MessageHandler;
use rust_logger::{Logger, Severity};
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

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
        if msg.content == "!ping" {
            match msg.channel_id.say(&ctx.http, "Pong!").await {
                Ok(_) => return true,
                Err(why) => {
                    self.logger.error(
                        format!("Error sending message: {why:?}").as_str(),
                        Severity::High,
                    );
                    return false;
                }
            };
        }
        return false;
    }
}
