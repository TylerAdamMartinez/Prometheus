use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dtos::{requests::bodies::UpdateLocationBody, responses::LocationDTO},
    models::Location,
};

pub async fn update_location(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateLocationBody>,
) -> impl IntoResponse {
    tracing::info!("Received request to update location ID: {}", id);

    let record = sqlx::query_as!(
        Location,
        r#"
        UPDATE location
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
            bounding_box = COALESCE(ST_GeomFromText($10, 4326), bounding_box), -- Convert WKT to Geometry 
            location = COALESCE(ST_GeomFromText($11, 4326), location),         -- Convert WKT to Geometry 
            time_zone = COALESCE($12, time_zone),
            description = COALESCE($13, description),
            is_active = COALESCE($14, is_active),
            is_public = COALESCE($15, is_public),
            notes = COALESCE($16, notes),
            updated_at = NOW()
        WHERE location_id = $17
        RETURNING 
            location_id, name, latitude, longitude, altitude, 
            street_number, street_name, city, state, country, postal_code, 
            ST_AsText(bounding_box) AS bounding_box,  -- Convert Geometry to WKT
            ST_AsText(location) AS location,          -- Convert Geometry to WKT
            time_zone, created_at, updated_at, description, is_active, 
            deactivated_at, is_public, notes
        "#,
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
        payload.location,
        payload.time_zone,
        payload.description,
        payload.is_active,
        payload.is_public,
        payload.notes,
        id
    )
    .fetch_optional(&pool)
    .await;

    match record {
        Ok(Some(record)) => {
            tracing::info!("Successfully updated location ID: {}", id);
            (StatusCode::OK, Json(LocationDTO::from(&record))).into_response()
        }
        Ok(None) => {
            tracing::info!("Location ID {} not found for update", id);
            (StatusCode::NOT_FOUND, Json("Location not found")).into_response()
        }
        Err(err) => {
            tracing::error!("Database error while updating location {}: {:?}", id, err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Database error")).into_response()
        }
    }
}
