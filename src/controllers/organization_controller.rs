// detective-board/src/controllers/organization_controller.rs
use actix_web::{
    get,
    http::StatusCode,
    web::{Data, ServiceConfig},
    HttpResponse,
};
use log::error;

use crate::{
    db::{config::Database, organization_db::*},
    error::organization_error::OrganizationError,
    models::organization_model::*,
};

#[get("/organizations")]
#[tracing::instrument(name = "Retrieve all organizations", skip(db))]
async fn find_all(db: Data<Database>) -> Result<HttpResponse, OrganizationError> {
    let organizations = Database::find_all(&db).await;

    match organizations {
        Some(found_organizations) => {
            let organizations_json: Vec<OrganizationJson> = found_organizations
                .into_iter()
                .map(|o| OrganizationJson {
                    id: o.id.to_string(),
                    name: o.name,
                    description: o.description,
                })
                .collect();
            Ok(HttpResponse::Ok()
                .status(StatusCode::OK)
                .json(organizations_json))
        }
        None => {
            error!("Didn't find any Organization data");
            Err(OrganizationError::NoOrganizationFound)
        }
    }
}

pub fn organization_html_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
}
