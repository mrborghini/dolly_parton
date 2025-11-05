use super::message_handler::MessageHandler;
use rust_logger::{Logger, Severity};
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

pub struct Ping {
    logger: Logger,
}

impl Ping {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("Ping"),
        }
    }
}

#[async_trait]
impl MessageHandler for Ping {
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
