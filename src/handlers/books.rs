use axum::{extract::State, http::StatusCode, Json};
use surrealdb::{engine::remote::ws::Client, opt::PatchOp, Surreal};

use crate::{
    handlers::internal_error,
    models::{Book, Review},
};

pub async fn add_book(
    State(db): State<Surreal<Client>>,
    Json(input): Json<Book>,
) -> Result<Json<Option<Book>>, (StatusCode, String)> {
    let book: Option<Book> = db
        .create(("book", input.title.clone()))
        .content(input)
        .await
        .map_err(internal_error)?;
    dbg!(book.clone());

    Ok(Json(book))
}

pub async fn add_review(
    State(db): State<Surreal<Client>>,
    Json(input): Json<Review>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let updated: Option<Book> = db
        .update(("book", input.title.clone()))
        .patch(PatchOp::add("reviews", &[input]))
        .await
        .map_err(internal_error)?;
    dbg!(updated.clone());

    Ok((StatusCode::OK, "success".to_string()))
}
