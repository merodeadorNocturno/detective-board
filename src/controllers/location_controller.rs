// detective-board/src/controllers/location_controller.rs
use actix_web::{
    get,
    http::StatusCode,
    web::{Data, ServiceConfig},
    HttpResponse,
};
use log::error;

use crate::{
    db::{config::Database, location_db::*},
    error::location_error::LocationError,
    models::location_model::*,
};

#[get("/locations")]
#[tracing::instrument(name = "Retrieve all locations", skip(db))]
async fn find_all(db: Data<Database>) -> Result<HttpResponse, LocationError> {
    let locations = Database::find_all(&db).await;
    match locations {
        Some(found_locations) => {
            let locations_json: Vec<LocationJson> = found_locations
                .into_iter()
                .map(|l| LocationJson {
                    id: l.id.to_string(),
                    name: l.name,
                    coordinates: l.coordinates,
                })
                .collect();
            Ok(HttpResponse::Ok()
                .status(StatusCode::OK)
                .json(locations_json))
        }
        None => {
            error!("Didn't find any Location data");
            Err(LocationError::NoLocationFound)
        }
    }
}

pub fn location_html_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
}
