// custom components
mod commands;
mod components;
mod messages;
use commands::{clear_converstation, ping, quote, rage};
use components::types::Severity;
use components::{DotEnvReader, Logger};
use messages::{AIDolly, Insult, MessageHandler, Ping};

// Cargo components
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use std::env;
use tokio::select;
use tokio::signal;

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
                "quote" => Some(commands::quote::run(&command.data.options()).await),
                "clearconversation" => Some(commands::clear_converstation::run(
                    &command.data.options(),
                    self.message_handlers.last().unwrap(),
                )),
                _ => {
                    self.logger.warning(
                        format!("Invalid command: {}", command.data.name.as_str()).as_str(),
                        function_name,
                        Severity::Medium,
                    );
                    Some("not implemented :(".to_string())
                }
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

        let guild_id = match env::var("GUILD_ID") {
            Ok(value) => {
                match value.parse::<u64>() {
                    Ok(parsed) => GuildId::new(parsed),
                    Err(_) => {
                        self.logger.warning(
                            "GUILD_ID is not a valid number. Defaulting to '0'",
                            function_name,
                            Severity::Medium,
                        );
                        GuildId::new(0) // Default to 0 if parsing fails
                    }
                }
            }
            Err(_) => {
                self.logger.debug(
                    "GUILD_ID has not been set in the environment. Defaulting to '0'",
                    function_name,
                );
                GuildId::new(0) // Default to 0 if the environment variable is not set
            }
        };

        // Only if guild_id is not 0 then create the guild commands
        if guild_id != 0 {
            let commands = guild_id
                .set_commands(
                    &ctx.http,
                    &[ping::register(), rage::register(), quote::register(), clear_converstation::register()],
                )
                .await;

            self.logger.debug(
                format!("I now have the following guild slash commands: {commands:#?}").as_str(),
                function_name,
            );
        }

        // Global commands
        let global_commands = [
            Command::create_global_command(&ctx.http, ping::register()).await,
            Command::create_global_command(&ctx.http, rage::register()).await,
            Command::create_global_command(&ctx.http, quote::register()).await,
            Command::create_global_command(&ctx.http, clear_converstation::register()).await,
        ];

        for global_command in global_commands {
            self.logger.debug(
                format!("I created the following global slash command: {global_command:#?}")
                    .as_str(),
                function_name,
            );
        }
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
    let token = env::var("DISCORD_TOKEN").unwrap_or_else(|_| {
        logger.error(
            "DISCORD_TOKEN not found in environment",
            function_name,
            Severity::Critical,
        );
        std::process::exit(1);
    });

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut handlers: Vec<Box<dyn MessageHandler + Send + Sync>> = Vec::new();

    let insult_handler = Insult::new();
    let ai_dolly_handler = AIDolly::new();
    let ping_handler = Ping::new();

    handlers.push(Box::new(insult_handler));
    handlers.push(Box::new(ping_handler));

    // AI dolly should always be last
    handlers.push(Box::new(ai_dolly_handler));

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            logger: Logger::new("Handler"),
            message_handlers: handlers,
        })
        .await
        .expect("Err creating client");

    let client_start_future = client.start();

    // Handle signals
    let ctrl_c_future = signal::ctrl_c();

    #[cfg(unix)]
    let mut sigterm_signal =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Unable to create SIGTERM listener");

    #[cfg(unix)]
    let sigterm_future = sigterm_signal.recv();

    // Use a conditional compilation to make it cross-platform
    #[cfg(unix)]
    select! {
        _ = client_start_future => {
            logger.info("Client has shut down.", function_name);
        },
        _ = ctrl_c_future => {
            logger.info("Ctrl-C has been pressed, shutting down...", function_name);
        },
        _ = sigterm_future => {
            logger.info("SIGTERM received, shutting down...", function_name);
        },
    }

    #[cfg(not(unix))]
    select! {
        _ = client_start_future => {
            logger.info("Client has shut down.", function_name);
        },
        _ = ctrl_c_future => {
            logger.info("Ctrl-C has been pressed, shutting down...", function_name);
        },
    }

    logger.info("Application shutting down gracefully.", function_name);
}
