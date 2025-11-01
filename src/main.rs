#![allow(unused)]

mod api;

// For beginning only
//
use axum::{
    response::Html,
    routing::get,
    routing::post,
    Json,
    Router,
};
use api::api::{NameRequest, NameResponse, test_handler, name_handler};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use futures::executor::block_on;
use sea_orm::{Database, DbErr};
const DATABASE_URL: &str = "postgresql://rolanveroncruz:@localhost:5432/dnc";
const DB_NAME: &str = "dnc_backend";
async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
    println!("Connected to database");
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

// }
    

