use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::{env, fs};

fn write_system_prompt(prompt: String) -> Result<(), std::io::Error> {
    fs::write("system_message.txt", prompt)
}

pub fn run(options: &[ResolvedOption]) -> String {
    let allow_changing =
        env::var("ALLOW_CHANGING_SYSTEM_PROMPT").unwrap_or("false".to_string()) == "true";

    if !allow_changing {
        return "Not allowed to change the system prompt".to_string();
    }

    let mut prompt = String::new();

    for option in options {
        if option.name == "prompt"
            && let ResolvedValue::String(value) = &option.value
        {
            prompt = value.to_string();
        }
    }

    if prompt.is_empty() {
        return "Error: no prompt provided".to_string();
    }

    let result = write_system_prompt(prompt);

    match result {
        Ok(()) => "Successfully changed prompt".to_string(),
        Err(_) => "Failed to edit the system prompt".to_string(),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("change_system_prompt")
        .description("Allows you to change the system prompt")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "prompt",
                "The prompt with how you want it to respond.",
            )
            .required(true),
        )
}
