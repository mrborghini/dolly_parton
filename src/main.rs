// custom components
mod commands;
mod components;
mod messages;
use commands::{ping, rage};
use components::types::Severity;
use components::{DotEnvReader, Logger};

// Cargo components
use messages::ai_dolly::AIDolly;
use messages::insult::Insult;
use messages::message_handler::MessageHandler;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use std::env;

struct Handler {
    logger: Logger,
    message_handlers: Vec<Box<dyn MessageHandler + Send + Sync>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let function_name = "message";
        // Prevent responding to other bots
        if !ctx.cache.current_user().bot() {
            return;
        }

        // Prevent the bot responding to it self
        if ctx.cache.current_user().id == msg.author.id {
            return;
        }

        self.logger.info(
            format!("Received: {}", msg.content),
            function_name.to_string(),
        );

        for handler in self.message_handlers.iter() {
            if handler.respond(&ctx, &msg).await {
                return;
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let function_name = "interaction_create";

        if let Interaction::Command(command) = interaction {
            self.logger.debug(
                format!("Received command interaction: {command:#?}").as_str(),
                function_name,
            );

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "rage" => Some(commands::rage::run(&command.data.options())),
                _ => {
                    self.logger.warning(format!("Invalid command: {}", command.data.name.as_str()).as_str(), function_name, Severity::Medium);
                    Some("not implemented :(".to_string())
                },
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);

                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    self.logger.error(
                        format!("Cannot respond to slash command: {why}").as_str(),
                        function_name,
                        Severity::High,
                    );
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let function_name = "ready";

        self.logger.info(
            format!("{} is connected!", ready.user.name).as_str(),
            function_name,
        );

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, &[ping::register(), rage::register()])
            .await;

        self.logger.debug(
            format!("I now have the following guild slash commands: {commands:#?}").as_str(),
            function_name,
        );
        // Global commands

        // let guild_command = Command::create_global_command(&ctx.http, ping::register()).await;

        // self.logger.debug(
        //     format!("I created the following global slash command: {guild_command:#?}").as_str(),
        //     function_name,
        // );
    }
}

#[tokio::main]
async fn main() {
    let dotenv = DotEnvReader::new(".env");
    dotenv.parse_and_set_env();

    let function_name = "main";

    let logger = Logger::new("Main");
    logger.info("Starting up", function_name);

    // Handle .env
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut handlers: Vec<Box<dyn MessageHandler + Send + Sync>> = Vec::new();

    let insult_handler = Insult::new();
    let ai_dolly_handler = AIDolly::new();

    handlers.push(Box::new(insult_handler));
    handlers.push(Box::new(ai_dolly_handler));

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            logger: Logger::new("Handler"),
            message_handlers: handlers,
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        logger.error(
            format!("Client error: {why:?}").as_str(),
            function_name,
            Severity::Critical,
        );
    }
}
