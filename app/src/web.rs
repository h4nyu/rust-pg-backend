use crate::database::{create_pool, DBPool};
use crate::domain;
use crate::error::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::convert::Infallible;
use std::future::Future;
use warp::{http::StatusCode, Filter};

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
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

#[derive(Serialize, Deserialize)]
pub struct AAA {
    id: i32,
}
// async fn handler() -> Result<AAA, Error> {
//     Ok(AAA{id:1})
// }

fn json_api<Fut, I, O>(
    handler: fn(I) -> Fut,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    Fut: Future<Output = Result<O, Error>> + Send,
    I: DeserializeOwned + Send,
    O: Serialize,
{
    warp::any()
        .and(warp::post())
        .and(warp::body::json())
        .map_async(handler)
        .map(to_response)
}

pub async fn serve() -> Result<(), Error> {
    let db = create_pool()?;
    let routes = warp::path!("api")
        .and(json_api(domain::user::create))
        .or(warp::path!("api/update").and(json_api(domain::user::update)));

    // warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
    Ok(())
}
