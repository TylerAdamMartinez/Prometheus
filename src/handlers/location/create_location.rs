use axum::{extract::State, Json};
use http::StatusCode;
use sqlx::PgPool;

use crate::{dtos::requests::bodies::CreateLocationBody, models::Location};

pub async fn create_location(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLocationBody>,
) -> Result<Json<Location>, StatusCode> {
    let new_location = sqlx::query_as!(
        Location,
        "INSERT INTO locations (name, latitude, longitude, street_number, street_name, city, state, country, postal_code, bounding_box, time_zone, created_at, description, is_active, is_public, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, now(), $12, true, false, $13)
        RETURNING *",
        payload.name,
        payload.latitude,
        payload.longitude,
        payload.street_number,
        payload.street_name,
        payload.city,
        payload.state,
        payload.country,
        payload.postal_code,
        payload.bounding_box,
        payload.time_zone,
        payload.description,
        payload.notes
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(new_location))
}
