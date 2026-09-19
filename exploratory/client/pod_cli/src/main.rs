use clap::Parser;
use pod_cli::cli::config::CliConfig;
use pod_cli::cli::model::CliArgs;
use pod_cli::dispatch;

fn main() {
	let config = CliConfig::from_file("config/config.yaml");
	let args = CliArgs::parse();
	dispatch(&config, &args)
}
