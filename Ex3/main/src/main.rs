use tokio::net::TcpListener;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn health() -> String{
    "hello".to_string()
}

