use crate::database::{create_pool, DBConn, DBPool, Transaction};
use crate::domain;
use crate::domain::*;
use crate::error::Error;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::convert::Infallible;
use std::future::Future;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

pub async fn serve() -> Result<(), Error> {
    let db = create_pool()?;
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}
