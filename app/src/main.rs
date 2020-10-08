use app::web::serve;

#[actix_web::main]
async fn main() {
    serve().await.unwrap();
}
