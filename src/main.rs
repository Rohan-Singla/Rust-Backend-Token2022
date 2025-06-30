mod models;
mod routes;

mod state;

use axum::Router;
use routes::keypair::keypair_routes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/keypair", keypair_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
