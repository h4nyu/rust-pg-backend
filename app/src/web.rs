use crate::database::{create_pool, DBPool};
use crate::domain;
use crate::domain::{Lock};
use crate::error::Error;
use actix_web::{delete, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

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
        lock: web::Data<Lock>,
        payload: web::Json<domain::user::CreatePayload>,
    ) -> impl Responder {
        to_response(
            async {
                let conn = pool.get().await?;
                domain::user::create(&conn, &lock, &payload).await
            }
            .await,
        )
    }

    #[put("/api/v1/user")]
    async fn update(
        pool: web::Data<DBPool>,
        lock: web::Data<Lock>,
        payload: web::Json<domain::user::UpdatePayload>,
    ) -> impl Responder {
        to_response(
            async {
                let conn = pool.get().await?;
                domain::user::update(&conn, &lock, &payload).await
            }
            .await,
        )
    }

    #[delete("/api/v1/user")]
    async fn delete(
        pool: web::Data<DBPool>,
        lock: web::Data<Lock>,
        payload: web::Json<domain::user::DeletePayload>,
    ) -> impl Responder {
        to_response(
            async {
                let conn = pool.get().await?;
                domain::user::delete(&conn, &lock, &payload).await
            }
            .await,
        )
    }
}

pub async fn serve() -> Result<(), Error> {
    let db = create_pool()?;
    let lock = Arc::new(Lock::default());
    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .data(lock.clone())
            .service(user::create)
            .service(user::update)
            .service(user::delete)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
