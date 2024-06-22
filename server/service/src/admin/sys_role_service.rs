use async_trait::async_trait;
use server_model::admin::entities::Role;

#[async_trait]
pub trait TRoleService {
    async fn get_all_roles(&self) -> Vec<Role>;
}

#[derive(Clone)]
pub struct SysRoleService;

#[async_trait]
impl TRoleService for SysRoleService {
    async fn get_all_roles(&self) -> Vec<Role> {
        vec![
            Role::new(1, "Admin".to_string()),
            Role::new(2, "Test".to_string()),
        ]
    }
}
