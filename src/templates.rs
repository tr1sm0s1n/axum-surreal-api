use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct Index {
    pub title: &'static str,
    pub message: &'static str,
}

#[derive(Template)]
#[template(path = "register-user.html")]
pub struct RegisterUser {
    pub title: &'static str,
}

#[derive(Template)]
#[template(path = "login-user.html")]
pub struct LoginUser {
    pub title: &'static str,
}
