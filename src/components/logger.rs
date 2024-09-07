use std::{
    env, fs::{self, OpenOptions}, io::Write, path::Path, time::{SystemTime, UNIX_EPOCH}
};

use chrono::{Local, TimeZone, Utc};

use super::types::{Colors, Log, LogType, Severity};

/// The core struct for the logger.
pub struct Logger {
    type_name: String,
}

impl Logger {
    /// Constructor
    ///
    /// # Arguments
    ///
    /// * `type_name` The struct/trait name.
    pub fn new<S: AsRef<str>>(type_name: S) -> Self {
        let type_name = type_name.as_ref().to_string();
        Self { type_name }
    }

    /// This function will log and write based on the environment.
    ///
    ///  # Arguments
    ///
    /// * `info` - All the info about the log that is being created.
    fn log(&self, mut info: Log) {
        info.application_name = env!("CARGO_PKG_NAME").to_string();
        info.app_version = env!("CARGO_PKG_VERSION").to_string();

        info.time = self.get_time();

        let message = format!(
            "[{} - {} - (v{}) ({} - {} - {}) - {}]: {}",
            info.log_type,
            info.severity,
            info.app_version,
            info.application_name,
            info.type_name,
            info.function_name,
            self.format_time(info.time),
            info.message
        );

        // Print the message
        match info.log_type {
            LogType::Info => println!("{}{}{}", Colors::ok_blue(), message, Colors::normal()),
            LogType::Debug => match env::var("LOGGER_DEBUG") {
                Ok(value) => {
                    if value.to_lowercase() == "true" {
                        println!("{}{}{}", Colors::ok_green(), message, Colors::normal());
                    } else {
                        return;
                    }
                }
                Err(_) => {
                    return;
                }
            },
            LogType::Warning => println!("{}{}{}", Colors::warning(), message, Colors::normal()),
            LogType::Error => println!("{}{}{}", Colors::error(), message, Colors::normal()),
        }

        // Write to file
        match env::var("WRITE_LOGS") {
            Ok(value) => {
                if value.to_lowercase() == "true" {
                    self.write_log(message.clone());
                }
            }
            Err(_) => {}
        }
    }

    /// This function will always log important info to the console.
    ///
    /// # Arguments
    ///
    /// * `message` - The info you want to log.
    /// * `function_name` - The name of the function that you're logging from.
    pub fn info<S: AsRef<str>>(&self, message: S, function_name: S) {
        let message = message.as_ref().to_string();
        let function_name = function_name.as_ref().to_string();

        let info = Log::new(
            "".to_string(),
            Severity::None,
            LogType::Info,
            0,
            message,
            self.type_name.clone(),
            function_name,
            "".to_string(),
        );
        self.log(info);
    }

    /// This function will log debug info that is not really important, but can help find bugs.
    ///
    /// The messages can be hidden by adding `LOGGER_DEBUG=false` to your environment.
    ///
    /// # Arguments
    ///
    /// * `message` - The debug info you want to log.
    /// * `function_name` - The name of the function that you're logging from.
    pub fn debug<S: AsRef<str>>(&self, message: S, function_name: S) {
        let message = message.as_ref().to_string();
        let function_name = function_name.as_ref().to_string();

        let info = Log::new(
            "".to_string(),
            Severity::None,
            LogType::Debug,
            0,
            message,
            self.type_name.clone(),
            function_name,
            "".to_string(),
        );
        self.log(info);
    }

    /// This function will log warnings that are not very severe, but important.
    ///
    /// # Arguments
    ///
    /// * `message` - The warning you want to log.
    /// * `function_name` - The name of the function that you're logging from.
    /// * `severity` - The severity of the warning.
    pub fn warning<S: AsRef<str>>(&self, message: S, function_name: S, severity: Severity) {
        let message = message.as_ref().to_string();
        let function_name = function_name.as_ref().to_string();

        let info = Log::new(
            "".to_string(),
            severity,
            LogType::Warning,
            0,
            message,
            self.type_name.clone(),
            function_name,
            "".to_string(),
        );
        self.log(info);
    }

    /// This function will log errors that can be severe.
    ///
    /// # Arguments
    ///
    /// * `message` - The error you want to log.
    /// * `function_name` - The name of the function that you're logging from.
    /// * `severity` - The severity of the error.
    pub fn error<S: AsRef<str>>(&self, message: S, function_name: S, severity: Severity) {
        let message = message.as_ref().to_string();
        let function_name = function_name.as_ref().to_string();

        let info = Log::new(
            "".to_string(),
            severity,
            LogType::Error,
            0,
            message,
            self.type_name.clone(),
            function_name,
            "".to_string(),
        );
        self.log(info);
    }

    /// This function will get the current unix time stamp in seconds.
    fn get_time(&self) -> u64 {
        let start = SystemTime::now();

        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        return since_the_epoch.as_secs();
    }

    /// This function will format the time into `year-month-day hour:minute:second`.
    ///
    /// # Arguments
    ///
    /// * `unix_time` - the current timestamp in seconds
    fn format_time(&self, unix_time: u64) -> String {
        let utc_datetime = Utc
            .timestamp_opt(unix_time as i64, 0)
            .single()
            .expect("Invalid timestamp");

        let local_datetime = utc_datetime.with_timezone(&Local);

        local_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// This function will write logs to a file.
    ///
    /// You can disable writing to a logfile by adding `WRITE_LOGS=false` to your environment.
    ///
    /// # Arguments
    ///
    /// * `message` - The message that will be written to the file
    fn write_log(&self, message: String) {
        // Get the appropriate newline character based on the OS.
        let new_line = if cfg!(windows) { "\r\n" } else { "\n" };

        // Define the directory path.
        let dir_path = Path::new("out_data");

        // Create the directory if it doesn't exist.
        if !dir_path.exists() {
            if let Err(e) = fs::create_dir_all(dir_path) {
                eprintln!("Failed to create directory: {}", e);
                return;
            }
        }

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(dir_path.join("dolly.log"));

        match file {
            Ok(mut f) => {
                let _ = f.write_all(format!("{}{}", message, new_line).as_bytes());
                drop(f);
            }
            Err(_) => {}
        }
    }
}
