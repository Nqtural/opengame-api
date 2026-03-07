use axum::{Router, serve};
use tokio::net::TcpListener;
use opengame_api::routes;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/status", routes::status::routes());
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
