mod tsu;

use tsu::config::AppConfig;
use tsu::resource::File;
use tsu::serde::*;

fn main() {
	load_config();
	load_file("resources/one-piece.txt");
	use_serdes();
}

fn load_config() {
	println!("----");
	let cfg = AppConfig::from_file().expect("");
	println!("Server will run on {}:{}", cfg.server.host, cfg.server.port)
}

fn load_file(file: &str) {
	println!("----");
	println!("Lines:");
	let file = File::from_file(file);
	for line in file.lines {
		println!("- {}", line)
	}
}

fn use_serdes() {
	println!("----");
	serdes_json();
	println!("----");
	serdes_xml();
}
