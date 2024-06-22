use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginInput {
    #[validate(length(min = 5, message = "Account cannot be empty"))]
    pub account: String,
    #[validate(length(min = 6, message = "Password cannot be empty"))]
    pub password: String,
}
