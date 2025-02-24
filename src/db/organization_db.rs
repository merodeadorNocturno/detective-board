// detective-board/src/db/organization_db.rs
use crate::{
    app_settings::env::set_environment_variable, models::organization_model::*, utils::crud::*,
};
use actix_web::web::Data;
use async_trait::async_trait;
use lazy_static::lazy_static;

use super::config::Database;

lazy_static! {
    static ref ORGANIZATION_TABLE: String = {
        let value = set_environment_variable("ORGANIZATION_TABLE", "organization");
        value.leak().to_string()
    };
}

#[async_trait]
pub trait OrganizationDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Organization>>;
}

#[async_trait]
impl OrganizationDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Organization>> {
        util_find_all(db, &ORGANIZATION_TABLE).await
    }
}
