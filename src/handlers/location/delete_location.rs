use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn delete_location(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "UPDATE location SET is_active = false, deactivated_at = NOW() WHERE location_id = $1 RETURNING location_id",
        id
    )
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(_)) => (StatusCode::OK, Json("Location deactivated")),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Location not found")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Database error")),
    }
}
