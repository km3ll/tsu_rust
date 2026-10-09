use crate::http::dtos::FindAllZettelsResponse;
use crate::{
	http::dtos::Zettel, init::app::AppState, persistence::repositories::zettel_repository,
};
use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;

pub async fn find_all_zettels(
	State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<FindAllZettelsResponse>) {
	let zettels: Vec<Zettel> = zettel_repository::find_all(&state.pg_pool)
		.await
		.expect("Failed to find all Zettels")
		.iter()
		.map(|dao| Zettel::new(dao.id, dao.content.clone()))
		.collect();

	(StatusCode::OK, Json(FindAllZettelsResponse::new(zettels)))
}
