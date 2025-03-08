use crate::enums::{Country, USAState};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub bounding_box: Option<((f64, f64), (f64, f64))>, // ((lat_min, lon_min), (lat_max, lon_max))
    pub time_zone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub is_active: bool,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub is_public: bool,
    pub notes: Option<String>,
}

impl Location {
    pub fn new(
        name: &str,
        latitude: f64,
        longitude: f64,
        street_number: Option<&str>,
        street_name: &str,
        city: &str,
        state: Option<USAState>,
        country: Country,
        postal_code: &str,
        time_zone: Option<&str>,
        bounding_box: Option<((f64, f64), (f64, f64))>,
        description: Option<&str>,
        notes: Option<&str>,
    ) -> Self {
        Self {
            location_id: Uuid::new_v4(),
            name: name.to_string(),
            latitude,
            longitude,
            altitude: None,
            street_number: street_number.map(|s| s.to_string()),
            street_name: street_name.to_string(),
            city: city.to_string(),
            state,
            country,
            postal_code: postal_code.to_string(),
            bounding_box,
            time_zone: time_zone.map(|s| s.to_string()),
            created_at: Utc::now(),
            updated_at: None,
            description: description.map(|s| s.to_string()),
            is_active: true,
            deactivated_at: None,
            is_public: false,
            notes: notes.map(|s| s.to_string()),
        }
    }
}
