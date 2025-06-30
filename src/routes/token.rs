use axum::{routing::post, Json, Router};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;
use std::str::FromStr;

use crate::models::response::{ErrorResponse, SuccessResponse};
use crate::models::token::CreateTokenRequest;

pub fn token_routes() -> Router {
    Router::new().route("/token/create", post(handle_create_token))
}

async fn handle_create_token(
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<SuccessResponse<serde_json::Value>>, Json<ErrorResponse>> {
    let mint_auth = Pubkey::from_str(&payload.mint_authority)
        .map_err(|_| error_response("Invalid base58 mintAuthority"))?;

    let mint = Pubkey::from_str(&payload.mint)
        .map_err(|_| error_response("Invalid base58 mint address"))?;

    let instruction = initialize_mint(
        &spl_token::ID,
        &mint,
        &mint_auth,
        None,
        payload.decimals,
    )
    .map_err(|_| error_response("Unable to create initialize_mint instruction"))?;

    let account_meta = instruction
        .accounts
        .iter()
        .map(|acc| {
            serde_json::json!({
                "pubkey": acc.pubkey.to_string(),
                "is_signer": acc.is_signer,
                "is_writable": acc.is_writable
            })
        })
        .collect::<Vec<_>>();

    let encoded_instruction = STANDARD.encode(&instruction.data);

    let response_body = serde_json::json!({
        "program_id": instruction.program_id.to_string(),
        "accounts": account_meta,
        "instruction_data": encoded_instruction
    });

    Ok(Json(SuccessResponse {
        success: true,
        data: response_body,
    }))
}

fn error_response(msg: &str) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        success: false,
        error: msg.to_string(),
    })
}
