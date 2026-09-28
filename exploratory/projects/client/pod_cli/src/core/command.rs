use crate::cli::config::ServerConfig;

pub fn start_server(config: &ServerConfig, args: &Option<String>) {
	println!("Staring server at {}:{}", config.host, config.port);
	args.iter().for_each(|x| println!(" > {x}"))
}
