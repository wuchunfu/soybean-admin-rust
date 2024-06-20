use std::error::Error;

use axum_casbin::CasbinAxumLayer;
use casbin::DefaultModel;
use sea_orm::Database;
use sea_orm_adapter::SeaOrmAdapter;

pub async fn initialize_casbin(
    model_path: &str,
    db_url: &str,
) -> Result<CasbinAxumLayer, Box<dyn Error>> {
    let model = DefaultModel::from_file(model_path).await?;
    let db = Database::connect(db_url).await?;
    let adapter = SeaOrmAdapter::new(db).await?;

    let casbin_middleware = CasbinAxumLayer::new(model, adapter).await?;
    Ok(casbin_middleware)
}
