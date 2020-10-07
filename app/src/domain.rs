use crate::database::DBConn;
use crate::error::Error;
use serde::{Deserialize, Serialize};
pub struct User {}
use async_trait::async_trait;
use std::future::Future;

#[async_trait]
pub trait FetchUser<K> {
    async fn fetch_user(&mut self, key: K) -> Result<Option<User>, Error>;
}

#[async_trait]
pub trait Commit {
    async fn commit(&mut self) -> Result<(), Error>;
}

// pub trait Store {
//     fn commit(&mut self) -> Result<(), Error>;
//     fn fetch_user<T>(&self, key:&T) -> Result<Option<User>, Error>;
//     fn save_user<T>(&mut self, row:&User) -> Result<(), Error>;
// }

pub mod user {
    #[derive(Serialize, Deserialize)]
    pub struct CreatePayload {}
    use super::*;
    pub async fn create<T>(deps: T, payload: CreatePayload) -> Result<(), Error>
    where
        T: Commit + FetchUser<()>,
    {
        Ok(())
    }

    pub async fn update(payload: CreatePayload) -> Result<(), Error> {
        Ok(())
    }
}
