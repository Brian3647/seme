use core::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Isahc(isahc::Error),
	SerdeJson(serde_json::Error),
	IO(std::io::Error),
	Confy(confy::ConfyError),
	Utf8(std::string::FromUtf8Error),
}

impl From<isahc::Error> for Error {
	fn from(e: isahc::Error) -> Self {
		Error::Isahc(e)
	}
}

impl From<serde_json::Error> for Error {
	fn from(e: serde_json::Error) -> Self {
		Error::SerdeJson(e)
	}
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Self {
		Error::IO(e)
	}
}
impl From<confy::ConfyError> for Error {
	fn from(e: confy::ConfyError) -> Self {
		Error::Confy(e)
	}
}

impl From<std::string::FromUtf8Error> for Error {
	fn from(e: std::string::FromUtf8Error) -> Self {
		Error::Utf8(e)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::Isahc(e) => write!(f, "Isahc error: {}", e),
			Error::SerdeJson(e) => write!(f, "SerdeJson error: {}", e),
			Error::IO(e) => write!(f, "IO error: {}", e),
			Error::Confy(e) => write!(f, "Confy error: {}", e),
			Error::Utf8(e) => write!(f, "UTF-8 error: {}", e),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Isahc(e) => Some(e),
			Error::SerdeJson(e) => Some(e),
			Error::IO(e) => Some(e),
			Error::Confy(e) => Some(e),
			Error::Utf8(e) => Some(e),
		}
	}

	fn description(&self) -> &str {
		match self {
			Error::Isahc(_) => "Isahc error",
			Error::SerdeJson(_) => "SerdeJson error",
			Error::IO(_) => "IO error",
			Error::Confy(_) => "Confy error",
			Error::Utf8(_) => "UTF-8 error",
		}
	}

	fn cause(&self) -> Option<&dyn std::error::Error> {
		match self {
			Error::Isahc(e) => Some(e),
			Error::SerdeJson(e) => Some(e),
			Error::IO(e) => Some(e),
			Error::Confy(e) => Some(e),
			Error::Utf8(e) => Some(e),
		}
	}
}
