use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::found_at_edge::RecordLinkEvidence;

#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedEvidence {
    #[serde(rename = "in")]
    pub in_id: RecordLinkPersonEventOrganization,
    #[serde(rename = "out")]
    pub out_id: RecordLinkEvidence,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordLinkPersonEventOrganization {
    Evidence(RecordId),
    Person(RecordId),
    Organization(RecordId),
}
