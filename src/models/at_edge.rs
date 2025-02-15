use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct At {
    #[serde(rename = "in")]
    pub in_id: RecordLinkLocation,
    #[serde(rename = "out")]
    pub out_id: RecordLinkPersonEvent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordLinkPersonEvent {
    Event(RecordId),
    Person(RecordId),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordLinkLocation {
    Location(RecordId),
}
