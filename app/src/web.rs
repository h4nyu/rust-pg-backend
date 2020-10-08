use crate::database::{create_pool, DBPool, Transaction};
use crate::domain;
use crate::error::Error;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::fmt::Debug;

pub fn to_response<T>(input: Result<T, Error>) -> impl Responder
where
    T: Serialize + Debug,
{
    input
        .map(|x| HttpResponse::Ok().json(x))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub mod user {
    use super::*;

    #[post("/api/v1/user")]
    async fn create(
        pool: web::Data<DBPool>,
        payload: web::Json<domain::user::CreatePayload>,
    ) -> impl Responder {
        to_response(
            async {
                let mut conn = pool.get().await?;
                let res = domain::user::create(&conn, &payload).await;
                res
            }
            .await,
        )
    }

    #[put("/api/v1/user")]
    async fn update(
        pool: web::Data<DBPool>,
        payload: web::Json<domain::user::UpdatePayload>,
    ) -> impl Responder {
        to_response(
            async {
                let mut conn = pool.get().await?;
                domain::user::update(&conn, &payload).await
            }
            .await,
        )
    }

    #[delete("/api/v1/user")]
    async fn delete(
        pool: web::Data<DBPool>,
        payload: web::Json<domain::user::DeletePayload>,
    ) -> impl Responder {
        to_response(
            async {
                let mut conn = pool.get().await?;
                let resp = domain::user::delete(&conn, &payload).await;
                resp
            }
            .await,
        )
    }
}

pub async fn serve() -> Result<(), Error> {
    let db = create_pool()?;
    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .service(user::create)
            .service(user::update)
            .service(user::delete)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
