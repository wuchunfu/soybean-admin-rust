use std::sync::Arc;

use axum::Extension;
use server_core::web::{error::AppError, res::Res, validator::ValidatedForm};
use server_service::admin::{AuthOutput, LoginInput, SysAuthService, TAuthService};

pub struct SysAuthenticationApi;

impl SysAuthenticationApi {
    pub async fn login_handler(
        Extension(service): Extension<Arc<SysAuthService>>,
        ValidatedForm(input): ValidatedForm<LoginInput>,
    ) -> Result<Res<AuthOutput>, AppError> {
        service.pwd_login(input, "built-in").await.map(Res::new_data)
    }
}
