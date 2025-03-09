use axum::{extract::{Query, State}, Json};
use http::StatusCode;
use sqlx::PgPool;

use crate::{dtos::requests::queries::AllLocationQueries, models::Location};

pub async fn get_all_locations(
    State(pool): State<PgPool>,
    Query(query): Query<AllLocationQueries>,
) -> Result<Json<Vec<Location>>, StatusCode> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let locations = sqlx::query_as!(
        Location,
        "SELECT * FROM locations WHERE ($1 IS NULL OR state = $1) AND ($2 IS NULL OR country = $2) AND ($3 IS NULL OR city ILIKE '%' || $3 || '%') ORDER BY created_at DESC LIMIT $4 OFFSET $5",
        query.state,
        query.country,
        query.city,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(locations))
}
