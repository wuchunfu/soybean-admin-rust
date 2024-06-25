use std::sync::Arc;

use axum::{body::Body, Extension, Router};
use server_middleware::{Request, RequestId, RequestIdLayer};
use server_router::admin::{SysAuthenticationRouter, SysUserRouter};
use server_service::admin::SysUserService;
use tower_http::trace::TraceLayer;
use tracing::info_span;

pub async fn initialize_admin_router() -> Router {
    let app = Router::new();
    app.merge(SysAuthenticationRouter::init_authentication_router().await)
        .merge(SysUserRouter::init_user_router().await)
        .layer(Extension(Arc::new(SysUserService)))
        .layer(TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
            let request_id = request
                .extensions()
                .get::<RequestId>()
                .map(ToString::to_string)
                .unwrap_or_else(|| "unknown".into());
            info_span!(
                "[soybean-admin-rust] >>>>>> request",
                id = %request_id,
                method = %request.method(),
                uri = %request.uri(),
            )
        }))
        .layer(RequestIdLayer)
}
