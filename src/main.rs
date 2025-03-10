use crate::handlers::{
    location::{
        create_location, delete_location, get_all_locations, get_location, get_locations_nearby,
        update_location,
    },
    root,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

mod dtos;
mod enums;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect_lazy(&database_url)?;

    let app = Router::new()
        .route("/", get(root))
        .route("/locations", post(create_location).get(get_all_locations))
        .route(
            "/locations/:id",
            get(get_location)
                .put(update_location)
                .delete(delete_location),
        )
        .route("/locations/nearby", get(get_locations_nearby))
        .with_state(db_pool);
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(&address).await.unwrap();

    println!("🚀 Server running at http://{}", &address);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
