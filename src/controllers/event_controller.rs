// detective-board/src/controllers/event_controller.rs
use actix_web::{
    get,
    http::StatusCode,
    web::{Data, ServiceConfig},
    HttpResponse,
};
use log::error;

use crate::{
    db::{config::Database, event_db::*},
    error::event_error::EventError,
    models::event_model::*,
};

#[get("/events")]
#[tracing::instrument(name = "Retrieve all events", skip(db))]
async fn find_all(db: Data<Database>) -> Result<HttpResponse, EventError> {
    let events = Database::find_all(&db).await;

    match events {
        Some(found_events) => {
            let events_json: Vec<EventJson> = found_events
                .into_iter()
                .map(|e| EventJson {
                    id: e.id.to_string(),
                    name: e.name,
                    description: e.description,
                })
                .collect();
            Ok(HttpResponse::Ok().status(StatusCode::OK).json(events_json))
        }
        None => {
            error!("Didn't find any Event data");
            Err(EventError::NoEventFound)
        }
    }
}

pub fn event_html_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
}
