use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

static GLOBAL_CONFIG: Lazy<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn init_config<T: 'static + Any + Send + Sync>(config: T) {
    let mut context = GLOBAL_CONFIG.write().await;
    context.insert(TypeId::of::<T>(), Arc::new(config));
}

pub async fn get_config<T: 'static + Any + Send + Sync>() -> Option<Arc<T>> {
    let context = GLOBAL_CONFIG.read().await;
    context
        .get(&TypeId::of::<T>())
        .and_then(|config| config.clone().downcast::<T>().ok())
}

#[allow(dead_code)]
static GLOBAL_DB_POOL: Lazy<Arc<RwLock<HashMap<String, DatabaseConnection>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

// static GLOBAL_REDIS_POOL: Lazy<Arc<RwLock<HashMap<String, RedisClient>>>> =
// Lazy::new(|| {     Arc::new(RwLock::new(HashMap::new()))
// });
//
// static GLOBAL_MONGO_POOL: Lazy<Arc<RwLock<HashMap<String, MongoClient>>>> =
// Lazy::new(|| {     Arc::new(RwLock::new(HashMap::new()))
// });
