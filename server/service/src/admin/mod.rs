pub use server_model::admin::{
    entities::{prelude::SysUser, sys_user},
    input::*,
};
pub use sys_user_service::{SysUserService, TUserService};
pub mod error;
pub mod errors;
mod sys_user_service;

pub use errors::*;
