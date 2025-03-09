use serde::Deserialize;

#[derive(Deserialize)]
pub struct LocationNearbyQueries {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius: Option<f64>,
}
