use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct AuthOutput {
    pub access_token: String,
}
