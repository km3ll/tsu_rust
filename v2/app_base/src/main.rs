mod tsu;

use tsu::config::AppConfig;

fn main() {
	load_config();
}

fn load_config() {
	let cfg = AppConfig::from_file().expect("");
	println!("Server will run on {}:{}", cfg.server.host, cfg.server.port)
}
