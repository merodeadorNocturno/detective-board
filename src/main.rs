use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Error as SurrealError, Surreal};

// mods
mod models;

#[tokio::main]
async fn main() -> Result<(), SurrealError> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "detective",
        password: "board_password",
    })
    .await?;

    db.use_ns("detective_board").use_db("detective_db");

    Ok(())
}
