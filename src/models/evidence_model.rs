use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct Evidence {
    pub name: String,
    pub evidence_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub location: Option<RecordId>,
}
