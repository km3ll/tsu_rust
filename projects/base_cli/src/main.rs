use clap::Parser;
use base_cli::core::processors::guessing_game;

#[derive(Debug, Parser)]
struct Args {
	cmd: String,
	args: Option<String>
}

fn main() {
	let args = Args::parse();
	dispatch(&args);
}

fn dispatch(args: &Args) {
	match args {
		Args { cmd, args: Some(arg) }
			if cmd.as_str() == "start" && arg.as_str() == "game" => guessing_game(),
		_ => println!("No match")
	}
}