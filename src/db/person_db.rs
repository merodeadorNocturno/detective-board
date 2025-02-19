use actix_web::web::Data;
use async_trait::async_trait;
use chrono::Local;
use lazy_static::lazy_static;
use log::error;
use surrealdb::{opt::PatchOp, Error as SurrealDBError};

use crate::{
    app_settings::env::set_environment_variable, db::config::Database, models::person_model::*,
    utils::crud::*,
};

lazy_static! {
    static ref PERSON_TABLE: String = {
        let value = set_environment_variable("PERSON_TABLE", "person");
        value.leak().to_string()
    };
}

#[async_trait]
pub trait PersonDB {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Person>>;
    async fn find_one(db: &Data<Database>, id: String) -> Option<Person>;
    async fn add_one(db: &Data<Database>, new_person: Person) -> Option<Person>;
    async fn update_one(db: &Data<Database>, person: Person) -> Option<Person>;
    async fn delete_one(db: &Data<Database>, id: String) -> Option<Person>;
    async fn find_all_non_deleted(db: &Data<Database>) -> Option<Vec<Person>>;
    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<Person>>;
}

#[async_trait]
impl PersonDB for Database {
    async fn find_all(db: &Data<Database>) -> Option<Vec<Person>> {
        util_find_all(db, &PERSON_TABLE).await
    }

    async fn find_one(db: &Data<Database>, uuid: String) -> Option<Person> {
        util_find_one(db, uuid, &PERSON_TABLE).await
    }

    async fn add_one(db: &Data<Database>, new_person: Person) -> Option<Person> {
        let id = new_person.id.clone();

        util_add_one(db, new_person, id, "&PERSON_TABLE").await
    }

    async fn update_one(db: &Data<Database>, person: Person) -> Option<Person> {
        let id = person.id.clone();
        util_update_one(db, person, id, &PERSON_TABLE).await
    }

    async fn delete_one(db: &Data<Database>, id: String) -> Option<Person> {
        let person_table = format!("{}", PERSON_TABLE.clone());
        let person_exists: Result<Option<Person>, SurrealDBError> =
            db.client.select((person_table.clone(), id.clone())).await;

        if let Ok(Some(_)) = person_exists {
            let person: Result<Option<Person>, SurrealDBError> = db
                .client
                .update((person_table, id))
                .patch(PatchOp::replace("/deleted", true))
                .patch(PatchOp::replace("/date_modified", Local::now()))
                .await;

            match person {
                Ok(d_person) => match d_person {
                    Some(deleted_person) => Some(deleted_person),
                    None => None,
                },
                Err(e) => {
                    error!("Failed to update person {}", e);
                    None
                }
            }
        } else {
            None
        }
    }

    async fn find_all_non_deleted(db: &Data<Database>) -> Option<Vec<Person>> {
        util_find_all_non_deleted(&db, &PERSON_TABLE).await
    }

    async fn find_all_deleted(db: &Data<Database>) -> Option<Vec<Person>> {
        let person_table = format!("{}", PERSON_TABLE.clone());
        let surreal_query = format!("SELECT * FROM {} WHERE deleted = true", &person_table);

        let person = db.client.query(surreal_query).await;

        match person {
            Ok(mut response) => match response.take(0) {
                Ok(deleted_person) => Some(deleted_person),
                Err(e) => {
                    error!("Failed to retrieve deleted persons {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Failed to retrieve deleted persons {}", e);
                None
            }
        }
    }
}
