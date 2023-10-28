use crate::commands::randomnumber::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

fn format_agents(agents: &[&str]) -> String {
    let mut formatted_agents = String::new();

    for (index, agent) in agents.iter().enumerate() {
        let player = format!("**Player {}**:", index + 1);
        formatted_agents.push_str(&format!("{} {}\n", player, agent));
    }

    formatted_agents
}

pub fn run(_options: &[CommandDataOption]) -> String {
    let mut selectedagents: Vec<&str> = Vec::new();

    let agents = [
        "Astra",
        "Breach",
        "Brimstone",
        "Chamber",
        "Cypher",
        "Deadlock",
        "Fade",
        "Gekko",
        "Harbor",
        "Iso",
        "Jett",
        "Kay/O",
        "Killjoy",
        "Neon",
        "Omen",
        "Phoenix",
        "Raze",
        "Reyna",
        "Sage",
        "Skye",
        "Sova",
        "Viper",
        "Yoru",
    ];

    while selectedagents.len() < 5 {
        let agent = agents[random_number(0, agents.len() - 1)];
        if !selectedagents.contains(&agent) {
            selectedagents.push(agent);
        }
    }
    format_agents(&selectedagents).to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("valagents").description("Generate 5 random valorant agents")
}
