use serenity::builder::CreateApplicationCommand;

use crate::version;

pub fn run() -> String {
    format!("I'm Dolly {}", version())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("version").description("Shows my version")
}
