#![allow(unused)] // For beginning only
//
use axum::{
    response::Html,
    routing::get,
    routing::post,
    Json,
    Router,
};

use tokio::net::TcpListener;
use std::net::SocketAddr;

use serde::{
    Deserialize, 
    Serialize,
};

// ---- Data Structures ----
#[derive(Deserialize, Serialize)]
   struct NameRequest{
    first_name: String,
    last_name: String,
   }
#[derive(Deserialize, Serialize)]
    struct NameResponse{
     full_name: String,
    }
#[derive(Deserialize, Serialize)]
    struct TestResponse{
     message: String,
    }

// ---- Handlers ----
async fn test_handler() ->Json<TestResponse> {
    let response = TestResponse {
        message: "Hello, World!".to_string(),
    };
    Json(response)

}
async fn name_handler(
    Json(payload): Json<NameRequest>,) -> Json<NameResponse> {
    let full_name = format!("{} {}", payload.first_name, payload.last_name);
    let resonse = NameResponse { full_name };
    Json(resonse)
}



#[tokio::main]
async fn main() {
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
    

