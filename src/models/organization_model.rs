// detective-board/src/models/organization_model.rs
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Deserialize, Serialize, Debug)]
pub struct Organization {
    pub id: RecordId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrganizationJson {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
