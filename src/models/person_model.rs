use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use surrealdb::{sql::Datetime, RecordId};

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub first: Cow<'static, str>,
    pub last: Cow<'static, str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: RecordId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub description: Option<String>,
    pub date_of_birth: Option<Datetime>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonJson {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub description: Option<String>,
    pub date_of_birth: Option<Datetime>,
    pub address: Option<String>,
}
