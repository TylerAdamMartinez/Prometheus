use serde::Deserialize;

#[derive(Deserialize)]
pub struct LocationNearbyQueries {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius: Option<f64>,
}
