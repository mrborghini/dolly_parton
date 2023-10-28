use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    let mut calmessage = "Yes, NO!??? maybe :thinking:".to_string();
    let mut calcompare = String::new();

    if let Some(option) = options.get(0) {
        if let Some(CommandDataOptionValue::String(message)) = &option.resolved {
            calcompare = message.clone();
        }
    }

    match calcompare.to_lowercase().as_str() {
        "yes" => calmessage = "YES!".to_string(),
        "no" => calmessage = "NO!".to_string(),
        "maybe" => calmessage = ":thinking:".to_string(),
        _ => {}
    }

    calmessage
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cal")
        .description("Yes no maybe")
        .create_option(|option| {
            option
                .name("option")
                .description("Yes no maybe")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
