use crate::error::Error;
use async_trait::async_trait;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct Lock {
    user: Mutex<()>,
}
impl Default for Lock {
    fn default() -> Self {
        Self {
            user: Mutex::new(()),
        }
    }
}

#[async_trait]
pub trait FetchUser<K> {
    async fn fetch_user(&self, key: &K) -> Result<Option<User>, Error>;
}

#[async_trait]
pub trait Insert<V> {
    async fn insert(&self, row: &V) -> Result<(), Error>;
}

#[async_trait]
pub trait Update<V> {
    async fn update(&self, row: &V) -> Result<(), Error>;
}

#[async_trait]
pub trait Delete<K> {
    async fn delete(&self, key: &K) -> Result<(), Error>;
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct UserId(pub String);
impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserName(pub String);

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatedAt(pub NaiveDateTime);
impl Default for CreatedAt {
    fn default() -> Self {
        Self(Utc::now().naive_utc())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub created_at: CreatedAt,
}

pub mod user {
    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct CreatePayload {
        pub name: UserName,
    }
    pub async fn create<T>(store: &T, lock: &Lock, payload: &CreatePayload) -> Result<UserId, Error>
    where
        T: FetchUser<UserName> + Insert<User>,
    {
        let id = Uuid::new_v4().to_string();
        let _l = lock.user.lock().await;
        if store.fetch_user(&payload.name).await?.is_some() {
            println!("unlock{:?}", id);
            Err(Error::UserAlreadyExists)?;
        }
        let new_user = User {
            id: Default::default(),
            name: payload.name.clone(),
            created_at: Default::default(),
        };
        store.insert(&new_user).await?;
        Ok(new_user.id)
    }

    #[derive(Serialize, Deserialize)]
    pub struct UpdatePayload {
        pub user_id: UserId,
        pub name: UserName,
    }
    pub async fn update<T>(store: &T, lock: &Lock, payload: &UpdatePayload) -> Result<UserId, Error>
    where
        T: FetchUser<UserId> + FetchUser<UserName> + Update<User>,
    {
        let _l = lock.user.lock().await;
        let mut user = store
            .fetch_user(&payload.user_id)
            .await?
            .ok_or(Error::UserNotFound)?;
        match store.fetch_user(&payload.name).await? {
            Some(u) => {
                if u.id == user.id {
                    Err(Error::UserAlreadyExists)?;
                }
            }
            None => {}
        }
        user.name = payload.name.clone();
        store.update(&user).await?;
        Ok(user.id)
    }

    #[derive(Serialize, Deserialize)]
    pub struct DeletePayload {
        pub user_id: UserId,
    }
    pub async fn delete<T>(store: &T, lock: &Lock, payload: &DeletePayload) -> Result<(), Error>
    where
        T: FetchUser<UserId> + Delete<UserId>,
    {
        let _l = lock.user.lock().await;
        store
            .fetch_user(&payload.user_id)
            .await?
            .ok_or(Error::UserNotFound)?;
        store.delete(&payload.user_id).await?;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    struct MemStore {
        pub uses: Vec<User>,
    }

    #[async_trait]
    impl FetchUser<UserId> for MemStore {
        async fn fetch_user(&self, key: &UserId) -> Result<Option<User>, Error> {
            let res = self.uses.iter().find(|x| &x.id == key).map(|x| x.clone());
            Ok(res)
        }
    }
}
