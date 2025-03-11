use serde::Serialize;

use crate::models::Location;

#[derive(Serialize, Debug, Clone)]
pub struct LocationDTO {
    pub location_id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub address: String,
    pub bounding_box: Option<String>,
    pub location: Option<String>,
    pub time_zone: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
}

impl From<Location> for LocationDTO {
    fn from(loc: Location) -> Self {
        LocationDTO {
            location_id: loc.location_id.to_string(),
            name: loc.name,
            latitude: loc.latitude,
            longitude: loc.longitude,
            altitude: loc.altitude,
            address: format!(
                "{} {}, {} {} {} {}",
                loc.street_number.unwrap_or_default(),
                loc.street_name,
                loc.city,
                loc.state.map_or_else(|| "".to_string(), |s| s.to_string()),
                loc.country.to_string(),
                loc.postal_code
            ),
            bounding_box: loc.bounding_box,
            location: loc.location,
            time_zone: loc.time_zone,
            created_at: loc.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: loc.updated_at.map(|dt| dt.to_rfc3339()),
            description: loc.description,
            notes: loc.notes,
        }
    }
}
