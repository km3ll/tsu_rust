use axum::{Router, response::Html, routing::get};

const INDEX_PAGE: &str = include_str!("index.html");

pub fn view_service() -> Router {
	Router::new().route("/", get(index_page))
}

async fn index_page() -> Html<&'static str> {
	Html(INDEX_PAGE)
}
