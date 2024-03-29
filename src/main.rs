pub mod handler;
pub mod router;
pub mod storage;
pub mod utils;
use router::router_init;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = router_init();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
