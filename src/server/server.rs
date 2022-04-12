use axum::{routing::get, Router};
use std::net::SocketAddr;

// Cloud Runにデプロイするために特定のPORTをリッスンしたサーバーを建てる必要がある
pub async fn launch() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
    .route("/", get(root));

    // TODO: Remove hard code for PORT
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}