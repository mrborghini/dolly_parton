use super::Logger;
use std::{collections::HashMap, env, fs::read_to_string};

pub struct DotEnvReader {
    logger: Logger,
    file_name: String,
}

impl DotEnvReader {
    pub fn new<S: AsRef<str>>(file_name: S) -> Self {
        let logger = Logger::new("DotEnvReader");
        let file_name = file_name.as_ref().to_string();

        Self { logger, file_name }
    }

    pub fn parse_and_set_env(&self) {
        let func_name = "parse_and_set_env";

        // Read the file content
        let file_content = self.read_file_content();
        self.logger.debug(
            format!("Read file: {}", self.file_name.clone()).as_str(),
            func_name,
        );

        // Parse the file content into a HashMap of key-value pairs
        let env_vars = self.parse_lines(file_content);
        self.logger.debug("Parsed variables", func_name);

        // Set environment variables
        self.set_env_vars(env_vars);
        self.logger.debug("Set enviroment variables", func_name)
    }

    fn read_file_content(&self) -> String {
        match read_to_string(&self.file_name) {
            Ok(content) => content,
            Err(e) => {
                self.logger.debug(
                    format!("Failed to read file: {}", e).as_str(),
                    "read_file_content",
                );
                return "".to_string();
            }
        }
    }

    fn parse_lines(&self, content: String) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();
        let func_name = "parse_lines";

        for line in content.lines() {
            let trimmed_line = line.trim();

            // Skip empty lines and comments
            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = self.parse_key_value(trimmed_line) {
                self.logger.debug(
                    format!("Parsed key-value pair: {}={}", key, value).as_str(),
                    func_name,
                );
                env_vars.insert(key, value);
            }
        }

        env_vars
    }

    fn parse_key_value(&self, line: &str) -> Option<(String, String)> {
        let mut split = line.splitn(2, '=');
        let key = split.next()?;
        let value = split.next().unwrap_or("").trim().to_string();
        Some((key.trim().to_string(), value))
    }

    fn set_env_vars(&self, env_vars: HashMap<String, String>) {
        let func_name = "set_env_vars";

        for (key, value) in env_vars {
            // Check if the environment variable is already set
            if env::var(&key).is_ok() {
                self.logger.debug(
                    format!("Environment variable {} is already set, skipping.", key).as_str(),
                    func_name,
                );
            } else {
                self.logger.debug(
                    format!("Setting environment variable: {}={}", key, value).as_str(),
                    func_name,
                );
                env::set_var(&key, &value);
            }
        }
    }
}
