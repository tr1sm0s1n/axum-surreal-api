use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub async fn connect() -> surrealdb::Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.signin(Root {
        username: "admin",
        password: "pw123",
    })
    .await?;
    db.use_ns("test").use_db("test").await?;

    Ok(db)
}
