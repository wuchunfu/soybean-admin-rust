use server_shared::error::{ApiError, AppError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
    #[error("Username cannot be empty")]
    UsernameEmpty,
}

impl ApiError for UserError {
    fn code(&self) -> u16 {
        match self {
            UserError::UserNotFound => 1001,
            UserError::UsernameEmpty => 1002,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}
