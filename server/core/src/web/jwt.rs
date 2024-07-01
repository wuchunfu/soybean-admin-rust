use std::{error::Error, fmt};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, TokenData};
use server_config::JwtConfig;
use server_global::global;

use crate::web::auth::Claims;

// pub static KEYS: Lazy<Arc<Mutex<Keys>>> = Lazy::new(|| {
//     let config = global::get_config::<JwtConfig>()
//         .expect("[soybean-admin-rust] >>>>>> [server-core] Failed to load JWT
// config");     Arc::new(Mutex::new(Keys::new(config.jwt_secret.as_bytes())))
// });
//
// pub static VALIDATION: Lazy<Arc<Mutex<Validation>>> = Lazy::new(|| {
//     let config = global::get_config::<JwtConfig>()
//         .expect("[soybean-admin-rust] >>>>>> [server-core] Failed to load JWT
// config");     let mut validation = Validation::default();
//     validation.leeway = 60;
//     validation.set_issuer(&[config.issuer.clone()]);
//     Arc::new(Mutex::new(validation))
// });

#[derive(Debug)]
pub enum JwtError {
    KeysNotInitialized,
    ValidationNotInitialized,
    TokenCreationError(String),
    TokenValidationError(String),
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::KeysNotInitialized => write!(f, "Keys not initialized"),
            JwtError::ValidationNotInitialized => write!(f, "Validation not initialized"),
            JwtError::TokenCreationError(err) => write!(f, "Token creation error: {}", err),
            JwtError::TokenValidationError(err) => write!(f, "Token validation error: {}", err),
        }
    }
}

impl Error for JwtError {}

pub struct JwtUtils;

impl JwtUtils {
    pub async fn generate_token(claims: &Claims) -> Result<String, JwtError> {
        let keys_arc = global::KEYS.get().ok_or(JwtError::KeysNotInitialized)?;

        let keys = keys_arc.lock().await;

        let mut claims_clone = claims.clone();

        let now = Utc::now();
        let timestamp = now.timestamp() as usize;
        let jwt_config = global::get_config::<JwtConfig>().await.unwrap();
        claims_clone.set_exp((now + Duration::seconds(jwt_config.expire)).timestamp() as usize);
        claims_clone.set_iss(jwt_config.issuer.to_string());
        claims_clone.set_iat(timestamp);
        claims_clone.set_nbf(timestamp);
        claims_clone.set_jti("unique_token_id".to_string());

        encode(&Header::default(), &claims_clone, &keys.encoding)
            .map_err(|e| JwtError::TokenCreationError(e.to_string()))
    }

    pub async fn validate_token(
        token: &str,
        audience: &str,
    ) -> Result<TokenData<Claims>, JwtError> {
        let keys_arc = global::KEYS.get().ok_or(JwtError::KeysNotInitialized)?;

        let keys = keys_arc.lock().await;
        let validation_arc = global::VALIDATION.get().ok_or(JwtError::ValidationNotInitialized)?;
        let validation = validation_arc.lock().await;

        let mut validation_clone = validation.clone();
        validation_clone.set_audience(&[audience.to_string()]);
        decode::<Claims>(token, &keys.decoding, &validation_clone)
            .map_err(|e| JwtError::TokenValidationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use server_initialize::{initialize_config, initialize_keys_and_validation};
    use tokio::sync::Mutex;

    use super::*;

    fn create_claims(issuer: &str, audience: &str) -> Claims {
        let now = Utc::now();
        Claims::new(
            "user123".to_string(),
            audience.to_string(),
            "account".to_string(),
            vec!["admin".to_string()],
            "example_domain".to_string(),
            Option::from("example_org".to_string()),
        )
    }

    static INITIALIZED: Mutex<Option<Arc<()>>> = Mutex::const_new(None);

    async fn init() {
        let mut initialized = INITIALIZED.lock().await;
        if initialized.is_none() {
            initialize_config("../resources/application.yaml").await;
            initialize_keys_and_validation().await;
            *initialized = Some(Arc::new(()));
        }
    }

    #[tokio::test]
    async fn test_validate_token_success() {
        init().await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "audience");
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_audience() {
        init().await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "invalid_audience");
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_issuer() {
        init().await;

        let claims = create_claims("invalid_issuer", "audience");
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_expired() {
        init().await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "audience");
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let result = JwtUtils::validate_token(&token, "audience").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_invalid_signature() {
        init().await;

        let claims =
            create_claims("https://github.com/ByteByteBrew/soybean-admin-rust", "audience");
        let token = JwtUtils::generate_token(&claims).await.unwrap();

        let mut invalid_token = token.clone();
        let len = invalid_token.len();
        invalid_token.replace_range((len - 1)..len, "X");

        let result = JwtUtils::validate_token(&invalid_token, "audience").await;
        assert!(result.is_err());
    }
}
