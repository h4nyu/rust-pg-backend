use crate::error::Error;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, tokio_postgres::NoTls, PgConnectionManager};
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::Config;

pub fn create_pool() -> Result<DBPool, Error> {
    let config = Config::from_str("postgres://app@db")?;
    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(32)
        .max_idle(8)
        .get_timeout(Some(Duration::from_secs(15)))
        .build(manager))
}

pub type DBPool = Pool<PgConnectionManager<NoTls>>;
