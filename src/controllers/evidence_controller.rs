// detective-board/src/controllers/evidence_controller.rs
use actix_web::{
    get,
    http::StatusCode,
    web::{Data, ServiceConfig},
    HttpResponse,
};
use log::error;

use crate::{
    db::{config::Database, evidence_db::*},
    error::evidence_error::EvidenceError,
    models::evidence_model::*,
};

#[get("/evidences")]
#[tracing::instrument(name = "Retrieve all evidences", skip(db))]
async fn find_all(db: Data<Database>) -> Result<HttpResponse, EvidenceError> {
    let evidences = Database::find_all(&db).await;

    match evidences {
        Some(found_evidences) => {
            let evidences_json: Vec<EvidenceJson> = found_evidences
                .into_iter()
                .map(|e| EvidenceJson {
                    id: e.id.to_string(),
                    name: e.name,
                    description: e.description,
                    evidence_type: e.evidence_type,
                    location: e.location.map(|loc_id| loc_id.to_string()),
                })
                .collect();
            Ok(HttpResponse::Ok()
                .status(StatusCode::OK)
                .json(evidences_json))
        }
        None => {
            error!("Didn't find any Evidence data");
            Err(EvidenceError::NoEvidenceFound)
        }
    }
}

pub fn evidence_html_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
}
