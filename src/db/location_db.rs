// detective-board/src/db/location_db.rs
use crate::{
    app_settings::env::set_environment_variable, models::location_model::*, utils::crud::*,
};
use actix_web::web::Data;
use async_trait::async_trait;
use lazy_static::lazy_static;

use super::config::Database;

lazy_static! {
    static ref LOCATION_TABLE: String = {
        let value = set_environment_variable("LOCATION_TABLE", "location");
        value.leak().to_string()
    };
}

#[async_trait]
pub trait LocationDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Location>>;
}

#[async_trait]
impl LocationDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Location>> {
        util_find_all(db, &LOCATION_TABLE).await
    }
}
