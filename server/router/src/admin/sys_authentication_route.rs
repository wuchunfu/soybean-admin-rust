use axum::{routing::post, Router};
use server_api::admin::SysAuthenticationApi;

pub struct SysAuthenticationRouter;

impl SysAuthenticationRouter {
    pub async fn init_authentication_router() -> Router {
        let router = Router::new().route("/login", post(SysAuthenticationApi::login_handler));
        Router::new().nest("/auth", router)
    }
}
