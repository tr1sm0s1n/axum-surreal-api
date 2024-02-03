use axum::{extract::State, http::StatusCode, Json};
// use axum_macros::debug_handler;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::User,
    types::LoginRequest,
};

// #[debug_handler]
pub async fn register(
    State(db): State<Surreal<Client>>,
    Json(input): Json<User>,
) -> Result<Json<Option<User>>, (StatusCode, String)> {
    let user: Option<User> = db
        .create(("user", input.email.clone()))
        .content(input)
        .await
        .map_err(internal_error)?;
    dbg!(user.clone());

    Ok(Json(user))
}

pub async fn login(
    State(db): State<Surreal<Client>>,
    Json(input): Json<LoginRequest>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let user: Option<User> = db
        .select(("user", input.email))
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

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
