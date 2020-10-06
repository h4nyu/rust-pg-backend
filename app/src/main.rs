use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /{String} => 200 OK with body "Hello, {String}!"
    let hello = warp::path!(String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 80))
        .await;
}
