pub mod zettel_repository {
	use crate::persistence::models::ZettelDao;
	use sqlx::{Error, PgPool};

	pub async fn find_all(pool: &PgPool) -> Result<Vec<ZettelDao>, Error> {
		let zettels = sqlx::query_as::<_, ZettelDao>("SELECT id, content FROM zettels ORDER BY id")
			.fetch_all(pool)
			.await?;

		Ok(zettels)
	}
}
