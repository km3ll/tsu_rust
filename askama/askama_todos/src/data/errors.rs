use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
	#[error("Failed database query: {0}")]
	Query(#[from] sqlx::Error),

	#[error("Failed to hash: {0}")]
	Bcrypt(#[from] bcrypt::BcryptError),

	#[error("Failed to query: {0}")]
	FailedQuery(String),

	#[error("Internal error: {0}")]
	Internal(String),
}
