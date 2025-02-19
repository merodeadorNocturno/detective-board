use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Error as SurrealError, Surreal,
};

use crate::app_settings::env::set_environment_variable;

#[derive(Debug)]
pub struct Database {
    pub client: Surreal<Client>,
    pub database: String,
    pub namespace: String,
    pub username: String,
    pub password: String,
}

impl Database {
    pub async fn init() -> Result<Self, SurrealError> {
        let db_address: String = set_environment_variable("DB_ADDRESS", "0.0.0.0:8000");
        let namespace: String = set_environment_variable("DB_NAMESPACE", "detective");
        let database: String = set_environment_variable("DB_NAME", "board");
        let username: &str = &set_environment_variable("USER_NAME", "detective");
        let password: &str = &set_environment_variable("USER_PASSWORD", "board_password");

        let client = Surreal::new::<Ws>(db_address).await?;

        client.signin(Root { username, password }).await?;

        client.use_ns(&namespace).use_db(&database).await.unwrap();

        Ok(Database {
            client,
            database,
            namespace,
            username: String::from(username),
            password: String::from(password),
        })
    }
}
