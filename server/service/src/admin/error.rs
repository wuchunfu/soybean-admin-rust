use sea_orm::DbErr;

pub struct AppError {
    pub code: u16,
    pub message: String,
}

impl AppError {
    pub fn new(code: u16, message: String) -> Self {
        AppError { code, message }
    }
}

impl From<DbErr> for AppError {
    fn from(db_err: DbErr) -> Self {
        AppError::new(500, db_err.to_string())
    }
}
