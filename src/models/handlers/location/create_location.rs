use axum::{Json, extract::State};
use http::StatusCode;
use sqlx::PgPool;
use std::str::FromStr;

use crate::{
    dtos::{requests::bodies::CreateLocationBody, responses::LocationDTO},
    enums::{Country, USAState},
    models::Location,
};

pub async fn create_location(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLocationBody>,
) -> Result<Json<LocationDTO>, StatusCode> {
    tracing::info!("Received request to create location: {:?}", payload);

    let state = match &payload.state {
        Some(state) => match USAState::from_str(state) {
            Ok(valid_state) => Some(valid_state),
            Err(_) => {
                tracing::error!("Invalid state provided: {}", state);
                return Err(StatusCode::BAD_REQUEST);
            }
        },
        None => None,
    };

    let country = match Country::from_str(&payload.country) {
        Ok(valid_country) => valid_country.alpha3_code(),
        Err(_) => {
            tracing::error!("Invalid country provided: {}", payload.country);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let record = sqlx::query_as!(
        Location,
        r#"
        INSERT INTO location (
            name, latitude, longitude, street_number, street_name, city, state, country, 
            postal_code, bounding_box, location, time_zone, created_at, 
            description, is_active, is_public, notes
        ) 
        VALUES (
            $1, $2, $3, $4, $5, $6, $7::text, $8::text, 
            $9, ST_GeomFromText($10, 4326), ST_SetSRID(ST_MakePoint($2, $3), 4326), $11, 
            now(), $12, true, false, $13
        ) 
        RETURNING 
            location_id, name, latitude, longitude, altitude, 
            street_number, street_name, city, state, country, postal_code, 
            ST_AsText(bounding_box) AS "bounding_box?", ST_AsText(location) AS location, 
            time_zone, created_at, updated_at, description, is_active, 
            deactivated_at, is_public, notes
        "#,
        payload.name,
        payload.latitude,
        payload.longitude,
        payload.street_number,
        payload.street_name,
        payload.city,
        state.map(|s| s.to_string()),
        country,
        payload.postal_code,
        payload.bounding_box,
        payload.time_zone,
        payload.description,
        payload.notes
    )
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to insert location into database: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(
        "Successfully inserted location with ID: {}",
        record.location_id
    );

    tracing::info!("Returning location data for ID: {}", record.location_id);
    Ok(Json(LocationDTO::from(&record)))
}
