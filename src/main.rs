mod models;
mod routes;
mod state;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use routes::keypair::keypair_routes;
// use routes::token::token_routes;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "✅ Server is live!" })) // 👈 add this
        .nest("/keypair", keypair_routes());
        // .merge(token_routes());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
