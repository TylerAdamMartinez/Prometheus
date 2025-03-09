use serde::Deserialize;

#[derive(Deserialize)]
pub struct AllLocationQueries {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
}
