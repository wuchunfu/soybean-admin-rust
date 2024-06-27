use async_trait::async_trait;
use sea_orm::EntityTrait;
use server_global::global::GLOBAL_PRIMARY_DB;
use server_model::admin::entities::{prelude::SysUser, sys_user};

use crate::admin::error::AppError;

#[async_trait]
pub trait TUserService {
    async fn find_all(&self) -> Result<Vec<sys_user::Model>, AppError>;
}

#[derive(Clone)]
pub struct SysUserService;

#[async_trait]
impl TUserService for SysUserService {
    async fn find_all(&self) -> Result<Vec<sys_user::Model>, AppError> {
        let db = GLOBAL_PRIMARY_DB.read().await;
        match *db {
            Some(ref db_ref) => SysUser::find().all(db_ref.as_ref()).await.map_err(AppError::from),
            None => Err(AppError::new(500, "Unable to acquire database connection".to_string())),
        }
    }
}
