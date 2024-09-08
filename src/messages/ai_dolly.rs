use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::gateway::client::Context;
use serenity::model::channel::Message;

use crate::components::types::Severity;
use crate::components::Logger;

use super::message_handler::MessageHandler;

#[derive(Debug, Serialize)]
struct OllamaBody {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct OllamaMessage {
    content: String,
    role: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Conversation {
    messages: Vec<OllamaMessage>,
}

impl Conversation {
    fn add_message(&mut self, message: String, role: String) {
        let ollama_message = OllamaMessage {
            content: message,
            role,
        };

        self.messages.push(ollama_message);
    }
}

pub struct AIDolly {
    logger: Logger,
    ollama_base_url: String,
    respond_to_all_messages: bool,
    responds_to: String,
    ollama_model: String,
    out_dir: String,
    conversation_file: String,
}

impl AIDolly {
    pub fn new() -> Self {
        let function_name = "new";

        let out_dir = "out_data".to_string();

        let conversation_file = "conversation.json".to_string();

        let logger = Logger::new("AIdolly");

        // Ollama URL
        let ollama_base_url = env::var("OLLAMA_URL").unwrap_or_else(|_| {
            logger.error(
                "OLLAMA_URL has not been set in the environment",
                function_name,
                Severity::High,
            );
            "".to_string()
        });

        // Validate URL
        let ollama_base_url = if ollama_base_url.starts_with("http") {
            ollama_base_url
        } else {
            logger.error(
                "OLLAMA_URL does not contain 'http' or 'https' scheme",
                function_name,
                Severity::High,
            );
            "".to_string()
        };

        // Ollama model
        let ollama_model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| {
            let default_model = "llama3.1".to_string();
            logger.error(
                format!(
                    "OLLAMA_MODEL has not been set in the environment. Defaulting to {}",
                    default_model
                )
                .as_str(),
                function_name,
                Severity::Medium,
            );
            default_model
        });

        // Responds to
        let responds_to = env::var("RESPONDS_TO")
            .unwrap_or_else(|_| {
                let default_responds_to = "dolly".to_string();
                logger.warning(
                    format!(
                        "RESPONDS_TO has not been set in the environment. Defaulting to '{}'",
                        default_responds_to
                    )
                    .as_str(),
                    function_name,
                    Severity::Low,
                );
                default_responds_to
            })
            .to_lowercase();

        // Respond to all messages
        let respond_to_all_messages = env::var("RESPOND_TO_ALL_MESSAGES")
            .unwrap_or_else(|_| {
                logger.warning(
                    "RESPOND_TO_ALL_MESSAGES has not been set in the environment. Defaulting to 'false'",
                    function_name,
                    Severity::Low,
                );
                "false".to_string()
            })
            .to_lowercase() == "true";

        Self {
            logger,
            ollama_base_url,
            ollama_model,
            responds_to,
            respond_to_all_messages,
            conversation_file,
            out_dir,
        }
    }

    fn save_conversation(&self, conversation: Conversation) {
        let function_name = "save_conversation";

        // Define the directory path.
        let dir_path = Path::new(&self.out_dir);

        // Create the directory if it doesn't exist.
        if !dir_path.exists() {
            if let Err(e) = fs::create_dir_all(dir_path) {
                eprintln!("Failed to create directory: {}", e);
                return;
            }
        }

        // Overwrite file
        let file = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(dir_path.join(self.conversation_file.clone()));

        let json_string = serde_json::to_string_pretty(&conversation).unwrap();

        match file {
            Ok(mut f) => {
                let _ = f.write_all(json_string.as_bytes());
                self.logger.info("Saved conversation", function_name);
                drop(f)
            }
            Err(e) => self.logger.error(
                format!("Could not save conversation: '{}'", e).as_str(),
                function_name,
                Severity::High,
            ),
        }
    }

    fn load_conversation(&self) -> Conversation {
        let function_name = "load_conversation";

        let dir_path = Path::new(&self.out_dir);

        let conversation_file = dir_path.join(self.conversation_file.clone());

        let content = read_to_string(&conversation_file);

        match content {
            Ok(content) => {
                let conversation: Conversation = serde_json::from_str(&content.as_str()).unwrap();
                return conversation;
            }
            Err(_) => self.logger.warning(
                format!(
                    "No '{}' found. You can safely ignore this.",
                    conversation_file.into_os_string().into_string().unwrap(),
                )
                .as_str(),
                function_name,
                Severity::None,
            ),
        }

        return Conversation {
            messages: Vec::new(),
        };
    }

    fn read_system_message(&self) -> String {
        let system_message = read_to_string("system_message.txt");

        match system_message {
            Ok(message) => {
                return message;
            }
            Err(_) => {
                self.logger.warning(
                    "No system message found. Please create 'system_message.txt'",
                    "read_system_message",
                    Severity::High,
                );
                return read_to_string("system_message_example.txt").unwrap();
            }
        }
    }

    fn format_into_prompt(&self, conversation: Conversation) -> String {
        let new_line = if cfg!(windows) { "\r\n" } else { "\n" };
        let mut output = String::new();

        for message in conversation.messages {
            output.push_str(format!("{}: {}{}", message.role, message.content, new_line).as_str());
        }

        return output;
    }

    fn crop_string(&self, input_string: &String, limit: usize) -> String {
        // Use char_indices to ensure we don't split a multi-byte character
        let mut end_index = input_string.len();

        for (i, _) in input_string.char_indices() {
            if i > limit {
                end_index = i;
                break;
            }
        }

        // Return the safely cropped string
        input_string[..end_index].to_string()
    }

    async fn get_ollama_message(&self, msg: &Message) -> String {
        let function_name = "get_ollama_message";

        if self.ollama_base_url == "" {
            self.logger.error(
                "Ollama url has not been set",
                function_name,
                Severity::High,
            );
            return "Something went wrong 😭".to_string();
        }

        let mut conversation = self.load_conversation();

        conversation.add_message(msg.content.to_string(), msg.author.to_string());

        let request_url = format!("{}/api/generate", self.ollama_base_url.clone());

        self.logger.debug(request_url.as_str(), function_name);

        let prompt_data = OllamaBody {
            model: self.ollama_model.clone(),
            prompt: self.format_into_prompt(conversation.clone()),
            system: self.read_system_message(),
            stream: false,
        };

        let request_body = serde_json::to_string(&prompt_data).unwrap();

        let response = reqwest::Client::new()
            .post(request_url)
            .body(request_body)
            .send()
            .await;

        match response {
            Ok(response) => {
                let response_json = response.json::<OllamaResponse>().await;

                match response_json {
                    Ok(ollama_response) => {
                        conversation
                            .add_message(ollama_response.response.clone(), "assistant".to_string());
                        self.save_conversation(conversation);
                        return self.crop_string(&ollama_response.response, 1950);
                    }
                    Err(why) => {
                        self.logger.error(
                            format!("Could not request Ollama response: {}", why).as_str(),
                            function_name,
                            Severity::High,
                        );
                        return "Something went wrong 😭".to_string();
                    }
                }
            }
            Err(why) => {
                self.logger.error(
                    format!("Could not request Ollama response: {}", why).as_str(),
                    function_name,
                    Severity::High,
                );
                return "Something went wrong 😭".to_string();
            }
        }
    }
}

#[async_trait]
impl MessageHandler for AIDolly {
    async fn respond(&self, ctx: &Context, msg: &Message) -> bool {
        let function_name = "respond";
        let bot_id = &ctx.cache.current_user().id.to_string();

        let message = msg.content.to_lowercase();

        // Respond if responding to all messages is on.
        // Respond if the message contains the input from the RESPONDS_TO environment
        // Respond if the bot has been pinged inside of the message
        if self.respond_to_all_messages
            || message.contains(&self.responds_to)
            || message.contains(bot_id)
        {
            self.logger.info("Using ollama to respond", function_name);
            match msg
                .channel_id
                .say(&ctx.http, self.get_ollama_message(msg).await)
                .await
            {
                Ok(_) => return true,
                Err(why) => {
                    self.logger.error(
                        format!("Error sending message: {why:?}").as_str(),
                        function_name,
                        Severity::High,
                    );
                    return false;
                }
            }
        }
        return false;
    }
}
