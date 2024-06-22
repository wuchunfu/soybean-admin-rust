use std::sync::Arc;

use axum::Extension;
use server_core::web::res::Res;
use server_model::admin::entities::Role;
use server_service::admin::{SysRoleService, TRoleService};

pub struct SysRoleApi;

impl SysRoleApi {
    pub async fn get_all_roles(
        Extension(service): Extension<Arc<SysRoleService>>,
    ) -> Res<Vec<Role>> {
        let roles = service.get_all_roles().await;
        Res::new_data(roles)
    }
}
