
// Adapted from https://github.com/paritytech/parity-ethereum/blob/master/ethkey/src/error.rs
use std::{fmt, error};



#[derive(Debug)]
pub enum Error {
	InvalidSecretKey,
	InvalidPublicKey,
	InvalidBufferLength,
	Io(::std::io::Error),
	HexIo(::hex::FromHexError),
	RustcHexIo(::rustc_hex::FromHexError),
	Custom(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let msg = match *self {
			Error::InvalidSecretKey => "Invalid secret key".into(),
			Error::InvalidPublicKey => "Invalid public key".into(),
			Error::InvalidBufferLength => "Invalid buffer length".into(),
			Error::Io(ref err) => format!("I/O error: {}", err),
			Error::Custom(ref s) => s.clone(),
			Error::HexIo(ref err) => format!("Hex error: {}", err),
			Error::RustcHexIo(ref err) => format!("RustcHex error: {}", err),
		};

		f.write_fmt(format_args!("Crypto error ({})", msg))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		"Crypto error"
	}
}

impl Into<String> for Error {
	fn into(self) -> String {
		format!("{}", self)
	}
}

impl From<::std::io::Error> for Error {
	fn from(err: ::std::io::Error) -> Error {
		Error::Io(err)
	}
}

impl From<::rustc_hex::FromHexError> for Error {
    fn from(err: ::rustc_hex::FromHexError) -> Error {
		Error::RustcHexIo(err)
	}
}

impl From<::hex::FromHexError> for Error {
    fn from(err: ::hex::FromHexError) -> Error {
		Error::HexIo(err)
	}
}