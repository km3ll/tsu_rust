use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ZettelDao {
	pub id: i32,
	pub content: String,
}
