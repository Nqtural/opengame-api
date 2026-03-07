use axum::serve;
use tokio::net::TcpListener;
use opengame_api::app::app;

#[tokio::main]
async fn main() {
    let app = app();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
