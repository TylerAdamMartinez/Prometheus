use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateLocationBody {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub street_number: Option<String>,
    pub street_name: String,
    pub city: String,
    pub state: Option<String>,
    pub country: String,
    pub postal_code: String,
    pub bounding_box: Option<String>,
    pub time_zone: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
}
