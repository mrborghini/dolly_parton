use serde::{Deserialize, Serialize};

use super::{LogType, Severity};

#[derive(Deserialize, Serialize)]
pub struct Log {
    pub application_name: String,
    pub severity: Severity,
    pub log_type: LogType,
    pub time: u64,
    pub message: String,
    pub type_name: String,
    pub function_name: String,
    pub app_version: String,
}
