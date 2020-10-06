use app::error::Error;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres::Config, tokio_postgres::NoTls, PgConnectionManager};
use std::convert::Infallible;
use std::str::FromStr;
use std::time::Duration;
use warp::{http::StatusCode, Filter};
use serde::Serialize;

type DBPool = Pool<PgConnectionManager<NoTls>>;

pub fn create_pool() -> Result<DBPool, Error> {
    let config = Config::from_str("postgres://postgres@127.0.0.1:7878/postgres")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(32)
        .max_idle(8)
        .get_timeout(Some(Duration::from_secs(15)))
        .build(manager))
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health").map(|| StatusCode::OK);
    let routes = health_route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
