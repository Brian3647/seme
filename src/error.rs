use core::fmt;

#[derive(Debug)]
pub enum Error {
	Isahc(isahc::Error),
	SerdeJson(serde_json::Error),
	IO(std::io::Error),
	Confy(confy::ConfyError),
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

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::Isahc(e) => write!(f, "Isahc error: {}", e),
			Error::SerdeJson(e) => write!(f, "SerdeJson error: {}", e),
			Error::IO(e) => write!(f, "IO error: {}", e),
			Error::Confy(e) => write!(f, "Confy error: {}", e),
		}
	}
}

impl std::error::Error for Error {}
