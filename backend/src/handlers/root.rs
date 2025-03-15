use axum::{Json, http::StatusCode};

pub async fn root() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({"message": "Howdy, y'all"})),
    )
}
