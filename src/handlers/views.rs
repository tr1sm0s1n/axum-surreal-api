use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::templates::{Index, LoginUser, RegisterUser};
use askama::Template;

pub async fn home() -> impl IntoResponse {
    let template = Index {
        title: "Axum-Surreal",
        message: "Hello, World!",
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn register_user() -> impl IntoResponse {
    let template = RegisterUser {
        title: "Register User",
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn login_user() -> impl IntoResponse {
    let template = LoginUser {
        title: "Login User",
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
