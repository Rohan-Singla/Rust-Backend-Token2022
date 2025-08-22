use axum::{Json, Router, routing::post};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::json;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use spl_token::instruction::{ initialize_mint, mint_to};
use std::str::FromStr;
use spl_token::instruction as token_instruction;
use crate::models::response::{ErrorResponse, SuccessResponse};
use crate::models::token::{
    AccountMetaResponse, CreateTokenRequest, InstructionResponse, MintRequest, TokenTransferRequest,
};

pub fn token_routes() -> Router {
    Router::new()
        .route("/token/create", post(handle_create_token))
        .route("/token/mint", post(handle_mint_token))
        .route("/send/token", post(handle_token_transfer))
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

    let instruction: Instruction =
        mint_to(&spl_token::ID, &mint, &dest, &auth, &[], payload.amount)
            .map_err(|_| error_response("Unable to create mint_to instruction"))?;

    Ok(Json(build_success_response(instruction)))
}

async fn handle_token_transfer(
    Json(payload): Json<TokenTransferRequest>,
) -> Json<serde_json::Value> {
    // Validate inputs
    let owner = match Pubkey::from_str(&payload.owner) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid owner pubkey"
            }));
        }
    };

    let destination = match Pubkey::from_str(&payload.destination) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid destination pubkey"
            }));
        }
    };

    let mint = match Pubkey::from_str(&payload.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid mint pubkey"
            }));
        }
    };

    if payload.amount == 0 {
        return Json(json!({
            "success": false,
            "error": "Amount must be greater than 0"
        }));
    }

    let owner_token_account =
        spl_associated_token_account::get_associated_token_address(&owner, &mint);
    let destination_token_account =
        spl_associated_token_account::get_associated_token_address(&destination, &mint);

    let ix = match token_instruction::transfer(
        &spl_token::id(),         
        &owner_token_account,    
        &destination_token_account,
        &owner,                   
        &[],                   
        payload.amount,            
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Failed to build transfer instruction"
            }));
        }
    };

    let accounts: Vec<AccountMetaResponse> = ix
        .accounts
        .iter()
        .map(|acc| AccountMetaResponse {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
        })
        .collect();

    let resp = InstructionResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data: bs58::encode(ix.data).into_string(),
    };

    Json(json!({
        "success": true,
        "data": resp
    }))
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
