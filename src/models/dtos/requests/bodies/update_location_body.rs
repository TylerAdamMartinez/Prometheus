use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateLocationBody {
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub street_number: Option<String>,
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub bounding_box: Option<String>, // WKT format
    pub location: Option<String>,     // WKT format
    pub time_zone: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub is_public: Option<bool>,
    pub notes: Option<String>,
}
