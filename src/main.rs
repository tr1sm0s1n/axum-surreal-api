mod config;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use handlers::users;

#[tokio::main]
async fn main() {
    // connecting to surrealdb
    let client = config::connect().await.unwrap();

    // logging middleware
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_surreal_app=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(client)).await.unwrap();
}

fn app(client: Surreal<Client>) -> Router {
    Router::new()
        .route("/", get(home))
        .route("/register", post(users::register))
        .route("/login", post(users::login))
        .layer(TraceLayer::new_for_http())
        .with_state(client)
}

async fn home() -> &'static str {
    "Hello, World!"
}
