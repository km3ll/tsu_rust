use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
	pub cmd: String,
	pub args: Option<String>,
}

impl CliArgs {
	pub fn new(cmd: String, args: Option<String>) -> Self {
		CliArgs { cmd, args }
	}
}
