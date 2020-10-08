use crate::domain::*;
use crate::error::Error;
use async_trait::async_trait;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, tokio_postgres::NoTls, PgConnectionManager};
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::Config;
pub use tokio_postgres::{Row, Transaction};

pub fn create_pool() -> Result<DBPool, Error> {
    let config = Config::from_str("postgres://app:app@db")?;
    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(32)
        .max_idle(8)
        .get_timeout(Some(Duration::from_secs(15)))
        .build(manager))
}

pub type DBPool = Pool<PgConnectionManager<NoTls>>;
pub type DBConn = Connection<PgConnectionManager<NoTls>>;

fn to_user(row: Row) -> User {
    User {
        id: UserId(row.get("id")),
        name: UserName(row.get("name")),
        created_at: CreatedAt(row.get("created_at")),
    }
}

#[async_trait]
impl FetchUser<UserName> for DBConn {
    async fn fetch_user(&self, key: &UserName) -> Result<Option<User>, Error> {
        let res = self
            .query_opt("SELECT * FROM users where name = $1", &[&key.0])
            .await?
            .map(to_user);
        Ok(res)
    }
}

#[async_trait]
impl FetchUser<UserId> for DBConn {
    async fn fetch_user(&self, key: &UserId) -> Result<Option<User>, Error> {
        let res = self
            .query_opt("SELECT * FROM users where id = $1", &[&key.0])
            .await?
            .map(to_user);
        Ok(res)
    }
}

#[async_trait]
impl Upsert<User> for DBConn {
    async fn upsert(&self, row: &User) -> Result<(), Error> {
        let stmt = "INSERT INTO users (id, name, created_at) VALUES($1, $2, $3) ON CONFLICT (id) DO UPDATE SET id=$1, name=$2, created_at=$3";
        self.execute(stmt, &[&row.id.0, &row.name.0, &row.created_at.0]).await?;
        Ok(())
    }
}

#[async_trait]
impl Delete<UserId> for DBConn {
    async fn delete(&self, key: &UserId) -> Result<(), Error> {
        let stmt = "DELETE FROM users where id = $1";
        self.execute(stmt, &[&key.0]).await?;
        Ok(())
    }
}
