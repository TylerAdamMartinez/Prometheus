use crate::enums::{Country, USAState};
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Location {
    pub location_id: Uuid,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub street_number: Option<String>,
    pub street_name: String,
    pub city: String,
    pub state: Option<USAState>,
    pub country: Country,
    pub postal_code: String,
    // See https://github.com/opengeospatial/wkt for more details on KWTs
    pub bounding_box: Option<String>, // Store as WKT (Well-Known Text)
    pub location: Option<String>,     // Store as WKT (Well-Known Text)
    pub time_zone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub is_public: Option<bool>,
    pub notes: Option<String>,
}
