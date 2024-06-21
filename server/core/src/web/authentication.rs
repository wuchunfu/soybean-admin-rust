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
    role: String,
    domain: String,
    org: String,
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
        role: String,
        domain: String,
        org: String,
    ) -> Self {
        Self {
            sub,
            exp,
            iss,
            aud,
            iat,
            nbf,
            jti,
            role,
            domain,
            org,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    user_id: String,
    account_name: String,
    role: String,
    organization: String,
}

impl From<Claims> for User {
    fn from(claims: Claims) -> Self {
        User {
            user_id: claims.sub,
            account_name: claims.aud,
            role: claims.role,
            organization: claims.org,
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
