use std::sync::Arc;

use axum::Extension;
use server_core::web::res::Res;
use server_service::admin::{sys_user, SysUserService, TUserService};

pub struct SysUserApi;

impl SysUserApi {
    pub async fn get_all_users(
        Extension(service): Extension<Arc<SysUserService>>,
    ) -> Res<Vec<sys_user::Model>> {
        match service.find_all().await {
            Ok(users) => Res::<Vec<sys_user::Model>>::new_data(users),
            Err(err) => Res::new_error(err.code, err.message.as_str()),
        }
    }
}
