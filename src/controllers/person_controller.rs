use std::io::ErrorKind;

use actix_web::{
    delete,
    error::HttpError,
    get,
    http::StatusCode,
    patch, post,
    web::{Data, Json, Path, ServiceConfig},
    HttpRequest, HttpResponse,
};
use chrono::Local;
use log::{error, info};
use serde_json::json;
// use validator::Validate;

use crate::{
    db::{config::Database, person_db::PersonDB},
    error::person_error::PersonError,
    models::person_model::*,
};

#[get("/persons")]
#[tracing::instrument(name = "Retrieve all persons", skip(db))]
async fn find_all(db: Data<Database>) -> Result<HttpResponse, PersonError> {
    let person = Database::find_all(&db).await;

    match person {
        Some(found_persons) => {
            let mut person_vec: Vec<PersonJson> = Vec::new();
            for this_person in &found_persons {
                let this_person_json = PersonJson {
                    id: this_person.id.clone().to_string(),
                    name: this_person.name.clone(),
                    alias: this_person.alias.clone(),
                    description: this_person.description.clone(),
                    date_of_birth: this_person.date_of_birth.clone(),
                    address: this_person.address.clone(),
                };

                person_vec.push(this_person_json);
            }
            Ok(HttpResponse::Ok().status(StatusCode::OK).json(person_vec))
        }
        None => {
            error!("Didn't find any User data");
            Ok(HttpResponse::InternalServerError().json(PersonId {
                id: format!("{}", PersonError::NoPersonFound),
            }))
        }
    }
}

pub fn person_html_controllers(cfg: &mut ServiceConfig) {
    cfg.service(find_all);
}
