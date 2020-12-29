#![allow(dead_code)]

use std::env;

mod server;
use server::Server;

mod website_handler;
use website_handler::WebsiteHandler;

mod http;

fn main() {
	// set default path - at compile time use Cargo.toml location + /public
	let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
	// get public path
	let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
	dbg!(&public_path);
	// Initialize Server
	let server: Server = Server::new("127.0.0.1:6969".to_string());
	// Run server
	server.run(WebsiteHandler::new(public_path));
}
