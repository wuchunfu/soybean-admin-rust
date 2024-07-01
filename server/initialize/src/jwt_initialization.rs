use std::sync::Arc;

use server_config::JwtConfig;
use server_global::{global, Validation};
use tokio::sync::{mpsc, Mutex};

pub async fn initialize_keys_and_validation() {
    let jwt_config = match global::get_config::<JwtConfig>().await {
        Some(cfg) => cfg,
        None => {
            eprintln!("[soybean-admin-rust] >>>>>> [server-core] Failed to load JWT config");
            return;
        }
    };

    let keys = global::Keys::new(jwt_config.jwt_secret.as_bytes());
    global::KEYS.set(Arc::new(Mutex::new(keys))).unwrap_or_else(|_| {
        eprintln!("[soybean-admin-rust] >>>>>> [server-core] Failed to set KEYS")
    });

    let mut validation = Validation::default();
    validation.leeway = 60;
    validation.set_issuer(&[&jwt_config.issuer]);
    global::VALIDATION.set(Arc::new(Mutex::new(validation))).unwrap_or_else(|_| {
        eprintln!("[soybean-admin-rust] >>>>>> [server-core] Failed to set VALIDATION")
    });

    initialize_global_event_channel().await;
    if let Some(rx) = global::get_event_receiver().await {
        server_service::admin::start_event_listener(rx).await;
    }
}

pub async fn initialize_global_event_channel() {
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    *global::EVENT_SENDER.lock().await = Some(tx);
    *global::EVENT_RECEIVER.lock().await = Some(rx);
}
