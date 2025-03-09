use axum::{extract::{Query, State}, Json};
use http::StatusCode;
use sqlx::PgPool;

use crate::{dtos::requests::queries::LocationNearbyQueries, models::Location};

pub async fn get_locations_nearby(
    State(pool): State<PgPool>,
    Query(params): Query<LocationNearbyQueries>,
) -> Result<Json<Vec<Location>>, StatusCode> {
    let latitude = params.latitude.unwrap_or(0.0);
    let longitude = params.longitude.unwrap_or(0.0);
    let radius = params.radius.unwrap_or(10000.0); // Default: 10km

    let locations = sqlx::query_as!(
        Location,
        "SELECT * FROM locations 
         WHERE ST_DWithin(location, ST_GeomFromText($1, 4326), $2)",
        format!("POINT({} {})", longitude, latitude),
        radius
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(locations))
}
