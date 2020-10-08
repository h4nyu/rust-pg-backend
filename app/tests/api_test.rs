use actix_web::{App, test, http};
use app::domain::{UserName, Lock};
use app::domain;
use app::web;
use uuid::Uuid;
use futures::future::{try_join_all};
use app::database;
use std::sync::Arc;

#[tokio::test]
async fn test_lock() {
    let db = database::create_pool().unwrap();
    let conn = db.get().await.unwrap();
    let lock = Arc::new(Lock::default());
    let futs =
        (0..10).map(|_| async {
            let payload = domain::user::CreatePayload{name: UserName(Uuid::new_v4().to_string())};
            domain::user::create(&conn, &lock.clone(), &payload).await
        });
    try_join_all(futs).await.unwrap();
}
