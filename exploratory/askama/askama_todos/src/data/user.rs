use crate::data::errors::DataError;
use sqlx::{Error, PgPool};

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<(), DataError> {
	let hashed_password: String = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
	let bytea_hash: &[u8] = hashed_password.as_bytes();

	// Runtime checking
	// sqlx::query(&query)
	//     .bind(email)
	//     .bind(bytea_hash)
	//     .execute(pool)
	//     .await?;

	sqlx::query!(
		"INSERT INTO users( email, password_hash ) VALUES($1, $2)",
		email,
		bytea_hash
	)
	.execute(pool)
	.await
	.map_err(|err| {
		match err {
			//Error::Configuration(_) => {}
			//Error::InvalidArgument(_) => {}
			Error::Database(e) => {
				if e.constraint() == Some("users_email_key") {
					DataError::FailedQuery("This email address is already used".to_string())
				} else {
					DataError::Internal(e.to_string())
				}
			}
			//Error::Io(_) => {}
			//Error::Tls(_) => {}
			//Error::Protocol(_) => {}
			//Error::RowNotFound => {}
			//Error::TypeNotFound { .. } => {}
			//Error::ColumnIndexOutOfBounds { .. } => {}
			//Error::ColumnNotFound(_) => {}
			//Error::ColumnDecode { .. } => {}
			//Error::Encode(_) => {}
			//Error::Decode(_) => {}
			//Error::AnyDriverError(_) => {}
			//Error::PoolTimedOut => {}
			//Error::PoolClosed => {}
			//Error::WorkerCrashed => {}
			//Error::Migrate(_) => {}
			//Error::InvalidSavePointStatement => {}
			//Error::BeginFailed => {}
			e => DataError::Query(e),
		}
	})?;

	Ok(())
}
