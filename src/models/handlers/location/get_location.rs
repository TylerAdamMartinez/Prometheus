use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{dtos::responses::LocationDTO, models::Location};

pub async fn get_location(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    tracing::info!("Received GET request for location_id: {}", id);

    let record = match sqlx::query_as!(
        Location,
        r#"
        SELECT 
            location_id, name, latitude, longitude, altitude, 
            street_number, street_name, city, state, country, postal_code, 
            ST_AsText(bounding_box) AS "bounding_box?", 
            ST_AsText(location) AS location, 
            time_zone, created_at, updated_at, description, is_active, 
            deactivated_at, is_public, notes
        FROM location
        WHERE location_id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(record)) => {
            tracing::info!("Location found: {:?}", record.location_id);
            record
        }
        Ok(None) => {
            tracing::info!("Location ID {} not found", id);
            return StatusCode::NO_CONTENT.into_response();
        }
        Err(e) => {
            tracing::error!("Database error for ID {}: {:?}", id, e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if let Some(deactivated_at) = record.deactivated_at {
        tracing::info!(
            "Location ID {} is no longer in use, deactivated at: {}",
            id,
            deactivated_at
        );
        return (
            StatusCode::OK,
            Json(format!(
                "This location is no longer in use and was deactivated at {}",
                deactivated_at
            )),
        )
            .into_response();
    }

    tracing::info!("Returning location data for ID: {}", record.location_id);
    Json(LocationDTO::from(&record)).into_response()
}
