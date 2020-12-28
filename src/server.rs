use std::net::TcpListener;
use std::io::Read;

pub struct Server {
	address: String,
}

impl Server {
	pub fn new(address: String) -> Server {
		Server {
			address
		}
	}

	// takes ownership since we want to be dealocated upon exit
	pub fn run(self) {
		let listener: TcpListener = TcpListener::bind(&self.address).unwrap();
		println!("Listening at {}", self.address);
		loop {
			match listener.accept() {
				Ok((mut stream, address)) => {
					println!("Connection established with {}", address);
					let mut buffer: [u8; 4096] = [0; 4096];
					match stream.read(&mut buffer) {
						Ok(size) => {
							println!("Read {} bytes:", size);
							println!("{}", String::from_utf8_lossy(&buffer));
						}
						Err(e) => println!("Failed to read from connection: {}", e)
					}
				}
				Err(e) => {
					println!("Failed to establish a connection: {}", e);
				}
			}
			// let result = listener.accept();
			// if result.is_err() {
			// 	println!("Experienced connection error. Continuing.");
			// 	continue ;
			// }
			// let (stream, socket_address) = result.unwrap();
		}
	}
}