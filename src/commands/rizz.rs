use crate::commands::randomnumber::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::user::User;

pub fn run(user: User, _options: &[CommandDataOption]) -> String {
    let mut rizz = random_number(0, 100) as i32;
    let user = &format!("{}", user.clone());
    match user.as_str() {
        "<@1019339320731119626>" => rizz = 0,
        "<@507705534917378050>" => rizz = 100,
        _ => {}
    }
    format!("{} rizz has: {}%", user, rizz)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("rizz")
        .description("Get your rizz in percentage")
}
