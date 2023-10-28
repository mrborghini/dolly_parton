use serde::Deserialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

#[derive(Debug, Deserialize)]
struct ReceivedQuote {
    content: String,
    author: String,
}

async fn quote() -> Result<String, reqwest::Error> {
        let resp: ReceivedQuote = reqwest::Client::new()
            .get("https://api.quotable.io/random")
            .send()
            .await?
            .json()
            .await?;

        let quote = format!("```{} -- {}```", resp.content, resp.author);
        Ok(quote)
}

pub async fn run(_options: &[CommandDataOption]) -> String {
    match quote().await {
        Ok(quote) => return quote,
        Err(_err) => return "Sorry something unexpected happened".to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("quote").description("Get a random quote")
}
