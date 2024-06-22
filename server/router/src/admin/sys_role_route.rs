use axum::{routing::get, Router};
use server_api::admin::SysRoleApi;

pub struct SysRoleRouter;

impl SysRoleRouter {
    pub async fn init_role_router() -> Router {
        let router = Router::new().route("/roles", get(SysRoleApi::get_all_roles));
        Router::new().nest("/role", router)
    }
}
