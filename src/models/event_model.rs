use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Event {
    name: String,
    timestamp: DateTime<Local>,
    description: Option<String>,
}
