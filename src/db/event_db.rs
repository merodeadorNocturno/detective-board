// detective-board/src/db/event_db.rs
use crate::{app_settings::env::set_environment_variable, models::event_model::*, utils::crud::*};
use actix_web::web::Data;
use async_trait::async_trait;
use lazy_static::lazy_static;

use super::config::Database;

lazy_static! {
    static ref EVENT_TABLE: String = {
        let value = set_environment_variable("EVENT_TABLE", "event");
        value.leak().to_string()
    };
}

#[async_trait]
pub trait EventDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Event>>;
}

#[async_trait]
impl EventDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Event>> {
        util_find_all(db, &EVENT_TABLE).await
    }
}
