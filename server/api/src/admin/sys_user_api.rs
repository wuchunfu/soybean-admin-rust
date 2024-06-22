use std::sync::Arc;

use axum::Extension;
use server_core::web::res::Res;
use server_model::admin::entities::User;
use server_service::admin::{SysUserService, TUserService};

pub struct SysUserApi;

impl SysUserApi {
    pub async fn get_all_users(
        Extension(service): Extension<Arc<SysUserService>>,
    ) -> Res<Vec<User>> {
        let users = service.get_all_users().await;
        Res::new_data(users)
    }
}
