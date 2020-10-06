use crate::database::{create_pool, DBPool};
use crate::domain;
use crate::error::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::convert::Infallible;
use warp::{http::StatusCode, Filter};

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn with_handler<T, I, O>(handler: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone
where
    T: Fn(I) -> O + Send + Clone,
    O: Send,
{
    warp::any().map(move || handler.clone())
}

fn to_response<T: Serialize>(res: Result<T, Error>) -> impl warp::Reply {
    match res {
        Ok(x) => {
            let json = warp::reply::json(&x);
            warp::reply::with_status(json, StatusCode::OK)
        }
        Err(e) => {
            let json = warp::reply::json(&ErrorResponse {
                message: e.to_string(),
            });
            warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
// pub fn json_api(
//     db: DBPool,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
// {
//     warp::path!("todos").and(warp::body::json())
//         // .and_then(domain::user::create)
// }
//
mod user {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct CreatePayload {}
    pub async fn create(payload: CreatePayload) -> Result<impl warp::Reply, warp::Rejection> {
        Ok(StatusCode::OK)
    }
}

pub async fn serve() -> Result<(), Error> {
    let db = create_pool()?;
    let routes = warp::path!("api")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(user::create);
    //     .or(warp::path!("health").map(|_| StatusCode::OK));
    // warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
    Ok(())
}
