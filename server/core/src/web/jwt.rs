use std::sync::Arc;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use server_config::JwtConfig;
use server_global::global;
use tokio::sync::Mutex;

use crate::web::auth::Claims;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: Lazy<Arc<Mutex<Keys>>> = Lazy::new(|| {
    let config = global::get_config::<JwtConfig>()
        .expect("[soybean-admin-rust] >>>>>> [server-core] Failed to load JWT config");
    Arc::new(Mutex::new(Keys::new(config.jwt_secret.as_bytes())))
});

pub static VALIDATION: Lazy<Arc<Mutex<Validation>>> = Lazy::new(|| {
    let config = global::get_config::<JwtConfig>()
        .expect("[soybean-admin-rust] >>>>>> [server-core] Failed to load JWT config");
    let mut validation = Validation::default();
    validation.leeway = 60;
    validation.set_issuer(&[config.issuer.clone()]);
    Arc::new(Mutex::new(validation))
});

pub struct JwtUtils;

impl JwtUtils {
    pub async fn generate_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        let keys = KEYS.lock().await;
        encode(&Header::default(), claims, &keys.encoding)
    }

    pub async fn validate_token(
        token: &str,
        audience: &str,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let keys = KEYS.lock().await;
        let val = VALIDATION.lock().await;

        let mut validation_clone = val.clone();
        validation_clone.set_audience(&[audience.to_string()]);
        decode::<Claims>(token, &keys.decoding, &validation_clone)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use server_initialize::initialize_config;

    use super::*;

    fn create_claims(issuer: &str, audience: &str, exp_offset: i64) -> Claims {
        let now = Utc::now();
        Claims::new(
            "user123".to_string(),
            (now + Duration::seconds(exp_offset)).timestamp() as usize,
            issuer.to_string(),
            audience.to_string(),
            now.timestamp() as usize,
            now.timestamp() as usize,
            "unique_token_id".to_string(),
            "account".to_string(),
            "admin".to_string(),
            "example_domain".to_string(),
        )
    }

    #[tokio::test]
    async fn test_validate_token_success() {
        initialize_config("../resources/application.yaml").await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "audience", 3600);
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_audience() {
        initialize_config("../resources/application.yaml").await;

        let claims = create_claims(
            "https://github.com/ByteByteBrew/soybean-admin-rust",
            "invalid_audience",
            3600,
        );
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_issuer() {
        initialize_config("../resources/application.yaml").await;

        let claims = create_claims("invalid_issuer", "audience", 3600);
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_expired() {
        initialize_config("../resources/application.yaml").await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "audience", -3600);
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_signature() {
        initialize_config("../resources/application.yaml").await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "audience", 3600);
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let mut invalid_token = token.clone();
        let len = invalid_token.len();
        invalid_token.replace_range((len - 1)..len, "X");

        let result = JwtUtils::validate_token(&invalid_token, "audience").await;
        assert!(result.is_err());
    }
}
