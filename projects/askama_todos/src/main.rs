use askama_todos::routes::router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	let listener = TcpListener::bind("localhost:8000").await.unwrap();
	let app = router();
	axum::serve(listener, app).await.unwrap();
}
