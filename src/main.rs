use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use std::env;
mod database;
use database::*;
mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{} {}", msg.author.name, msg.content);

        let splitcommand: Vec<&str> = msg.content.split_whitespace().collect();

        if splitcommand.len() > 0 {
            match splitcommand[0] {
                _ => {}
            }
        }
        match msg
            .content
            .to_lowercase()
            .replace(
                &['(', ')', '?', '!', ' ', ',', '\"', '.', ';', ':', '\''][..],
                "",
            )
            .as_str()
        {
            "morning"
            | "gm"
            | "gutenmorgen"
            | "goodmorning"
            | "goodmorningeveryone"
            | "goodmornin"
            | "buenosdias"
            | "goedemorgen"
            | "buongiorno" => {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        commands::goodmorning::run(format!("{}", msg.author)),
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            "fuckyoudolly" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, ":rage:").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            _ => {}
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let user = command.user.clone();
            // println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "wonderful_command" => commands::wonderful_command::run(&command.data.options),
                "dollyhelp" => commands::dollyhelp::run(&command.data.options),
                "work" => commands::work::run(user, &command.data.options),
                "dolly" => commands::dolly::run(&command.data.options),
                "valagents" => commands::valagents::run(&command.data.options),
                "valgun" => commands::valgun::run(&command.data.options),
                "quote" => commands::quote::run(&command.data.options).await,
                "rage" => commands::rage::run(&command.data.options),
                "daddy" => commands::daddy::run(&command.data.options),
                "gosleep" => commands::gosleep::run(&command.data.options),
                "compliment" => commands::compliment::run(user, &command.data.options),
                "socialcredits" => commands::socialcredits::run(user, &command.data.options),
                "rizz" => commands::rizz::run(user, &command.data.options),
                "cal" => commands::cal::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} HAS AWAKENEND!", ready.user.name);
        let _guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        // let commands = GuildId::set_application_commands(&_guild_id, &ctx.http, |commands| {
        //     commands
        //         .create_application_command(|command| commands::ping::register(command))
        //         .create_application_command(|command| commands::work::register(command))
        //         .create_application_command(|command| commands::socialcredits::register(command))
        //         .create_application_command(|command| commands::cal::register(command))
        // })
        // .await;

        // println!(
        //     "I now have the following guild slash commands: {:#?}",
        //     commands
        // );

        let _guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::wonderful_command::register(command);
            commands::ping::register(command);
            commands::dollyhelp::register(command);
            commands::work::register(command);
            commands::dolly::register(command);
            commands::valagents::register(command);
            commands::valgun::register(command);
            commands::quote::register(command);
            commands::compliment::register(command);
            commands::daddy::register(command);
            commands::gosleep::register(command);
            commands::rage::register(command);
            commands::socialcredits::register(command);
            commands::rizz::register(command);
            commands::cal::register(command)
        })
        .await;

        // println!(
        //     "I created the following global slash command: {:#?}",
        //     _guild_command
        // );
    }
}

#[tokio::main]
async fn main() {
    println!("WAKE UP!");
    dotenv().ok();
    let database_name = "dolly_parton";
    let username = env::var("SQL_USERNAME").expect("Expected a SQL_USERNAME in the environment");
    let password = env::var("SQL_PASSWORD").expect("Expected a SQL_PASSWORD in the environment");
    let hostname = env::var("HOSTNAME").expect("Expected a HOSTNAME in the environment");
    let port = 3306;

    let result = _createdb(&database_name, &username, &password, &hostname, port);

    match result {
        Ok(_) => println!("Waking up brain"),
        Err(err) => eprintln!("Error creating the database: {}", err),
    }

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
