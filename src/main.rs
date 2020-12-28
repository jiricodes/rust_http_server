mod server;
use server::Server;

fn main() {
	// Initialize Server
	let server: Server = Server::new("127.0.0.1:6969".to_string());
	// Run server
	server.run();
}
