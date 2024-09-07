use serenity::async_trait;
use serenity::gateway::client::Context;
use serenity::model::channel::Message;

#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn respond(&self, ctx: &Context, msg: &Message) -> bool;
}
