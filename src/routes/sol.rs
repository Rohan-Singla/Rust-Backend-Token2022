use std::str::FromStr;

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    system_instruction,
};
use bs58;

#[derive(Deserialize)]
struct SolTransferRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Serialize)]
struct InstructionResponse {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}

pub fn sol_routes() -> Router {
    Router::new().route("/send/sol", post(handle_sol_transfer))
}

async fn handle_sol_transfer(
    Json(payload): Json<SolTransferRequest>,
) -> Json<serde_json::Value> {
    if payload.lamports == 0 {
        return Json(json!({
            "success": false,
            "error": "Lamports must be greater than 0"
        }));
    }

    let from_key = match Pubkey::from_str(&payload.from) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid sender pubkey"
            }));
        }
    };

    let to_key = match Pubkey::from_str(&payload.to) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(json!({
                "success": false,
                "error": "Invalid recipient pubkey"
            }));
        }
    };

    if from_key == to_key {
        return Json(json!({
            "success": false,
            "error": "Sender and recipient cannot be the same"
        }));
    }

    let instruction: Instruction =
        system_instruction::transfer(&from_key, &to_key, payload.lamports);

    let instruction_data = bs58::encode(&instruction.data).into_string();

    let resp = InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts: instruction
            .accounts
            .iter()
            .map(|acc| acc.pubkey.to_string())
            .collect(),
        instruction_data,
    };

    Json(json!({
        "success": true,
        "data": resp
    }))
}
