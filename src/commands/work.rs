use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
mod database {
    include!("../database.rs");
}
use database::_add_credits;
use serenity::model::user::User;

use crate::commands::randomnumber::*;

pub fn run(userid: User, _options: &[CommandDataOption]) -> String {
    let author: String = format!("{}", userid);
    let money: i32 = random_number(0, 20) as i32;
    let creditadd = _add_credits(&format!("{}", author), money);
    let mut userfound = false;
    match creditadd {
        Ok(None) => {
            return format!("{} you're currently not in my database. Please run /socialcredits to get added to the database :)", author).to_string();
        }
        Ok(_) => {
            println!("Added {} social credits to database for {}", money, author);
            userfound = true;
        }
        Err(err) => eprintln!("Error adding credits: {}", err),
    }
    if userfound {
        let mut moneyemote: String = String::new();
        for _ in 0..money {
            moneyemote.push_str(":dollar: ");
        }
        return format!(
            "{} worked and received {} social credits {}",
            author, money, moneyemote
        );
    } else {
        return "You're currently not in my database. Please run !socialcredits to get added to the database :)".to_string();
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("work")
        .description("You work. What did you expect?")
}
