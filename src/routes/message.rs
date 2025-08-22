use axum::{debug_handler, response::IntoResponse, routing::post, Json, Router};
use bs58;
use solana_sdk::signature::{Keypair, Signature, Signer};

use crate::models::token::SignMessageRequest;


pub fn message_routes() -> Router {
    Router::new().route("/message/sign", post(handle_sign_message))
}
#[debug_handler]
async fn handle_sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> impl IntoResponse {
    let secret_bytes: Vec<u8> = match bs58::decode(&payload.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(serde_json::json!({
                "success": false,
                "error": "Invalid base58 secret key"
            }));
        }
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Json(serde_json::json!({
                "success": false,
                "error": "Invalid secret key bytes"
            }));
        }
    };

    // Sign the message
    let signature: Signature = keypair.sign_message(payload.message.as_bytes());

    Json(serde_json::json!({
        "success": true,
        "data": {
            "public_key": keypair.pubkey().to_string(),
            "message": payload.message,
            "signature": signature.to_string()
        }
    }))
}
