#[cfg(test)]
mod lib_tests {
	use pod_cli::cli::config::{CliConfig, ServerConfig};
	use pod_cli::cli::model::CliArgs;
	use pod_cli::dispatch;

	#[test]
	fn dispatch_runs_valid_command() {
		let config = CliConfig::new(ServerConfig::new(String::from("localhost"), 8080));
		let args = CliArgs::new(String::from("start-server"), None);
		dispatch(&config, &args)
	}

	#[test]
	#[should_panic]
	fn dispatch_panics_upon_invalid_command() {
		let config = CliConfig::new(ServerConfig::new(String::from("localhost"), 8080));
		let args = CliArgs::new(String::from("unknown"), None);
		dispatch(&config, &args)
	}
}
