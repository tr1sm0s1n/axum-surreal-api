use axum::{extract::State, http::StatusCode, Json};
// use axum_macros::debug_handler;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{handlers::internal_error, models::User};

// #[debug_handler]
pub async fn register(
    State(db): State<Surreal<Client>>,
    Json(input): Json<User>,
) -> Result<Json<Option<User>>, (StatusCode, String)> {
    let user: Option<User> = db
        .create(("user", input.name.clone()))
        .content(input)
        .await
        .map_err(internal_error)?;
    dbg!(user.clone());

    Ok(Json(user))
}

pub async fn login(
    State(db): State<Surreal<Client>>,
    Json(input): Json<User>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let user: Option<User> = db
        .select(("user", input.name))
        .await
        .map_err(internal_error)?;
    dbg!(user.clone());

    match user {
        Some(u) => {
            if u.password != input.password {
                Ok((StatusCode::UNAUTHORIZED, "invalid credentials".to_string()))
            } else {
                Ok((StatusCode::OK, "success".to_string()))
            }
        }
        None => Ok((StatusCode::NOT_FOUND, "invalid user".to_string())),
    }
}
