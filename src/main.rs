mod entities;
mod api;
mod setup_role;
mod setup_user;

use axum::{
    response::Html,
    routing::get,
    routing::post,
    Router,
};
use api::api::{  test_handler, name_handler};
use tokio::net::TcpListener;
use sea_orm::{Database, DatabaseConnection, DbErr};

use setup_role::setup_role::insert_unique_admin_role;
use crate::setup_user::setup_user::insert_admin_user;

const DATABASE_URL: &str = "postgresql://rolanveroncruz:1lap2ace@localhost:5432/dnc";
async fn connect_to_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

#[tokio::main]
async fn main() {
    // 1. Connect to the database.
    let db_result = connect_to_db().await;
    if let Err(err) = db_result {
        panic!("Failed to connect to database: {}", err);
    }
    let db= db_result.unwrap();
    println!("Connected to database");

    // 2. Insert an admin role if it doesn't exist.
    let insert_admin_role_result = insert_unique_admin_role(&db).await;
    if let Err(err) = insert_admin_role_result {
        panic!("Failed to insert admin role: {}", err);
    }
    println!("Admin role in database.");

    // 3. Insert an admin user if it doesn't exist.
    let admin_user_id = insert_admin_user(&db, insert_admin_role_result.unwrap()).await;
    if let Err(err) = admin_user_id {
        panic!("Failed to insert admin user: {}", err);
    }
    println!("Admin user in database.");

    // Build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Welcome to the Axum server!" }))
        .route(
        "/hello",
        get(|| async { Html("Hello, <strong>World</strong>!") }))
            .route("/test", get(test_handler))
            .route("/name", post(name_handler));

    // run the app with hyper on port 3000
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000/hello");
    axum::serve(listener, app).await.unwrap(); 
}



