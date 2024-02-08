use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub title: &'static str,
    pub message: &'static str
}
