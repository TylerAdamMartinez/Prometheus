use crate::handlers::{root, location::{create_location, get_location, get_all_locations, get_locations_nearby}};
use tokio::net::TcpListener;
use axum::{routing::{get, post}, Router};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use sqlx::PgPool;

mod enums;
mod models;
mod handlers;
mod dtos;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/locations", post(create_location))
        .route("/locations/:id", get(get_location))
        .route("/locations", get(get_all_locations))
        .route("/locations/nearby", get(get_locations_nearby))
        // .route("/locations/:id", put(update_location))
        // .route("/locations/:id", delete(delete_location))
        .with_state(db_pool);
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(&address).await.unwrap();

    println!("🚀 Server running at http://{}", &address);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
