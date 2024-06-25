pub use server_model::admin::{
    entities::{prelude::SysUser, sys_user},
    input::*,
};
pub use sys_user_service::{SysUserService, TUserService};
mod sys_user_service;
