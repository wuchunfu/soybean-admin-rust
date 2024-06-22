use std::sync::Arc;

use axum::{Extension, Router};
use server_router::admin::{SysAuthenticationRouter, SysRoleRouter, SysUserRouter};
use server_service::admin::{SysRoleService, SysUserService};

pub async fn initialize_admin_router() -> Router {
    let app = Router::new();
    app.merge(SysAuthenticationRouter::init_authentication_router().await)
        .merge(SysUserRouter::init_user_router().await)
        .merge(SysRoleRouter::init_role_router().await)
        .layer(Extension(Arc::new(SysUserService)))
        .layer(Extension(Arc::new(SysRoleService)))
}
