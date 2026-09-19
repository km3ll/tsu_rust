pub mod cli;
pub mod core;
pub mod utils;

use crate::cli::config::CliConfig;
use crate::cli::model::CliArgs;
use crate::core::command::start_server;

pub fn dispatch(cli_config: &CliConfig, cli_args: &CliArgs) {
	let command = cli_args.cmd.as_str();
	match command {
		"start-server" => start_server(&cli_config.server, &cli_args.args),
		cmd => panic!("Unrecognized command: {cmd}"),
	}
}
