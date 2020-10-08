use actix_web::{http, test, App};
use app::database;
use app::domain;
use app::domain::{Lock, UserName};
use app::web;
use futures::future::try_join_all;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_lock() {
    let db = database::create_pool().unwrap();
    let conn = db.get().await.unwrap();
    let lock = Arc::new(Lock::default());
    let futs = (0..10).map(|_| async {
        let payload = domain::user::CreatePayload {
            name: UserName(Uuid::new_v4().to_string()),
        };
        domain::user::create(&conn, &lock.clone(), &payload).await
    });
    try_join_all(futs).await.unwrap();
}
