mod models;
mod routes;
mod state;

use axum::Router;
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
        .nest("/keypair", keypair_routes());
        // .merge(token_routes()); // Uncomment this once token routes are ready

    // Use Railway's PORT env or fallback to 3000 locally
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
