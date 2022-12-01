use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
    pub token_type: String,
}
