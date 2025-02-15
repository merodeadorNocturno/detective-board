use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedTo {
    #[serde(rename = "in")]
    pub in_id: RecordLink,
    #[serde(rename = "out")]
    pub out_id: RecordLink,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordLink {
    Event(RecordId),
    Evidence(RecordId),
    Location(RecordId),
    Organization(RecordId),
    Person(RecordId),
}
