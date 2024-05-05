use serenity::builder::CreateApplicationCommand;

use crate::VERSION;

pub fn run() -> String {
    format!("I'm Dolly v{}", VERSION)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("version").description("Shows my version")
}
