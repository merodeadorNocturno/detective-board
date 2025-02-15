use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::at_edge::RecordLinkLocation;

#[derive(Debug, Deserialize, Serialize)]
pub struct FoundAt {
    #[serde(rename = "in")]
    pub in_id: RecordLinkLocation,
    #[serde(rename = "out")]
    pub out_id: RecordLinkEvidence,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordLinkEvidence {
    Evidence(RecordId),
}
