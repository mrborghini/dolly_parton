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

/// This type contains some settings for Ollama
///
/// # Fields
///
/// * `model` - The model that's gonna be used. Like `llama3.1`
/// * `prompt` - The string that's being sent to the api
/// * `system` - System message as a string
#[derive(Debug, Serialize)]
struct OllamaBody {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
}

/// Ollama response as a string
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

/// Ollama message stored in the json file
///
/// # fields
///
/// * `content` - String of the message
/// * `Role` - String of the user id or assistant
#[derive(Debug, Deserialize, Serialize, Clone)]
struct OllamaMessage {
    content: String,
    role: String,
}

/// The whole conversation that gets stored
///
/// # fields
///
/// * `messages` - A vector of OllamaMessages
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Conversation {
    messages: Vec<OllamaMessage>,
}

impl Conversation {
    /// Adds message to the conversation
    ///
    /// # Arguments
    ///
    /// * `message` - The string of the messsage
    /// * `role` - The string of the role. So either userid or assistant
    fn add_message(&mut self, message: String, role: String) {
        let ollama_message = OllamaMessage {
            content: message,
            role,
        };

        self.messages.push(ollama_message);
    }
}

/// This type will communicate with the Ollama api
///
/// # fields
///
/// `logger` - Used for logging information and errors
/// `ollama_base_url` - The url to the ollama server
/// `respond_to_all_messages` - A boolean to send messages to all messages it receives
/// `responds_to_vec` - This is the type of messages it will always respond to
/// `ollama_model` - The model that's gonna be used. Like `llama3.1`
/// `out_dir` - The output directory of the json file
/// `conversation_file` - The json file where it contains the whole conversation
pub struct AIDolly {
    logger: Logger,
    ollama_base_url: String,
    respond_to_all_messages: bool,
    responds_to_vec: Vec<String>,
    ollama_model: String,
    out_dir: String,
    conversation_file: String,
}

impl AIDolly {
    /// Constructor
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

        let mut responds_to_vec: Vec<String> = Vec::new();

        for respond in responds_to.split(",") {
            responds_to_vec.push(respond.to_lowercase().to_string());
        }

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
            responds_to_vec,
            respond_to_all_messages,
            conversation_file,
            out_dir,
        }
    }

    /// This function will save the conversation to a json file
    ///
    /// # Arguments
    ///
    /// `conversation` - The whole conversation with OllamaMessages
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

    /// This function will load the whole conversation from the json file
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

    /// This function will read the system message from system_message.txt
    fn read_system_message(&self) -> String {
        let system_message = read_to_string("system_message.txt");

        match system_message {
            Ok(message) => {
                return message;
            }
            Err(_) => {
                let path_dir = Path::new(&self.out_dir);

                let out_data = read_to_string(path_dir.join("system_message.txt"));

                match out_data {
                    Ok(data) => return data,
                    Err(_) => {
                        self.logger.warning(
                            "No system message found. Please create 'system_message.txt' using 'system_message_example.txt' as a backup",
                            "read_system_message",
                            Severity::High,
                        );
                        return read_to_string("system_message_example.txt").unwrap();
                    }
                }
            }
        }
    }

    /// This function will format the prompt like: `role: message`
    fn format_into_prompt(&self, conversation: Conversation) -> String {
        let new_line = if cfg!(windows) { "\r\n" } else { "\n" };
        let mut output = String::new();

        for message in conversation.messages {
            output.push_str(format!("{}: {}{}", message.role, message.content, new_line).as_str());
        }

        return output;
    }

    /// This function will crop a string to a set limit in case the message is too long
    ///
    /// # Arguments
    ///
    /// * `input_string` - The string of the message to be cropped
    /// * `limit` - The max length that the message will crop to
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

    /// This function will get a message from the ollama api
    ///
    /// # Arguments
    ///
    /// * `msg` - The original Discord message
    async fn get_ollama_message(&self, msg: &Message) -> String {
        let function_name = "get_ollama_message";

        if self.ollama_base_url == "" {
            self.logger
                .error("Ollama url has not been set", function_name, Severity::High);
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

    /// This function just removes spaces
    ///
    /// # Arguments
    ///
    /// * `input` - The string you want to remove the spaces from.
    fn remove_spaces(&self, input: String) -> String {
        input.replace(" ", "")
    }

    /// Check if message contains any string from `responds_to_vec`
    ///
    /// # Arguments
    ///
    /// * `message` - The string of the message you want to check for matches.
    fn contains_names(&self, message: String) -> bool {
        for respond in &self.responds_to_vec {
            if self
                .remove_spaces(message.trim().to_string())
                .contains(&self.remove_spaces(respond.trim().to_string()))
            {
                return true;
            }
        }

        return false;
    }

    /// Clears the conversation by removing the conversation json file
    pub fn clear_conversation(&self) -> bool {
        let function_name = "clear_conversation";
        let dir_path = Path::new(&self.out_dir);

        let conversation_file = dir_path.join(self.conversation_file.clone());

        if !conversation_file.exists() {
            self.logger
                .warning("No conversation to delete.", function_name, Severity::Low);
            return true;
        }

        let result = fs::remove_file(conversation_file);

        match result {
            Ok(_) => {
                self.logger
                    .info("Successfully cleared conversation", function_name);
                return true;
            }
            Err(why) => {
                self.logger.error(
                    format!("Could not delete conversation: {}", why).as_str(),
                    function_name,
                    Severity::Medium,
                );
                return false;
            }
        }
    }
}

#[async_trait]
impl MessageHandler for AIDolly {
    /// This function will respond using a received message from discord using Ollama
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context from where the message is from.
    /// * `msg` - The message that has been received.
    async fn respond(&self, ctx: &Context, msg: &Message) -> bool {
        let function_name = "respond";
        let bot_id = &ctx.cache.current_user().id.to_string();

        let message = msg.content.to_lowercase();

        // Respond if responding to all messages is on.
        // Respond if the message contains the input from the RESPONDS_TO environment
        // Respond if the bot has been pinged inside of the message
        if self.respond_to_all_messages
            || self.contains_names(message.clone())
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

    /// This function will clear the conversations
    fn clean_up(&self) -> bool {
        return self.clear_conversation();
    }
}
