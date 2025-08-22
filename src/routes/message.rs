use axum::{Json, Router, debug_handler, response::IntoResponse, routing::post};
use bs58;
use serde_json::json;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
};

use crate::models::token::{SignMessageRequest, VerifyMessageRequest, VerifyMessageResponse};

pub fn message_routes() -> Router {
    Router::new()
        .route("/message/sign", post(handle_sign_message))
        .route("/message/verify", post(handle_verify_message))
}

#[debug_handler]
async fn handle_sign_message(Json(payload): Json<SignMessageRequest>) -> impl IntoResponse {
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

pub async fn handle_verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> Json<serde_json::Value> {
    let pubkey_bytes = match bs58::decode(payload.pubkey.clone()).into_vec() {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid public key encoding"
            }));
        }
    };

    let pubkey = match Pubkey::try_from(pubkey_bytes.as_slice()) {
        Ok(p) => p,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid pubkey bytes"
            }));
        }
    };

    let sig_bytes = match bs58::decode(payload.signature.clone()).into_vec() {
        Ok(s) => s,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid signature encoding"
            }));
        }
    };

    let signature = match Signature::try_from(sig_bytes.as_slice()) {
        Ok(s) => s,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid signature bytes"
            }));
        }
    };

    let is_valid = signature.verify(pubkey.as_ref(), payload.message.as_bytes());

    Json(json!({
        "success": true,
        "data": VerifyMessageResponse {
            valid: is_valid,
            message: payload.message,
            pubkey: payload.pubkey,
        }
    }))
}
