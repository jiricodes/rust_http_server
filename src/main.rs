struct Server {
	address: String,
}

impl Server {
	fn new(address: String) -> Server {
		Server {
			address
		}
	}
}

fn main() {
	// Initialize Server
	let server: Server = Server::new("127.0.0.1:6969");
	// Run server
	server.run();
}
