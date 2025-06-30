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
        // .merge(token_routes()); 

    println!("🚀 Server running at http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}