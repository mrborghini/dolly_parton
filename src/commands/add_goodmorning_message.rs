use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::_add_goodmorning_message;

pub fn run(options: &[CommandDataOption]) -> String {
    let mut message = String::new();
    if let Some(option) = options.get(0) {
        if let Some(CommandDataOptionValue::String(customuser)) = &option.resolved {
            message = customuser.clone();
        }
    }

    let _ = _add_goodmorning_message(message.as_str());

    format!("Successfully added '{}'", message)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add_goodmorning_message")
        .description("Adds a silly message to my database")
        .create_option(|message| {
            message
                .name("message")
                .description("The message you want to add :) to add a user do :user:")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
