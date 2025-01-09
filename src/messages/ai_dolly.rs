use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

use crate::components::types::Severity;
use crate::components::Logger;

use super::message_handler::MessageHandler;
use super::{Cohere, LlmProvider, MessageRequest, Ollama, OpenAI};

/// This type contains some settings for Ollama
///
/// # Fields
///
/// * `model` - The model that's gonna be used. Like `llama3.1`
/// * `prompt` - The string that's being sent to the api
/// * `system` - System message as a string
#[derive(Debug, Clone, Serialize)]
pub struct LlmBody {
    pub model: String,
    pub messages: Vec<LlmMessage>,
    pub stream: bool,
}

/// Ollama response as a string
#[derive(Debug, Clone, Deserialize)]
pub struct LlmResponse {
    pub message: LlmMessage,
}

/// Ollama message stored in the json file
///
/// # fields
///
/// * `content` - String of the message
/// * `Role` - String of the user id or assistant
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LlmMessage {
    pub content: String,
    pub role: String,
}

/// The whole conversation that gets stored
///
/// # fields
///
/// * `messages` - A vector of OllamaMessages
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Conversation {
    pub messages: Vec<LlmMessage>,
}

impl Conversation {
    /// Adds message to the conversation
    ///
    /// # Arguments
    ///
    /// * `message` - The string of the messsage
    /// * `role` - The string of the role. So either userid or assistant
    fn add_message(&mut self, message: String, role: String, max_messages: i32) {
        self.trim_messages(max_messages);
        let ollama_message = LlmMessage {
            content: message,
            role,
        };

        self.messages.push(ollama_message);
    }

    fn trim_messages(&mut self, max_messages: i32) {
        if max_messages == 0 {
            return;
        }

        if self.messages.len() as i32 == max_messages - 1 {
            self.messages.remove(0);
            return;
        }

        if self.messages.len() as i32 > max_messages - 1 {
            for _ in self.messages.clone() {
                if self.messages.len() as i32 == max_messages - 1 {
                    return;
                }

                self.messages.remove(0);
            }
        }
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
    max_stored_messages: i32,
    openai_model: String,
    openai_token: String,
    cohere_token: String,
    cohere_model: String,
    priortize_ollama: bool,
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

        // Max stored messages
        let max_stored_messages: i32 = env::var("MAX_STORED_MESSAGES")
            .unwrap_or_else(|_| {
                let default_amount = 6;
                logger.error(
                    format!(
                        "MAX_STORED_MESSAGES has not been set in the environment. Defaulting to {}",
                        default_amount
                    )
                    .as_str(),
                    function_name,
                    Severity::Medium,
                );
                default_amount.to_string() // Convert to String for parse.
            })
            .parse()
            .unwrap_or_else(|_| {
                let fallback_amount = 6;
                logger.error(
                    format!(
                        "MAX_STORED_MESSAGES is an invalid number. Defaulting to {}",
                        fallback_amount
                    )
                    .as_str(),
                    function_name,
                    Severity::Medium,
                );
                fallback_amount
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

        // OpenAI Token
        let openai_token = env::var("OPENAI_TOKEN").unwrap_or_else(|_| "".to_string());

        // OpenAI Model
        let openai_model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o".to_string());

        // Cohere Token
        let cohere_token = env::var("COHERE_TOKEN").unwrap_or_else(|_| "".to_string());

        // Cohere Model
        let cohere_model =
            env::var("COHERE_MODEL").unwrap_or_else(|_| "command-r-plus-08-2024".to_string());

        // Prioritize Ollama
        let priortize_ollama = env::var("PRIORTIZE_OLLAMA")
            .unwrap_or_else(|_| "true".to_string())
            .to_lowercase()
            == "true";

        Self {
            logger,
            ollama_base_url,
            ollama_model,
            responds_to_vec,
            respond_to_all_messages,
            conversation_file,
            out_dir,
            max_stored_messages,
            openai_token,
            openai_model,
            cohere_token,
            cohere_model,
            priortize_ollama,
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
            .truncate(true)
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

        self.logger.debug(
            format!("{:?}", conversation_file.as_os_str()).as_str(),
            function_name,
        );

        let content = read_to_string(&conversation_file);

        match content {
            Ok(content) => {
                let conversation: Conversation = serde_json::from_str(content.as_str()).unwrap();
                self.logger
                    .debug(format!("{:#?}", conversation).as_str(), function_name);
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

        Conversation {
            messages: Vec::new(),
        }
    }

    /// This function will read the system message from system_message.txt
    fn read_system_message(&self) -> String {
        let function_name = "read_system_message";
        let system_message = read_to_string("system_message.txt");

        match system_message {
            Ok(message) => {
                self.logger.debug(message.as_str(), function_name);
                message
            }
            Err(_) => {
                let path_dir = Path::new(&self.out_dir);

                let out_data = read_to_string(path_dir.join("system_message.txt"));

                match out_data {
                    Ok(data) => data,
                    Err(_) => {
                        self.logger.warning(
                            "No system message found. Please create 'system_message.txt' using 'system_message_example.txt' as a backup",
                            "read_system_message",
                            Severity::High,
                        );
                        let system_message = read_to_string("system_message_example.txt").unwrap();
                        self.logger.debug(system_message.as_str(), function_name);
                        system_message
                    }
                }
            }
        }
    }

    async fn get_llm_message_based_on_settings(&self, llm_body: LlmBody) -> LlmResponse {
        let function_name = "get_llm_message_based_on_settings";

        if self.priortize_ollama {
            self.logger.info("Using Ollama to respond", function_name);
            let ollama_response = Ollama::get_message(MessageRequest::WithUrl {
                url: self.ollama_base_url.clone(),
                llm_body: llm_body.clone(),
            }, self.logger.clone())
            .await;

            // If Ollama response is empty and Cohere token is available, fall back to Cohere
            if ollama_response.message.role.is_empty() && !self.cohere_token.is_empty() {
                return self.get_cohere_response(llm_body).await;
            }

            // If Ollama response has a role and OpenAI token is available, fall back to OpenAI
            if !ollama_response.message.role.is_empty() && !self.openai_token.is_empty() {
                return self.get_openai_response(llm_body).await;
            }

            ollama_response
        } else {
            // Prioritize Cohere or OpenAI if available, otherwise fall back to Ollama
            if !self.cohere_token.is_empty() {
                return self.get_cohere_response(llm_body).await;
            } else if !self.openai_token.is_empty() {
                return self.get_openai_response(llm_body).await;
            }

            self.logger.info("Using Ollama to respond", function_name);
            Ollama::get_message(MessageRequest::WithUrl {
                url: self.ollama_base_url.clone(),
                llm_body: llm_body.clone(),
            }, self.logger.clone())
            .await
        }
    }

    // Helper function for Cohere
    async fn get_cohere_response(&self, mut llm_body: LlmBody) -> LlmResponse {
        let function_name = "get_cohere_response";
        self.logger.info("Using Cohere to respond", function_name);
        llm_body.model = self.cohere_model.clone();
        Cohere::get_message(MessageRequest::WithToken {
            llm_body,
            token: self.cohere_token.clone(),
        }, self.logger.clone())
        .await
    }

    // Helper function for OpenAI
    async fn get_openai_response(&self, mut llm_body: LlmBody) -> LlmResponse {
        let function_name = "get_openai_response";
        self.logger.info("Using OpenAI to respond", function_name);
        llm_body.model = self.openai_model.clone();
        OpenAI::get_message(MessageRequest::WithToken {
            llm_body,
            token: self.openai_token.clone(),
        }, self.logger.clone())
        .await
    }

    /// This function will format the prompt like: `role: message`
    fn format_into_prompt(&self, conversation: Conversation) -> Vec<LlmMessage> {
        let function_name = "format_into_prompt";

        let mut messages: Vec<LlmMessage> = Vec::new();

        let system_message = LlmMessage {
            role: "system".to_string(),
            content: self.read_system_message(),
        };

        messages.push(system_message);

        for message in conversation.messages {
            messages.push(message.clone());
            self.logger.debug(
                format!("{}: {}", message.role, message.content).as_str(),
                function_name,
            );
        }

        messages
    }

    /// This function will crop a string to a set limit in case the message is too long
    ///
    /// # Arguments
    ///
    /// * `input_string` - The string of the message to be cropped
    /// * `limit` - The max length that the message will crop to
    fn crop_string(&self, input_string: &str, limit: usize) -> String {
        // Use char_indices to ensure we don't split a multi-byte character
        let mut end_index = input_string.chars().count();

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
    async fn get_llm_message(&self, msg: &Message) -> String {
        let function_name = "get_ollama_message";

        if self.ollama_base_url.is_empty() {
            self.logger
                .error("Ollama url has not been set", function_name, Severity::High);
            return "Something went wrong 😭".to_string();
        }

        let mut conversation = self.load_conversation();

        conversation.add_message(
            format!("{}: {}", msg.author, msg.content),
            "user".to_string(),
            self.max_stored_messages,
        );

        let prompt_data = LlmBody {
            model: self.ollama_model.clone(),
            messages: self.format_into_prompt(conversation.clone()),
            stream: false,
        };

        let ollama_response = self.get_llm_message_based_on_settings(prompt_data).await;

        conversation.add_message(
            ollama_response.message.content.clone(),
            ollama_response.message.role,
            self.max_stored_messages,
        );
        self.save_conversation(conversation);
        let response = self.crop_string(&ollama_response.message.content, 1950);

        self.logger
            .debug(format!("Reponse: {}", response).as_str(), function_name);
        response
    }

    /// This function removes special chars
    ///
    /// # Arguments
    ///
    /// * `input` - The string you want to remove the special chars from.
    fn remove_special_chars(&self, mut input: String) -> String {
        let special_chars: Vec<char> = "\\,.!/()@#$%^&*(){{|?'\"<>-+=:;[]}}\n\r ".chars().collect();

        for special_char in special_chars {
            input = input.replace(special_char, "");
        }

        input
    }

    /// Check if message contains any string from `responds_to_vec`
    ///
    /// # Arguments
    ///
    /// * `message` - The string of the message you want to check for matches.
    fn contains_names(&self, message: String) -> bool {
        for respond in &self.responds_to_vec {
            let cleaned_message = self.remove_special_chars(message.clone());
            let cleaned_responds = self.remove_special_chars(respond.clone());

            // Prevent it to respond to everything
            if cleaned_responds.is_empty() {
                continue;
            }

            if respond.starts_with("=") {
                if cleaned_message.clone() == cleaned_responds.clone() {
                    return true;
                } else {
                    continue;
                }
            }

            if cleaned_message.contains(&cleaned_responds) {
                return true;
            }
        }

        false
    }

    /// Clears the conversation by removing the conversation json file
    pub fn clear_conversation(&self) -> bool {
        let function_name = "clear_conversation";
        let dir_path = Path::new(&self.out_dir);

        let conversation_file = dir_path.join(self.conversation_file.clone());

        self.logger.debug(
            format!("{:?}", conversation_file.as_os_str()).as_str(),
            function_name,
        );

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
                true
            }
            Err(why) => {
                self.logger.error(
                    format!("Could not delete conversation: {}", why).as_str(),
                    function_name,
                    Severity::Medium,
                );
                false
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

        self.logger.debug(
            format!("The message was formatted: {}", message).as_str(),
            function_name,
        );

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
                .say(&ctx.http, self.get_llm_message(msg).await)
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
        self.clear_conversation()
    }
}
