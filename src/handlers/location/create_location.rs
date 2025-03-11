use axum::{Json, extract::State};
use http::StatusCode;
use sqlx::PgPool;

use crate::{
    dtos::{requests::bodies::CreateLocationBody, responses::LocationDTO},
    models::Location,
};

pub async fn create_location(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLocationBody>,
) -> Result<Json<LocationDTO>, StatusCode> {
    tracing::info!("Received request to create location: {:?}", payload);

    let new_record = sqlx::query!(
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
    .map_err(|err| {
        tracing::error!("Failed to insert location into database: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(
        "Successfully inserted location with ID: {}",
        new_record.location_id
    );

    let location = Location {
        location_id: new_record.location_id,
        name: new_record.name,
        latitude: new_record.latitude,
        longitude: new_record.longitude,
        altitude: new_record.altitude,
        street_number: new_record.street_number,
        street_name: new_record.street_name,
        city: new_record.city,
        state: new_record
            .state
            .and_then(|s| Some(s.parse().unwrap_or(crate::enums::USAState::UNKNOWN))),
        country: new_record
            .country
            .parse()
            .unwrap_or(crate::enums::Country::Unknown),
        postal_code: new_record.postal_code,
        bounding_box: new_record.bounding_box,
        location: new_record.location,
        time_zone: new_record.time_zone,
        created_at: new_record.created_at,
        updated_at: new_record.updated_at,
        description: new_record.description,
        is_active: new_record.is_active,
        deactivated_at: new_record.deactivated_at,
        is_public: new_record.is_public,
        notes: new_record.notes,
    };

    tracing::info!("Returning location data for ID: {}", location.location_id);

    Ok(Json(location.into()))
}
