use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use tokio::fs;

// mods
mod app_settings;
mod controllers;
mod db;
mod error;
mod models;
mod utils;

// local crates
use crate::{
    controllers::{
        event_controller::event_html_controllers, evidence_controller::evidence_html_controllers,
        location_controller::location_html_controllers,
        organization_controller::organization_html_controllers,
        person_controller::person_html_controllers,
    },
    db::config::Database,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // DB connection
    let sdb = match Database::init().await {
        Ok(db_running) => db_running,
        Err(e) => {
            eprintln!("Error connecting to db: {:?}", e);
            return Ok(());
        }
    };

    // healthcheck
    println!("{:?}", &sdb.client.health());

    // Read DB Schema from board_schema.surrealql
    let schema_path = "src/db_schemas/board_schema.surrealql";
    let schema_content = match fs::read_to_string(schema_path).await {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading schema file: {}", e);
            return Err(e);
        }
    };

    // Create Schema on DB
    let create_schema_result = sdb.client.query(schema_content).await;
    match create_schema_result {
        Ok(_) => println!("Schema applied successfully"),
        Err(e) => {
            eprintln!("Error applying schema: {}", e);
        }
    }

    // Add Surreal<Client> to Datawrapper
    let sdb_data = Data::new(sdb);

    // Start http server
    HttpServer::new(move || {
        // Set CORS
        let cors = Cors::permissive().max_age(3600);

        // AppData, Wrappers, Middleware, Endpoints
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(cors)
            .app_data(sdb_data.clone())
            .configure(person_html_controllers)
            .configure(event_html_controllers)
            .configure(evidence_html_controllers)
            .configure(location_html_controllers)
            .configure(organization_html_controllers)
    })
    .bind("0.0.0.0:8080")
    .expect("FAILED TO BIND TO PORT")
    .run()
    .await
}
