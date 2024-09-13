use serenity::async_trait;
use serenity::gateway::client::Context;
use serenity::model::channel::Message;

#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// This function will respond to the user that send a message
    /// 
    /// # Arguments
    /// 
    /// * `ctx` - The context from where the message is from.
    /// * `msg` - The message that has been received.
    async fn respond(&self, ctx: &Context, msg: &Message) -> bool;
}
