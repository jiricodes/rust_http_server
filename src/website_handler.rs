use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		dbg!(request);

		match request.method() {
			Method::GET => match request.path() {
				"/" => Response::new(StatusCode::Ok, Some("<h1>jiricodes()</h1><p>This is msg matched to path in handler!</p>\n".to_string())),
				"/hello" => Response::new(StatusCode::Ok, Some("<h1>jiricodes()</h1><p>Hello!</p>\n".to_string())),
				_ => Response::new(StatusCode::NotFound, None)
			}
			_ => Response::new(StatusCode::NotFound, None)
		}
	}
}