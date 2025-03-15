use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_location(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    tracing::info!("Received request to deactivate location with ID: {}", id);

    let result = sqlx::query!(
        "UPDATE location SET is_active = false, deactivated_at = NOW() WHERE location_id = $1 RETURNING location_id",
        id
    )
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(_)) => {
            tracing::info!("Successfully deactivated location with ID: {}", id);
            (StatusCode::OK, Json("Location deactivated".to_string()))
        }
        Ok(None) => {
            tracing::info!("No location found with ID: {}", id);
            (
                StatusCode::BAD_REQUEST,
                Json(format!("No Location with {} location_id exists", id)),
            )
        }
        Err(err) => {
            tracing::error!("Failed to deactivate location {}: {:?}", id, err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Database error".to_string()),
            )
        }
    }
}
