use axum::{extract::State, Json};
use http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Location;

pub async fn get_location(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<Location>, StatusCode> {
    let location = sqlx::query_as!(Location, "SELECT * FROM locations WHERE location_id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NO_CONTENT)?;

    Ok(Json(location))
}
