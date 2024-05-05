use crate::commands::randomnumber::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
mod database {
    include!("../database.rs");
}
use database::_get_random_silly_message;
use serenity::model::user::User;

pub fn run(author: User, _options: &[CommandDataOption]) -> String {
    let sillymessages = [
        "Look at me I'm silly :man_with_veil_tone1:", 
        "https://media.tenor.com/8lBGroihh-QAAAAd/you-rage.gif", 
        "https://media.discordapp.net/attachments/925855860557746267/1081638870610886707/caption-3-1-1.gif?width=344&height=467&ex=66387372&is=663721f2&hm=51471f87bd0a6dcdb47f77b4af069ee969ed8dafa17f04e11a74d0790d843eb6&",
        "Why couldn't Einstein build a wall? Well he only had Ein stein"
        ];

    let get_message = _get_random_silly_message();

    match get_message {
        Ok(Some((message, _))) => {
            return message.replace(":user:", &author.to_string());
        }
        Ok(None) => {
            return sillymessages[random_number(0, sillymessages.len() - 1)].to_string();
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            return "Something went wrong".to_string();
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("silly").description("I'm so silly")
}
