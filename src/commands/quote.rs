use reqwest::Client;
use serde::Deserialize;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::components::types::Severity;
use crate::components::Logger;

#[derive(Debug, Deserialize)]
struct ReceivedQuote {
    content: String,
    author: String,
}

async fn get_quote() -> Result<String, reqwest::Error> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)  // This disables SSL certificate verification, because it seems to be having issues often
        .build()?;

    let resp: ReceivedQuote = client
        .get("https://api.quotable.io/random")
        .send()
        .await?
        .json()
        .await?;

    let quote = format!("```{} --{}```", resp.content, resp.author);
    Ok(quote)
}

pub async fn run<'a>(logger: Logger, _options: &[ResolvedOption<'a>]) -> String {
    let function_name = "run";

    match get_quote().await {
        Ok(quote) => quote,
        Err(e) => {
            logger.error(
                format!("Could not get quote: {}", e).as_str(),
                function_name,
                Severity::High,
            );
            "Something went wrong 😭".to_string()
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("quote").description("Generates random quote")
}
