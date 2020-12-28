use super::method::Method;
use std::convert::TryFrom;
	
pub struct Request {
	path: String,
	query: Option<String>,
	method: Method
}

impl TryFrom<&[u8]> for Request {
	type Error = String;
	
}