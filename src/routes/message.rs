use axum::{routing::post, Json, Router};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use solana_sdk::bs58;
use solana_sdk::signature::Keypair;
use crate::models::response::{ErrorResponse, SuccessResponse};
use crate::models::token::SignMessageRequest;
use solana_sdk::signer::Signer;

pub fn message_routes() -> Router {
    Router::new().route("/message/sign", post(handle_sign))
}

async fn handle_sign(
    Json(body): Json<SignMessageRequest>,
) -> Result<Json<SuccessResponse<serde_json::Value>>, Json<ErrorResponse>> {
    if body.message.is_empty() || body.secret.is_empty() {
        return Err(make_error("Missing required fields"));
    }

    let secret_bytes = bs58::decode(&body.secret)
        .into_vec()
        .map_err(|_| make_error("Secret key not valid base58"))?;

    let keypair = Keypair::from_bytes(&secret_bytes)
        .map_err(|_| make_error("Failed to parse keypair from secret"))?;

    let sig = keypair.sign_message(body.message.as_bytes());

    let data = serde_json::json!({
        "signature": STANDARD.encode(sig.as_ref()),
        "public_key": keypair.pubkey().to_string(),
        "message": body.message,
    });

    Ok(Json(SuccessResponse { success: true, data }))
}

fn make_error(msg: &str) -> Json<ErrorResponse> {
    Json(ErrorResponse { success: false, error: msg.to_owned() })
}
