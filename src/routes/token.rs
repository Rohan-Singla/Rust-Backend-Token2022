// use axum::{Json, Router, routing::post};
// use base64::{engine::general_purpose, Engine};
// use solana_sdk::{
//     instruction::{Instruction},
//     pubkey::Pubkey,
//     signature::{Keypair, Signature, Signer},
//     system_instruction,
// };
// use std::str::FromStr;

// use crate::models::{
//     response::{SuccessResponse, ErrorResponse},
//     token::{CreateTokenRequest, MintTokenRequest, SignMessageRequest, VerifyMessageRequest, SendSolRequest},
// };

// pub fn token_routes() -> Router {
//     Router::new()
//         .route("/token/create", post(create_token))
//         .route("/token/mint", post(mint_token))
//         .route("/message/sign", post(sign_message))
//         .route("/message/verify", post(verify_message))
//         .route("/send/sol", post(send_sol))
// }

// // === Endpoint 2 ===
// async fn create_token(Json(input): Json<CreateTokenRequest>) -> Result<Json<SuccessResponse<_>>, Json<ErrorResponse>> {
//     let mint = Pubkey::from_str(&input.mint).map_err(|e| err("Invalid mint pubkey"))?;
//     let mint_authority = Pubkey::from_str(&input.mint_authority).map_err(|e| err("Invalid mint authority"))?;

//     let ix = spl_token::instruction::initialize_mint(
//         &spl_token::ID,
//         &mint,
//         &mint_authority,
//         None,
//         input.decimals,
//     ).map_err(|e| err("Failed to create instruction"))?;

//     Ok(Json(SuccessResponse {
//         success: true,
//         data: serialize_instruction(ix),
//     }))
// }

// // === Endpoint 3 ===
// async fn mint_token(Json(input): Json<MintTokenRequest>) -> Result<Json<SuccessResponse<_>>, Json<ErrorResponse>> {
//     let mint = Pubkey::from_str(&input.mint).map_err(|_| err("Invalid mint"))?;
//     let destination = Pubkey::from_str(&input.destination).map_err(|_| err("Invalid destination"))?;
//     let authority = Pubkey::from_str(&input.authority).map_err(|_| err("Invalid authority"))?;

//     let ix = spl_token::instruction::mint_to(
//         &spl_token::ID,
//         &mint,
//         &destination,
//         &authority,
//         &[],
//         input.amount,
//     ).map_err(|_| err("Instruction creation failed"))?;

//     Ok(Json(SuccessResponse {
//         success: true,
//         data: serialize_instruction(ix),
//     }))
// }

// // === Endpoint 4 ===
// async fn sign_message(Json(input): Json<SignMessageRequest>) -> Result<Json<SuccessResponse<_>>, Json<ErrorResponse>> {
//     let secret_bytes = bs58::decode(&input.secret).into_vec().map_err(|_| err("Invalid secret key"))?;
//     let keypair = Keypair::from_bytes(&secret_bytes).map_err(|_| err("Invalid keypair"))?;
//     let message_bytes = input.message.as_bytes();
//     let signature = keypair.sign_message(message_bytes);

//     Ok(Json(SuccessResponse {
//         success: true,
//         data: serde_json::json!({
//             "signature": general_purpose::STANDARD.encode(signature.as_ref()),
//             "public_key": keypair.pubkey().to_string(),
//             "message": input.message
//         }),
//     }))
// }

// // === Endpoint 5 ===
// async fn verify_message(Json(input): Json<VerifyMessageRequest>) -> Result<Json<SuccessResponse<_>>, Json<ErrorResponse>> {
//     let pubkey = Pubkey::from_str(&input.pubkey).map_err(|_| err("Invalid pubkey"))?;
//     let signature_bytes = general_purpose::STANDARD.decode(&input.signature).map_err(|_| err("Invalid base64 sig"))?;
//     let signature = Signature::try_from(signature_bytes.as_slice()).map_err(|_| err("Invalid signature"))?;

//     let is_valid = signature.verify(pubkey.as_ref(), input.message.as_bytes()).is_ok();

//     Ok(Json(SuccessResponse {
//         success: true,
//         data: serde_json::json!({
//             "valid": is_valid,
//             "message": input.message,
//             "pubkey": input.pubkey
//         }),
//     }))
// }

// // === Endpoint 6 ===
// async fn send_sol(Json(input): Json<SendSolRequest>) -> Result<Json<SuccessResponse<_>>, Json<ErrorResponse>> {
//     let from = Pubkey::from_str(&input.from).map_err(|_| err("Invalid from pubkey"))?;
//     let to = Pubkey::from_str(&input.to).map_err(|_| err("Invalid to pubkey"))?;

//     let ix = system_instruction::transfer(&from, &to, input.lamports);

//     Ok(Json(SuccessResponse {
//         success: true,
//         data: serde_json::json!({
//             "program_id": ix.program_id.to_string(),
//             "accounts": ix.accounts.iter().map(|a| a.pubkey.to_string()).collect::<Vec<_>>(),
//             "instruction_data": general_purpose::STANDARD.encode(ix.data)
//         }),
//     }))
// }

// // === Utility ===
// fn err(msg: &str) -> Json<ErrorResponse> {
//     Json(ErrorResponse {
//         success: false,
//         error: msg.to_string(),
//     })
// }

// fn serialize_instruction(ix: Instruction) -> serde_json::Value {
//     serde_json::json!({
//         "program_id": ix.program_id.to_string(),
//         "accounts": ix.accounts.iter().map(|acc| {
//             serde_json::json!({
//                 "pubkey": acc.pubkey.to_string(),
//                 "is_signer": acc.is_signer,
//                 "is_writable": acc.is_writable
//             })
//         }).collect::<Vec<_>>(),
//         "instruction_data": general_purpose::STANDARD.encode(ix.data)
//     })
// }
