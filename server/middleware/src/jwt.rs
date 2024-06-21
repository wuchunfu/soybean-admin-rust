use axum::{
    body::Body, extract::Request, http::StatusCode, middleware::Next, response::IntoResponse,
};
use axum_casbin::CasbinVals;
use headers::{authorization::Bearer, Authorization, HeaderMapExt};
use server_core::web::{auth::User, jwt::JwtUtils, res::Res};

pub async fn jwt_auth_middleware(
    mut req: Request<Body>,
    next: Next,
    audience: &str,
) -> impl IntoResponse {
    let token = match req.headers().typed_get::<Authorization<Bearer>>() {
        Some(auth) => auth.token().to_string(),
        None => {
            return Res::<String>::new_error(
                StatusCode::UNAUTHORIZED.as_u16(),
                "No token provided or invalid token type",
            )
            .into_response();
        }
    };

    match JwtUtils::validate_token(&token, audience).await {
        Ok(data) => {
            let claims = data.claims;
            let user = User::from(claims);
            let vals = CasbinVals {
                subject: user.account().to_string(),
                domain: Option::from(user.organization().to_string()),
            };
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(vals);
            next.run(req).await.into_response()
        }
        Err(_) => Res::<String>::new_error(StatusCode::UNAUTHORIZED.as_u16(), "Invalid token")
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        response::IntoResponse,
        routing::get,
        Router,
    };
    use axum_casbin::{
        casbin::{DefaultModel, FileAdapter},
        CasbinAxumLayer,
    };
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use server_constant::definition::Audience;
    use server_core::web::{auth::Claims, res::Res};
    use server_initialize::initialize_config;
    use tower::{ServiceBuilder, ServiceExt};

    use crate::jwt::{jwt_auth_middleware, User};

    async fn user_info_handler(user: User) -> impl IntoResponse {
        Res::new_data(user).into_response()
    }

    #[tokio::test]
    async fn test_user_info_endpoint() {
        let m = DefaultModel::from_file("../../axum-casbin/examples/rbac_with_domains_model.conf")
            .await
            .unwrap();
        let a = FileAdapter::new("../../axum-casbin/examples/rbac_with_domains_policy.csv");

        let casbin_middleware = CasbinAxumLayer::new(m, a).await.unwrap();

        initialize_config("../resources/application.yaml").await;

        let app = Router::new()
            .route("/pen/1", get(user_info_handler))
            .layer(casbin_middleware)
            .layer(axum::middleware::from_fn(move |req, next| {
                jwt_auth_middleware(req, next, Audience::ManagementPlatform.as_str())
            }));

        let service = ServiceBuilder::new().service(app);

        let token = generate_jwt();

        let request = Request::builder()
            .uri("/pen/1")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = service.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(response.into_body(), 1000).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        println!("body_str is {}", body_str);
    }

    fn generate_jwt() -> String {
        let now = Utc::now();
        let claims = Claims::new(
            "user123".to_string(),
            (now + Duration::seconds(3600)).timestamp() as usize,
            "https://github.com/ByteByteBrew/soybean-admin-rust".to_string(),
            Audience::ManagementPlatform.as_str().to_string(),
            now.timestamp() as usize,
            now.timestamp() as usize,
            "unique_token_id".to_string(),
            "alice".to_string(),
            "example_role".to_string(),
            "domain1".to_string(),
        );

        let encoding_key = EncodingKey::from_secret("soybean-admin-rust".as_ref());
        encode(&Header::default(), &claims, &encoding_key).unwrap()
    }
}
