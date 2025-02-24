// detective-board/src/db/evidence_db.rs
use crate::{
    app_settings::env::set_environment_variable, models::evidence_model::*, utils::crud::*,
};
use actix_web::web::Data;
use async_trait::async_trait;
use lazy_static::lazy_static;

use super::config::Database;

lazy_static! {
    static ref EVIDENCE_TABLE: String = {
        let value = set_environment_variable("EVIDENCE_TABLE", "evidence");
        value.leak().to_string()
    };
}

#[async_trait]
pub trait EvidenceDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Evidence>>;
}

#[async_trait]
impl EvidenceDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Evidence>> {
        util_find_all(db, &EVIDENCE_TABLE).await
    }
}
