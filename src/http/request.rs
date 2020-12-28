use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Result as FmtResult, Formatter};
	
pub struct Request {
	path: String,
	query: Option<String>,
	method: Method
}

impl TryFrom<&[u8]> for Request {
	type Error = String;
	
	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}

pub enum ParseError {
	InvalidRequest,
	InvalidEncoding,
	InvalidProtocol,
	InvalidMethod,
}

impl ParseError {
	fn message(&self) -> &str {
		match self {
			Self::InvalidRequest => "Invalid Request",
			Self::InvalidEncoding => "Invalid Encoding - not utf8",
			Self::InvalidProtocol => "Invalid Protocol - Only HTTP/1.1 supported",
			Self::InvalidMethod => "Invalid Method - not existing or not implemented"
			}
		}
	}
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Debug for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Error for ParseError {

}