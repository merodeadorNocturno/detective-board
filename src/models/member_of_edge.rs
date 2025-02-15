use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberOf {
    #[serde(rename = "in")]
    pub in_id: RecordLinkPerson,
    #[serde(rename = "out")]
    pub out_id: RecordLinkOranization,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordLinkOranization {
    Organization(RecordId),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordLinkPerson {
    Person(RecordId),
}
