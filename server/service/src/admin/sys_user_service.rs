use async_trait::async_trait;
use sea_orm::EntityTrait;
use server_core::web::error::AppError;
use server_model::admin::entities::{prelude::SysUser, sys_user};

use crate::helper::db_helper;

#[async_trait]
pub trait TUserService {
    async fn find_all(&self) -> Result<Vec<sys_user::Model>, AppError>;
}

#[derive(Clone)]
pub struct SysUserService;

#[async_trait]
impl TUserService for SysUserService {
    async fn find_all(&self) -> Result<Vec<sys_user::Model>, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find().all(db.as_ref()).await.map_err(AppError::from)
    }
}
