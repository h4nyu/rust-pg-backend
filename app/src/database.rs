use crate::domain::*;
use crate::error::Error;
use async_trait::async_trait;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, tokio_postgres::NoTls, PgConnectionManager};
use std::future::Future;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::Config;
pub use tokio_postgres::Transaction;

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
pub type DBConn = Connection<PgConnectionManager<NoTls>>;

#[async_trait]
impl<'a> FetchUser<()> for Transaction<'a> {
    async fn fetch_user(&mut self, _: ()) -> Result<Option<User>, Error> {
        Ok(None)
    }
}

#[async_trait]
impl<'a> Commit for Transaction<'a> {
    async fn commit(&mut self) -> Result<(), Error> {
        self.commit().await?;
        Ok(())
    }
}
