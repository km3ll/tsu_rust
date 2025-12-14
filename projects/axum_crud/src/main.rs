use crate::db::init_db;
use crate::rest::router;
use tokio::net::TcpListener;

mod db;
mod rest;
mod view;

#[tokio::main]
async fn main() {
    // Load environment variables from .env if available
    dotenv::dotenv().ok();

    // Initialize the database and obtain a connection pool
    let connection_pool = init_db().await.unwrap();

    let listener = TcpListener::bind("localhost:3000")
        .await
        .expect("Failed to bind tcp listener");
    println!("Server running on http://localhost:3000");

    // Initialize the Axum routing service
    let app = router(connection_pool);

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start the server")

}
