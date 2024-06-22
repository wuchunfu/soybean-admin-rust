use axum::{
    async_trait,
    extract::{rejection::FormRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Form, Json,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

use crate::web::res::Res;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                Json(Res::<String>::new_error(StatusCode::BAD_REQUEST.as_u16(), message.as_str()))
            }
            ServerError::AxumFormRejection(_) => Json(Res::<String>::new_error(
                StatusCode::BAD_REQUEST.as_u16(),
                self.to_string().as_str(),
            )),
        }
        .into_response()
    }
}
