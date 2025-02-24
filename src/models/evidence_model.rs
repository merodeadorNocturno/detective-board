use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct Evidence {
    pub id: RecordId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub evidence_type: String,
    pub location: Option<RecordId>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EvidenceJson {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub evidence_type: String,
    pub location: Option<String>,
}
