use askama_todos::{init, routes};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    init::variables();
    init::logging();
    let config = init::config();

    info!("Server is starting...");
    let listener = TcpListener::bind(config.get_addr())
        .await
        .expect("Failed to bind the listener");

    let app = routes::router();
    info!("Listening at {}", config.get_addr());

    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
