use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dtos::requests::bodies::UpdateLocationBody;

pub async fn update_location(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateLocationBody>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "UPDATE location
        SET 
            name = COALESCE($1, name),
            latitude = COALESCE($2, latitude),
            longitude = COALESCE($3, longitude),
            street_number = COALESCE($4, street_number),
            street_name = COALESCE($5, street_name),
            city = COALESCE($6, city),
            state = COALESCE($7, state),
            country = COALESCE($8, country),
            postal_code = COALESCE($9, postal_code),
            bounding_box = COALESCE($10, bounding_box),
            time_zone = COALESCE($11, time_zone),
            description = COALESCE($12, description),
            is_active = COALESCE($13, is_active),
            is_public = COALESCE($14, is_public),
            notes = COALESCE($15, notes),
            updated_at = NOW()
        WHERE location_id = $16
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
        payload.is_active,
        payload.is_public,
        payload.notes,
        id
    )
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(location)) => (StatusCode::OK, Json(location)),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Location not found")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Database error")),
    }
}
