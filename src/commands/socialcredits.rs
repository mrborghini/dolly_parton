use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
mod database {
    include!("../database.rs");
}
use database::{_getuserinfo, _putindb};
use serenity::model::user::User;

pub fn run(user: User, options: &[CommandDataOption]) -> String {
    let mut chosenuser: Option<User> = Some(user);
    let mut ownbalance = true;

    if let Some(option) = options.get(0) {
        if let Some(CommandDataOptionValue::User(customuser, _member)) = &option.resolved {
            chosenuser = Some(customuser.clone());
            ownbalance = false;
        }
    }

    let message = match chosenuser {
        Some(user) => match _getuserinfo(&format!("{}", user)) {
            Ok(Some((username, credits))) => {
                format!(
                    "{} currently has: {} social credits! :money_mouth:",
                    username, credits
                )
            }

            Err(_) => "Something went wrong :(".to_string(),

            Ok(None) => {
                if ownbalance {
                    let adduser = _putindb(&format!("{}", user), 0);

                    match adduser {
                        Ok(_) => println!("Added {} to database", &format!("{}", user)),
                        Err(err) => eprintln!("Error creating user: {}", err),
                    }
                    format!("{} Looks like you're not on my database yet... But luckily for you I just added you on my database :wink:. Just do !socialcredits again to see your social credits :)", user)
                } else {
                    format!("{} has never ran /socialcredits", user)
                }
            }
        },
        None => "I don't know lol".to_string(),
    };
    message
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("socialcredits")
        .description("Get your current balance")
        .create_option(|option| {
            option
                .name("personbalance")
                .description("The person you want the balance of")
                .kind(CommandOptionType::User)
                .required(false)
        })
}
