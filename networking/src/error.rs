use std::fmt;
use std::error::Error as StdError;

/// Networking error types
#[derive(Debug)]
pub enum Error {
    /// Parsing errors
    Parsing(String),
    /// Network errors
    Network(String),
    /// HTTP errors
    Http(String),
    /// Timeout errors
    Timeout(String),
    /// Connection errors
    Connection(String),
    /// Protocol errors
    Protocol(String),
    /// IO errors
    Io(std::io::Error),
    /// URL errors
    Url(String),
    /// SSL/TLS errors
    Ssl(String),
    /// DNS resolution errors
    Dns(String),
    /// Configuration errors
    Config(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parsing(msg) => write!(f, "Parsing error: {}", msg),
            Error::Network(msg) => write!(f, "Network error: {}", msg),
            Error::Http(msg) => write!(f, "HTTP error: {}", msg),
            Error::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            Error::Connection(msg) => write!(f, "Connection error: {}", msg),
            Error::Protocol(msg) => write!(f, "Protocol error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Url(msg) => write!(f, "URL error: {}", msg),
            Error::Ssl(msg) => write!(f, "SSL/TLS error: {}", msg),
            Error::Dns(msg) => write!(f, "DNS error: {}", msg),
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::Network(format!("Address parse error: {}", err))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parsing(format!("Parse int error: {}", err))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Parsing(format!("UTF-8 error: {}", err))
    }
}

impl Error {
    /// Create parsing error
    pub fn parsing<T: Into<String>>(msg: T) -> Self {
        Error::Parsing(msg.into())
    }

    /// Create network error
    pub fn network<T: Into<String>>(msg: T) -> Self {
        Error::Network(msg.into())
    }

    /// Create HTTP error
    pub fn http<T: Into<String>>(msg: T) -> Self {
        Error::Http(msg.into())
    }

    /// Create timeout error
    pub fn timeout<T: Into<String>>(msg: T) -> Self {
        Error::Timeout(msg.into())
    }

    /// Create connection error
    pub fn connection<T: Into<String>>(msg: T) -> Self {
        Error::Connection(msg.into())
    }

    /// Create protocol error
    pub fn protocol<T: Into<String>>(msg: T) -> Self {
        Error::Protocol(msg.into())
    }

    /// Create URL error
    pub fn url<T: Into<String>>(msg: T) -> Self {
        Error::Url(msg.into())
    }

    /// Create SSL/TLS error
    pub fn ssl<T: Into<String>>(msg: T) -> Self {
        Error::Ssl(msg.into())
    }

    /// Create DNS error
    pub fn dns<T: Into<String>>(msg: T) -> Self {
        Error::Dns(msg.into())
    }

    /// Create configuration error
    pub fn config<T: Into<String>>(msg: T) -> Self {
        Error::Config(msg.into())
    }
}

/// Result type for networking operations
pub type Result<T> = std::result::Result<T, Error>;
