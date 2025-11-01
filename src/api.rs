
pub mod api {
    use axum::Json;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct NameRequest{
        first_name: String,
        last_name: String,
    }
    #[derive(Deserialize, Serialize)]
    pub struct NameResponse{
        full_name: String,
    }
    #[derive(Deserialize, Serialize)]
    pub struct TestResponse{
        message: String,
    }

    pub async fn name_handler(
        Json(payload): Json<NameRequest>,) -> Json<NameResponse> {
        let full_name = format!("{} {}", payload.first_name, payload.last_name);
        let resonse = NameResponse { full_name };
        Json(resonse)
    }
    pub async fn test_handler() ->Json<TestResponse> {
        let response = TestResponse {
            message: "Hello, World!".to_string(),
        };
        Json(response)

    }

}