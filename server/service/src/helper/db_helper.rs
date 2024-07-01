use std::sync::Arc;

use sea_orm::{ConnAcquireErr, DatabaseConnection, DbErr};
use server_core::web::error::AppError;
use server_global::global::GLOBAL_PRIMARY_DB;

pub async fn get_db_connection() -> Result<Arc<DatabaseConnection>, AppError> {
    let db = GLOBAL_PRIMARY_DB.read().await;
    db.as_ref()
        .cloned()
        .ok_or_else(|| AppError::from(DbErr::ConnectionAcquire(ConnAcquireErr::Timeout)))
}
