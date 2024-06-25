use async_trait::async_trait;
use sea_orm::{DbErr, EntityTrait};
use server_global::global::GLOBAL_PRIMARY_DB;
use server_model::admin::entities::{prelude::SysUser, sys_user};

#[async_trait]
pub trait TUserService {
    async fn find_all(&self) -> Result<Vec<sys_user::Model>, DbErr>;
}

#[derive(Clone)]
pub struct SysUserService;

#[async_trait]
impl TUserService for SysUserService {
    async fn find_all(&self) -> Result<Vec<sys_user::Model>, DbErr> {
        let db = GLOBAL_PRIMARY_DB.read().await.clone().unwrap();
        SysUser::find().all(db.as_ref()).await
    }
}
