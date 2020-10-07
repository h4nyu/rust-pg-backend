use crate::database::{create_pool, DBConn, DBPool, Transaction};
use crate::domain;
use crate::domain::*;
use crate::error::Error;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::convert::Infallible;
use std::future::Future;

pub fn to_response<T>(input: Result<T, Error>) -> impl Responder
where
    T: Serialize,
{
    input
        .map(|x| HttpResponse::Ok().json(x))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/api/v1/user/all")]
async fn all(
    pool: web::Data<DBPool>,
    payload: web::Json<domain::user::CreatePayload>,
) -> impl Responder {
    to_response(async {
        let mut conn = pool.get().await?;
        let tx = conn.transaction().await?;
        domain::user::create(&tx, &payload).await
    }.await)
}

pub async fn serve() -> Result<(), Error> {
    let db = create_pool()?;
    HttpServer::new(move || App::new().data(db.clone()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}
