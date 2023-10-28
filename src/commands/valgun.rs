use crate::commands::randomnumber::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    let valguns = [
        "Classic", "Shorty", "Frenzy", "Ghost", "Sherriff", "Stinger", "Spectre", "Bucky", "Judge",
        "Bulldog", "Guardian", "Phantom", "Vandal", "Marshal", "Operator", "Ares", "Odin",
    ];
    valguns[random_number(0, valguns.len() - 1)].to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("valgun")
        .description("Generate a random valorant gun")
}
