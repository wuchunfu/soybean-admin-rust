use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "example_validator=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    // build our application with a route
    let app = server_initialize::initialize_admin_router().await;

    // run it
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
