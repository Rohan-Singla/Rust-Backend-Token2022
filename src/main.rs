mod models;
mod routes;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber;
use crate::routes::{keypair::keypair_routes, token::token_routes};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let router = Router::new()
        .route("/", get(|| async { "🟢 API is up and running!" }))
        .nest("/keypair", keypair_routes())
        .merge(token_routes());

    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let socket_address = format!("0.0.0.0:{}", port);

    println!("🌐 Listening at http://{}", socket_address);

    let listener = TcpListener::bind(&socket_address).await.expect("Failed to bind port");
    axum::serve(listener, router).await.expect("Server crashed");
}
