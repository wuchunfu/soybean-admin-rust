pub use casbin_initialization::initialize_casbin;
pub use config_initialization::initialize_config;

pub mod casbin_initialization;
pub mod config_initialization;

#[cfg(test)]
mod tests {
    use std::{
        convert::Infallible,
        task::{Context, Poll},
    };

    use axum::{body::HttpBody, response::Response, routing::get, BoxError, Router};
    use axum_casbin::CasbinVals;
    use axum_test_helpers::TestClient;
    use bytes::Bytes;
    use casbin::{function_map::key_match2, CoreApi};
    use futures::future::BoxFuture;
    use http::{Request, StatusCode};
    use log::LevelFilter;
    use server_config::{get_config, DatabaseConfig};
    use simplelog::{Config as LogConfig, SimpleLogger};
    use tower::Service;

    use super::*;

    static INIT: std::sync::Once = std::sync::Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            SimpleLogger::init(LevelFilter::Info, LogConfig::default()).unwrap();
        });
    }

    #[tokio::test]
    async fn test_initialize_config() {
        init_logger();

        initialize_config("../resources/application.yaml").await;

        let db_config = get_config::<DatabaseConfig>().unwrap();
        assert_eq!(db_config.url, "postgres://user:password@localhost/db");
    }

    #[tokio::test]
    async fn test_initialize_casbin() {
        init_logger();

        let result = initialize_casbin(
            "../resources/rbac_model.conf",
            "postgresql://soybean:soybean@123.@localhost:35432/axum-admin",
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_initialize_casbin_with_axum() {
        init_logger();

        let casbin_middleware = initialize_casbin(
            "../resources/rbac_model.conf",
            "postgresql://soybean:soybean@123.@localhost:35432/axum-admin",
        )
        .await
        .unwrap();

        casbin_middleware
            .write()
            .await
            .get_role_manager()
            .write()
            .matching_fn(Some(key_match2), None);

        let app = Router::new()
            .route("/pen/1", get(handler))
            .route("/pen/2", get(handler))
            .route("/book/:id", get(handler))
            .layer(casbin_middleware)
            .layer(FakeAuthLayer);

        let client = TestClient::new(app);

        let resp_pen_1 = client.get("/pen/1").await;
        assert_eq!(resp_pen_1.status(), StatusCode::OK);

        let resp_book = client.get("/book/2").await;
        assert_eq!(resp_book.status(), StatusCode::OK);

        let resp_pen_2 = client.get("/pen/2").await;
        assert_eq!(resp_pen_2.status(), StatusCode::FORBIDDEN);
    }

    async fn handler() -> &'static str {
        "Hello, world!"
    }

    #[derive(Clone)]
    struct FakeAuthLayer;

    impl<S> tower::Layer<S> for FakeAuthLayer {
        type Service = FakeAuthMiddleware<S>;

        fn layer(&self, inner: S) -> Self::Service {
            FakeAuthMiddleware { inner }
        }
    }

    #[derive(Clone)]
    struct FakeAuthMiddleware<S> {
        inner: S,
    }

    impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for FakeAuthMiddleware<S>
    where
        S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
            + Clone
            + Send
            + 'static,
        S::Future: Send + 'static,
        ReqBody: Send + 'static,
        Infallible: From<<S as Service<Request<ReqBody>>>::Error>,
        ResBody: HttpBody<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError>,
    {
        type Error = S::Error;
        // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
        type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
        type Response = S::Response;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
            let not_ready_inner = self.inner.clone();
            let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

            Box::pin(async move {
                let vals = CasbinVals {
                    subject: String::from("alice"),
                    domain: None,
                };
                req.extensions_mut().insert(vals);
                inner.call(req).await
            })
        }
    }
}