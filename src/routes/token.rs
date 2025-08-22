use axum::{Json, Router, routing::post};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use solana_sdk::{pubkey::Pubkey, instruction::Instruction};
use spl_token::instruction::{initialize_mint, mint_to};
use std::str::FromStr;

use crate::models::response::{ErrorResponse, SuccessResponse};
use crate::models::token::{CreateTokenRequest, MintRequest};

pub fn token_routes() -> Router {
    Router::new()
        .route("/token/create", post(handle_create_token))
        .route("/token/mint", post(handle_mint_token))
}

async fn handle_create_token(
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<SuccessResponse<serde_json::Value>>, Json<ErrorResponse>> {
    let mint_auth = Pubkey::from_str(&payload.mint_authority)
        .map_err(|_| error_response("Invalid base58 mintAuthority"))?;

    let mint = Pubkey::from_str(&payload.mint)
        .map_err(|_| error_response("Invalid base58 mint address"))?;

    let instruction = initialize_mint(&spl_token::ID, &mint, &mint_auth, None, payload.decimals)
        .map_err(|_| error_response("Unable to create initialize_mint instruction"))?;

    Ok(Json(build_success_response(instruction)))
}

async fn handle_mint_token(
    Json(payload): Json<MintRequest>,
) -> Result<Json<SuccessResponse<serde_json::Value>>, Json<ErrorResponse>> {
    let mint = Pubkey::from_str(&payload.mint)
        .map_err(|_| error_response("Invalid base58 mint address"))?;

    let dest = Pubkey::from_str(&payload.destination)
        .map_err(|_| error_response("Invalid base58 destination address"))?;

    let auth = Pubkey::from_str(&payload.authority)
        .map_err(|_| error_response("Invalid base58 authority address"))?;

    let instruction: Instruction = mint_to(
        &spl_token::ID,
        &mint,
        &dest,
        &auth,
        &[],
        payload.amount,
    )
    .map_err(|_| error_response("Unable to create mint_to instruction"))?;

    Ok(Json(build_success_response(instruction)))
}

fn build_success_response(instruction: Instruction) -> SuccessResponse<serde_json::Value> {
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

    SuccessResponse {
        success: true,
        data: response_body,
    }
}

fn error_response(msg: &str) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        success: false,
        error: msg.to_string(),
    })
}
