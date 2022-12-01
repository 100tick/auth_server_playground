use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub picture: String,
    pub locale: String,
}
