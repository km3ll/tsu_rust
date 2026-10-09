use sqlx::PgPool;

pub struct AppState {
	pub pg_pool: PgPool,
}

impl AppState {
	pub fn new(pg_pool: PgPool) -> Self {
		Self { pg_pool }
	}
}
