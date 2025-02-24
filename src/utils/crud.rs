use actix_web::web::Data;
use log::error;

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{Error, RecordId};

use crate::db::config::Database;

// pub async fn util_find_all<T: DeserializeOwned>(
//     db: &Data<Database>,
//     table_name: &str,
// ) -> Option<Vec<T>> {
//     let result = db.client.select(table_name).await;

//     match result {
//         Ok(all_users) => Some(all_users),
//         Err(e) => {
//             error!("Error {}.find_all:: {:?}", &table_name, e);
//             None
//         }
//     }
// }
pub async fn util_find_all<T>(db: &Data<Database>, table_name: &str) -> Option<Vec<T>>
where
    T: DeserializeOwned + Serialize, // Remove Send + Sync + 'static if not strictly necessary
{
    let result: Result<Vec<T>, Error> = db.client.select(table_name).await;

    match result {
        Ok(items) => Some(items),
        Err(e) => {
            error!("Error {}.find_all:: {:?}", &table_name, e);

            // Attempt to query as raw values to debug the response
            if let Ok(mut response) = db
                .client
                .query(format!("SELECT * FROM {}", table_name))
                .await
            {
                if let Ok(raw_values) = response.take::<Vec<serde_json::Value>>(0) {
                    eprintln!("Raw values from {}: {:?}", table_name, raw_values);
                }
            }
            None
        }
    }
}

pub async fn util_find_one<T: DeserializeOwned>(
    db: &Data<Database>,
    uuid: String,
    table_name: &str,
) -> Option<T> {
    let t_by_uuid: Result<Option<T>, Error> = db.client.select((table_name, uuid)).await;

    match t_by_uuid {
        Ok(uuid_t) => uuid_t,
        Err(e) => {
            error!("Error {}.find_one:: {:?}", &table_name, e);
            None
        }
    }
}

pub async fn util_add_one<T>(db: &Data<Database>, t: T, id: RecordId, table_name: &str) -> Option<T>
where
    T: DeserializeOwned + Serialize + Send + Sync + 'static,
{
    let created_t = db
        .client
        .create((table_name, id.to_string()))
        .content(t)
        .await;

    match created_t {
        Ok(t_record) => t_record,
        Err(e) => {
            error!("Error {}.add_one:: {:?}", table_name, e);
            None
        }
    }
}

pub async fn util_update_one<T: DeserializeOwned + Serialize>(
    db: &Data<Database>,
    t: T,
    uuid: RecordId,
    table_name: &str,
) -> Option<T>
where
    T: DeserializeOwned + Serialize + Send + Sync + 'static,
{
    let t_id = uuid.clone();
    let t_to_update: Result<Option<T>, Error> =
        db.client.select((table_name, &t_id.to_string())).await;

    match t_to_update {
        Ok(found_t) => match found_t {
            Some(_t) => {
                let updated_t: Result<Option<T>, Error> = db
                    .client
                    .update((table_name, &t_id.to_string()))
                    .merge(t)
                    .await;

                match updated_t {
                    Ok(updated_t_values) => updated_t_values,
                    Err(e) => {
                        error!("Error {}.find_one:: {:?}", table_name, e);
                        None
                    }
                }
            }
            None => None,
        },
        Err(e) => {
            error!("Error {}: {:?}", table_name, e);
            None
        }
    }
}

pub async fn util_find_all_non_deleted<T: DeserializeOwned + Serialize>(
    db: &Data<Database>,
    table_name: &str,
) -> Option<Vec<T>> {
    let surreal_query = format!("SELECT * FROM {}", table_name);

    let query_t_result = db.client.query(surreal_query).await;

    match query_t_result {
        Ok(mut response) => match response.take(0) {
            Ok(deleted_t_records) => Some(deleted_t_records),
            Err(e) => {
                error!(
                    "Failed to retrieve active records from {}:: {}",
                    table_name, e
                );
                None
            }
        },
        Err(e) => {
            error!(
                "Failed to retrieve active records from {}:: {}",
                table_name, e
            );
            None
        }
    }
}
