use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    let mut commands = vec![
        "/dolly",
        "/cal",
        "/cal yes",
        "/cal no",
        "/cal maybe",
        "/socialcredits",
        "/socialcredits @user",
        "/rage",
        "/compliment",
        "/compliment @user",
        "/gosleep",
        "/silly",
        "/valgun",
        "/valagents",
        "/rizz",
        "/quote",
        "/work",
        "/wonderful_command",
        "/ping",
        "/add_silly_message message (to ping the user do :user:)",
        "/add_goodmorning_message message (to ping the user do :user:)",
        "/version",
    ];
    commands.sort();
    let response = format!("**Commands:**\n{}", commands.join("\n"));
    response.to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("dollyhelp")
        .description("Get a list of commands")
}
