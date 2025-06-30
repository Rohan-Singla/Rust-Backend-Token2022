use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}