use crate::error::Error;
use serde::{Deserialize, Serialize};
pub struct User {}
use async_trait::async_trait;

#[derive(Serialize, Deserialize)]
pub struct CreatePayload {}

#[async_trait]
pub trait Fetch<K, V> {}

pub mod user {
    use super::*;
    pub async fn create(payload: CreatePayload) -> Result<(), Error> {
        Ok(())
    }

    pub async fn update(payload: CreatePayload) -> Result<(), Error> {
        Ok(())
    }
}
