use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::env;

pub fn run(_options: &[ResolvedOption]) -> String {
    let version = env!("CARGO_PKG_VERSION").to_string();
    format!("My version is: `v{}`", version)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("version").description("Shows my current version")
}
