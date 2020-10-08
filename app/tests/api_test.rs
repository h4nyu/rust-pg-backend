use actix_web::{App, test};
use app::domain;
use app::domain::{UserName};
use app::web;
use app::database;

#[tokio::test]
async fn test_user_creat() {
    let db = database::create_pool().unwrap();
    let mut app = test::init_service(App::new().data(db).service(web::user::create)).await;
    let req = test::TestRequest::post().uri("/api/v1/user").set_json(&domain::user::CreatePayload{name: UserName("test".into())}).to_request();
    let resp = test::call_service(&mut app, req).await;
}
