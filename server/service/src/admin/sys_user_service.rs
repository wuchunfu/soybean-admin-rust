use async_trait::async_trait;
use server_model::admin::entities::User;

#[async_trait]
pub trait TUserService {
    async fn get_all_users(&self) -> Vec<User>;
}

#[derive(Clone)]
pub struct SysUserService;

#[async_trait]
impl TUserService for SysUserService {
    async fn get_all_users(&self) -> Vec<User> {
        vec![
            User::new(1, "Alice".to_string()),
            User::new(2, "Bob".to_string()),
        ]
    }
}
