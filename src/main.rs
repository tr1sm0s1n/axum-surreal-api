mod config;
mod handlers;
mod models;
mod templates;

use axum::{
    routing::{get, patch, post},
    Router,
};
use handlers::{books, users, views};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        .route("/", get(views::home))
        .route("/register", get(views::register_user))
        .route("/register", post(users::register))
        .route("/login", get(views::login_user))
        .route("/login", post(users::login))
        .route("/add-book", post(books::add_book))
        .route("/add-review", patch(books::add_review))
        .nest_service("/public", ServeDir::new("public"))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(client)
}
