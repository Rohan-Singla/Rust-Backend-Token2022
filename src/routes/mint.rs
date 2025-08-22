use axum::{routing::post, Json, Router};
use solana_sdk::{pubkey::Pubkey, instruction::Instruction};
use spl_token::instruction as token_instruction;

use crate::models::{MintRequest, ApiResponse, InstructionData, AccountMetaData};

pub fn mint_routes() -> Router {
    Router::new()
        .route("/token/mint", post(mint_tokens))
}

/// Handler for POST /token/mint
async fn mint_tokens(Json(req): Json<MintRequest>) -> Json<ApiResponse<InstructionData>> {
    match build_mint_to(req) {
        Ok(data) => Json(ApiResponse { success: true, data: Some(data), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e) }),
    }
}

/// Build SPL Token MintTo instruction
fn build_mint_to(req: MintRequest) -> Result<InstructionData, String> {
    // Parse pubkeys
    let mint_pubkey = req.mint.parse::<Pubkey>()
        .map_err(|_| "Invalid mint pubkey".to_string())?;
    let dest_pubkey = req.destination.parse::<Pubkey>()
        .map_err(|_| "Invalid destination pubkey".to_string())?;
    let auth_pubkey = req.authority.parse::<Pubkey>()
        .map_err(|_| "Invalid authority pubkey".to_string())?;

    // Build instruction
    let ix: Instruction = token_instruction::mint_to(
        &spl_token::id(),
        &mint_pubkey,
        &dest_pubkey,
        &auth_pubkey,
        &[],
        req.amount,
    ).map_err(|e| format!("Failed to build instruction: {:?}", e))?;

    // Convert accounts to serializable struct
    let accounts: Vec<AccountMetaData> = ix.accounts
        .iter()
        .map(|a| AccountMetaData {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();

    let instruction_data = base64::encode(ix.data);

    Ok(InstructionData {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    })
}
