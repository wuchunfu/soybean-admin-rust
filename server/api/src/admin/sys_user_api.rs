use std::sync::Arc;

use axum::Extension;
use server_core::web::{error::AppError, res::Res};
use server_service::admin::{sys_user, SysUserService, TUserService};

pub struct SysUserApi;

impl SysUserApi {
    pub async fn get_all_users(
        Extension(service): Extension<Arc<SysUserService>>,
    ) -> Result<Res<Vec<sys_user::Model>>, AppError> {
        service.find_all().await.map(Res::new_data)
    }
}
