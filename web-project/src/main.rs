use axum::{response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;

async fn get_index() -> impl IntoResponse {
    "Hello world"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_index));

    let addr = SocketAddr::new([0, 0, 0, 0].into(), 3000);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

