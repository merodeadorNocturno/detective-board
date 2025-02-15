use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Organization {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
