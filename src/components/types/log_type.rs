use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum LogType {
    Info,
    Debug,
    Warning,
    Error,
}

impl fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogType::Info => write!(f, "Info"),
            LogType::Debug => write!(f, "Debug"),
            LogType::Warning => write!(f, "Warning"),
            LogType::Error => write!(f, "Error"),
        }
    }
}
