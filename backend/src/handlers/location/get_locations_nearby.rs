use axum::{
    Json,
    extract::{Query, State},
};
use http::StatusCode;
use sqlx::PgPool;

use crate::{
    dtos::{requests::queries::LocationNearbyQueries, responses::LocationDTO},
    models::Location,
};

pub async fn get_locations_nearby(
    State(pool): State<PgPool>,
    Query(query): Query<LocationNearbyQueries>,
) -> Result<Json<Vec<LocationDTO>>, StatusCode> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;
    let latitude = query.latitude.unwrap_or(0.0);
    let longitude = query.longitude.unwrap_or(0.0);
    let radius = query.radius.unwrap_or(10000.0); // Default: 10km

    tracing::info!(
        "Received request for nearby locations: lat={}, lon={}, radius={}m, page={}, limit={}",
        latitude,
        longitude,
        radius,
        page,
        limit
    );

    let records = sqlx::query!(
        r#"
        SELECT 
            location_id, name, latitude, longitude, altitude, 
            street_number, street_name, city, state, country, postal_code, 
            ST_AsText(bounding_box) AS bounding_box,  -- Convert geometry to WKT format
            ST_AsText(location) AS location,         -- Convert geometry to WKT format
            time_zone, created_at, updated_at, description, is_active, 
            deactivated_at, is_public, notes,
            ST_Distance(location, ST_GeomFromText($1, 4326)) AS distance  -- Calculate distance
        FROM location
        WHERE ST_DWithin(location, ST_GeomFromText($1, 4326), $2)
          AND deactivated_at IS NULL  --  Exclude deactivated locations
        ORDER BY distance ASC  -- Order by closest locations first
        LIMIT $3 OFFSET $4
        "#,
        format!("POINT({} {})", longitude, latitude),
        radius,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to fetch nearby locations: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!("Successfully fetched {} nearby locations", records.len());

    Ok(Json(
        records
            .into_iter()
            .map(|record| {
                LocationDTO::from(&Location {
                    location_id: record.location_id,
                    name: record.name,
                    latitude: record.latitude,
                    longitude: record.longitude,
                    altitude: record.altitude,
                    street_number: record.street_number,
                    street_name: record.street_name,
                    city: record.city,
                    state: record.state,
                    country: record.country,
                    postal_code: record.postal_code,
                    bounding_box: record.bounding_box,
                    location: record.location,
                    time_zone: record.time_zone,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                    description: record.description,
                    is_active: record.is_active,
                    deactivated_at: record.deactivated_at,
                    is_public: record.is_public,
                    notes: record.notes,
                })
            })
            .collect(),
    ))
}
