#[allow(unused)]
use base_app::config::AppConfig;

#[cfg(test)]
mod config_tests {
	use super::*;

	#[test]
	fn from_file_with_valid_file_returns_config() {
		let result = AppConfig::from_file("tests/resources/config.test.yaml");
		assert!(result.is_ok());
	}

	#[test]
	fn from_file_with_invalid_file_returns_error() {
		let result = AppConfig::from_file("tests/resources/dummy.yaml");
		assert!(result.is_err());
	}
}
