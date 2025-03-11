use axum::{
    Json,
    extract::{Query, State},
};
use http::StatusCode;
use sqlx::PgPool;

use crate::{
    dtos::{requests::queries::AllLocationQueries, responses::LocationDTO},
    models::Location,
};

pub async fn get_all_locations(
    State(pool): State<PgPool>,
    Query(query): Query<AllLocationQueries>,
) -> Result<Json<Vec<LocationDTO>>, StatusCode> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    tracing::info!(
        "Received request to fetch locations with filters: page={}, limit={}, state={:?}, country={:?}, city={:?}",
        page,
        limit,
        query.state,
        query.country,
        query.city
    );

    let records = sqlx::query!(
        r#"
        SELECT 
            location_id, name, latitude, longitude, altitude, 
            street_number, street_name, city, state, country, postal_code, 
            ST_AsText(bounding_box) AS bounding_box,  -- Convert geometry to WKT format
            ST_AsText(location) AS location,         -- Convert geometry to WKT format
            time_zone, created_at, updated_at, description, is_active, 
            deactivated_at, is_public, notes
        FROM location
        WHERE ($1::TEXT IS NULL OR state = $1::TEXT)
          AND ($2::TEXT IS NULL OR country = $2::TEXT)
          AND ($3::TEXT IS NULL OR city ILIKE '%' || $3 || '%')
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        "#,
        query.state,
        query.country,
        query.city,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to fetch locations from database: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(
        "Successfully fetched {} locations from database",
        records.len()
    );

    Ok(Json(
        records
            .into_iter()
            .map(|record| {
                LocationDTO::from(Location {
                    location_id: record.location_id,
                    name: record.name,
                    latitude: record.latitude,
                    longitude: record.longitude,
                    altitude: record.altitude,
                    street_number: record.street_number,
                    street_name: record.street_name,
                    city: record.city,
                    state: record
                        .state
                        .and_then(|s| Some(s.parse().unwrap_or(crate::enums::USAState::UNKNOWN))),
                    country: record
                        .country
                        .parse()
                        .unwrap_or(crate::enums::Country::Unknown),
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
