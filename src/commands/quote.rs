use serde::Deserialize;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

#[derive(Debug, Deserialize)]
struct ReceivedQuote {
    content: String,
    author: String,
}

async fn get_quote() -> Result<String, reqwest::Error> {
    let resp: ReceivedQuote = reqwest::Client::new()
        .get("https://api.quotable.io/random")
        .send()
        .await?
        .json()
        .await?;

    let quote = format!("```{} -- {}```", resp.content, resp.author);
    Ok(quote)
}

pub async fn run<'a>(_options: &[ResolvedOption<'a>]) -> String {
    match get_quote().await {
        Ok(quote) => quote,
        Err(_) => "Something went wrong 😭".to_string(),
    }
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("quote").description("Generates random quote")
}
