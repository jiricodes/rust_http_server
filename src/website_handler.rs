use super::server::Handler;
use super::http::{Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		dbg!(request);
		Response::new(StatusCode::Ok, Some("<h1>jiricodes()</h1><p>This is msg from handler!</p>\n".to_string()))
	}
}