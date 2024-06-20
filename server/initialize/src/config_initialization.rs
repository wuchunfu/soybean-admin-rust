use log::{error, info};

pub async fn initialize_config(file_path: &str) {
    if let Err(e) = server_config::init_from_file(file_path).await {
        error!(
            "[soybean-admin-rust] >>>>>> [server-initialize] Failed to initialize config: {:?}",
            e
        );
    } else {
        info!(
            "[soybean-admin-rust] >>>>>> [server-initialize] Configuration initialized successfully"
        );
    }
}
