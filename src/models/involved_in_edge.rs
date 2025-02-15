use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::member_of_edge::RecordLinkPerson;

#[derive(Debug, Deserialize, Serialize)]
pub struct InvolvedIn {
    #[serde(rename = "in")]
    pub in_id: RecordLinkEvent,
    #[serde(rename = "out")]
    pub out_id: RecordLinkPerson,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordLinkEvent {
    Event(RecordId),
}
