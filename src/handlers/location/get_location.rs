use axum::{Json, extract::State, handler::HandlerWithoutStateExt, response::IntoResponse};
use http::StatusCode;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    enums::{Country, USAState},
    models::Location,
};

pub async fn get_location(State(pool): State<PgPool>, id: Uuid) -> impl IntoResponse {
    tracing::info!("Received GET request for location_id: {}", id);

    let row = match sqlx::query!(
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
        Ok(Some(row)) => {
            tracing::info!("Location found: {:?}", row.location_id);
            row
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

    if let Some(deactivated_at) = row.deactivated_at {
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

    let state = match row.state.as_deref().map(USAState::from_str) {
        Some(Ok(state)) => Some(state),
        Some(Err(_)) => {
            tracing::error!("Invalid state format in database: {:?}", row.state);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        None => None,
    };

    let country = match Country::from_str(&row.country) {
        Ok(country) => country,
        Err(_) => {
            tracing::error!("Invalid country format in database: {}", row.country);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let location = Location {
        location_id: row.location_id,
        name: row.name,
        latitude: row.latitude,
        longitude: row.longitude,
        altitude: row.altitude,
        street_number: row.street_number,
        street_name: row.street_name,
        city: row.city,
        state,
        country,
        postal_code: row.postal_code,
        bounding_box: row.bounding_box,
        location: row.location,
        time_zone: row.time_zone,
        created_at: row.created_at,
        updated_at: row.updated_at,
        description: row.description,
        is_active: row.is_active,
        deactivated_at: row.deactivated_at,
        is_public: row.is_public,
        notes: row.notes,
    };

    tracing::info!("Returning location data for ID: {}", location.location_id);
    (StatusCode::OK, Json(location)).into_response()
}
