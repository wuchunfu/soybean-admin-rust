use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::web::res::Res;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    sub: String,
    exp: usize,
    iss: String,
    aud: String,
    iat: usize,
    nbf: usize,
    jti: String,
    account: String,
    role: String,
    domain: String,
}

impl Claims {
    pub fn new(
        sub: String,
        exp: usize,
        iss: String,
        aud: String,
        iat: usize,
        nbf: usize,
        jti: String,
        account: String,
        role: String,
        domain: String,
    ) -> Self {
        Self {
            sub,
            exp,
            iss,
            aud,
            iat,
            nbf,
            jti,
            account,
            role,
            domain,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    user_id: String,
    account: String,
    role: String,
    organization: String,
}

impl User {
    pub fn account(&self) -> &str {
        &self.account
    }

    pub fn organization(&self) -> &str {
        &self.organization
    }
}

impl From<Claims> for User {
    fn from(claims: Claims) -> Self {
        User {
            user_id: claims.sub,
            account: claims.account,
            role: claims.role,
            organization: claims.domain,
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = Res<String>;

    async fn from_request(req: Request, _state: &B) -> Result<Self, Self::Rejection> {
        req.extensions()
            .get::<User>()
            .cloned()
            .ok_or_else(|| Res::new_error(StatusCode::UNAUTHORIZED.as_u16(), "Unauthorized"))
    }
}
