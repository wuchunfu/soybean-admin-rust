use axum::{
    async_trait,
    extract::{Form, FromRequest, Json, Request},
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

use crate::web::res::Res;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid JSON data")]
    JsonError,

    #[error("Invalid form data")]
    FormError,

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Data is missing")]
    DataMissing,
}

#[derive(Debug, Clone)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate + Send + Sync,
    B: Send + Sync,
{
    type Rejection = ValidationError;

    async fn from_request(req: Request, state: &B) -> Result<Self, Self::Rejection> {
        let content_type = req.headers().get(CONTENT_TYPE).and_then(|value| value.to_str().ok());

        match content_type.as_deref() {
            Some(ct) if ct.contains(mime::APPLICATION_JSON.as_ref()) => {
                let result = Json::<T>::from_request(req, state).await;
                if let Ok(json) = result {
                    json.0.validate().map_err(ValidationError::from)?;
                    Ok(ValidatedForm(json.0))
                } else {
                    Err(ValidationError::JsonError)
                }
            }
            Some(ct) if ct.contains(mime::APPLICATION_WWW_FORM_URLENCODED.as_ref()) => {
                let result = Form::<T>::from_request(req, state).await;
                match result {
                    Ok(form) => {
                        form.0.validate().map_err(ValidationError::from)?;
                        Ok(ValidatedForm(form.0))
                    }
                    Err(_) => Err(ValidationError::FormError),
                }
            }
            _ => Err(ValidationError::DataMissing),
        }
    }
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        Res::<String>::new_error(StatusCode::BAD_REQUEST.as_u16(), format!("{}", self).as_ref())
            .into_response()
    }
}
