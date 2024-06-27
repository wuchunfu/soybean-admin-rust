pub enum UserError {
    UserNotFound,
    UsernameEmpty,
}

impl UserError {
    pub fn code(&self) -> u16 {
        match self {
            UserError::UserNotFound => 1001,
            UserError::UsernameEmpty => 1002,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            UserError::UserNotFound => "User not found.",
            UserError::UsernameEmpty => "Username cannot be empty.",
        }
    }
}
