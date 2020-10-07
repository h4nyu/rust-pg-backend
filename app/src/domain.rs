use crate::database::DBConn;
use crate::error::Error;
use serde::{Deserialize, Serialize};
pub struct User {}
use async_trait::async_trait;
use std::future::Future;

#[async_trait]
pub trait FetchUser<K> {
    async fn fetch_user(&self, key: K) -> Result<Option<User>, Error>;
}

#[async_trait]
pub trait Upsert<V> {
    async fn fetch_user(&mut self, row: K) -> Result<Option<User>, Error>;
}

#[async_trait]
pub trait Commit {
    async fn commit(&mut self) -> Result<(), Error>;
}



pub mod user {
    #[derive(Serialize, Deserialize)]
    pub struct User {
        id: String,
        name: String,
    }


    #[derive(Serialize, Deserialize)]
    pub struct CreatePayload {
    }
    use super::*;
    pub async fn create<T>(deps: &T, payload: &CreatePayload) -> Result<(), Error>
    where
        T: Commit + FetchUser<()>,
    {
        Ok(())
    }

    pub async fn update(payload: CreatePayload) -> Result<(), Error> {
        Ok(())
    }
}
