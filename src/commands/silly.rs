use crate::commands::randomnumber::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    let sillymessages = [
        "Look at me I'm silly :man_with_veil_tone1:", 
        "https://media.tenor.com/8lBGroihh-QAAAAd/you-rage.gif", 
        "https://media.discordapp.net/attachments/925855860557746267/1081638870610886707/caption-3-1-1.gif?width=344&height=467",
        "Why couldn't Einstein build a wall? Well he only had Ein stein"
        ];

    sillymessages[random_number(0, sillymessages.len() - 1)].to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("silly").description("I'm so silly")
}
