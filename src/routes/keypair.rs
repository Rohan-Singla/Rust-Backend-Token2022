use axum::{Json, Router, routing::post};
use base58::ToBase58;
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};

use crate::models::response::{ErrorResponse, SuccessResponse};

#[derive(Serialize)]
struct KeypairResponse {
    pubkey: String,
    secret: String,
}

pub fn keypair_routes() -> Router {
    Router::new().route("/", post(generate_keypair))
}

async fn generate_keypair() -> Result<Json<SuccessResponse<KeypairResponse>>, Json<ErrorResponse>> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = keypair.to_bytes().to_base58();

    Ok(Json(SuccessResponse {
        success: true,
        data: KeypairResponse { pubkey, secret },
    }))
}
