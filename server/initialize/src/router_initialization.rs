use std::sync::Arc;

use axum::{Extension, Router};
use server_router::admin::{SysAuthenticationRouter, SysUserRouter};
use server_service::admin::SysUserService;

pub async fn initialize_admin_router() -> Router {
    let app = Router::new();
    app.merge(SysAuthenticationRouter::init_authentication_router().await)
        .merge(SysUserRouter::init_user_router().await)
        .layer(Extension(Arc::new(SysUserService)))
}
