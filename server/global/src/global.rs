use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};

use once_cell::sync::Lazy;

static GLOBAL_CONFIG: Lazy<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn init_config<T: 'static + Any + Send + Sync>(config: T) {
    let mut context = GLOBAL_CONFIG.write().unwrap();
    context.insert(TypeId::of::<T>(), Arc::new(config));
}

pub fn get_config<T: 'static + Any + Send + Sync>() -> Option<Arc<T>> {
    let context = GLOBAL_CONFIG.read().unwrap();
    context
        .get(&TypeId::of::<T>())
        .and_then(|config| config.clone().downcast::<T>().ok())
}
