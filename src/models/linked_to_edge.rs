use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkedTo {
    #[serde(rename = "in")]
    pub in_id: RecordLinkPersonOrganization,
    #[serde(rename = "out")]
    pub out_id: RecordLinkPersonOrganization,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordLinkPersonOrganization {
    Evidence(RecordId),
    Organization(RecordId),
}
