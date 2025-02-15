use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct Name {
    pub first: Cow<'static, str>,
    pub last: Cow<'static, str>,
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub description: Option<String>,
    pub date_of_birt: Option<String>,
    pub address: Option<String>,
}
