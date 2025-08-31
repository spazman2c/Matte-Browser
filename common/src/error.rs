//! Error handling for the Matte browser.

use thiserror::Error;

/// Result type for Matte browser operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the Matte browser
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("DOM error: {0}")]
    DomError(String),

    #[error("CSS error: {0}")]
    CssError(String),

    #[error("JavaScript error: {0}")]
    JsError(String),

    #[error("Graphics error: {0}")]
    GraphicsError(String),

    #[error("Platform error: {0}")]
    PlatformError(String),

    #[error("IPC error: {0}")]
    IpcError(String),

    #[error("Security error: {0}")]
    SecurityError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Memory error: {0}")]
    MemoryError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ParseError(err.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::ParseError(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::Unknown(err.to_string())
    }
}

impl Error {
    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Error::IoError(_) | Error::NetworkError(_) | Error::Timeout(_)
        )
    }

    /// Check if this is a fatal error
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            Error::MemoryError(_) | Error::SecurityError(_) | Error::InvalidState(_)
        )
    }

    /// Get a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Error::IoError(msg) => format!("File system error: {}", msg),
            Error::NetworkError(msg) => format!("Network error: {}", msg),
            Error::ParseError(msg) => format!("Parse error: {}", msg),
            Error::DomError(msg) => format!("Page error: {}", msg),
            Error::CssError(msg) => format!("Style error: {}", msg),
            Error::JsError(msg) => format!("Script error: {}", msg),
            Error::GraphicsError(msg) => format!("Display error: {}", msg),
            Error::PlatformError(msg) => format!("System error: {}", msg),
            Error::IpcError(msg) => format!("Internal error: {}", msg),
            Error::SecurityError(msg) => format!("Security error: {}", msg),
            Error::ConfigError(msg) => format!("Configuration error: {}", msg),
            Error::InvalidState(msg) => format!("Invalid state: {}", msg),
            Error::NotImplemented(msg) => format!("Feature not available: {}", msg),
            Error::NotFound(msg) => format!("Resource not found: {}", msg),
            Error::PermissionDenied(msg) => format!("Permission denied: {}", msg),
            Error::Timeout(msg) => format!("Operation timed out: {}", msg),
            Error::MemoryError(msg) => format!("Memory error: {}", msg),
            Error::Unknown(msg) => format!("Unknown error: {}", msg),
        }
    }

    /// Get error code for logging/monitoring
    pub fn error_code(&self) -> &'static str {
        match self {
            Error::IoError(_) => "IO_ERROR",
            Error::NetworkError(_) => "NETWORK_ERROR",
            Error::ParseError(_) => "PARSE_ERROR",
            Error::DomError(_) => "DOM_ERROR",
            Error::CssError(_) => "CSS_ERROR",
            Error::JsError(_) => "JS_ERROR",
            Error::GraphicsError(_) => "GRAPHICS_ERROR",
            Error::PlatformError(_) => "PLATFORM_ERROR",
            Error::IpcError(_) => "IPC_ERROR",
            Error::SecurityError(_) => "SECURITY_ERROR",
            Error::ConfigError(_) => "CONFIG_ERROR",
            Error::InvalidState(_) => "INVALID_STATE",
            Error::NotImplemented(_) => "NOT_IMPLEMENTED",
            Error::NotFound(_) => "NOT_FOUND",
            Error::PermissionDenied(_) => "PERMISSION_DENIED",
            Error::Timeout(_) => "TIMEOUT",
            Error::MemoryError(_) => "MEMORY_ERROR",
            Error::Unknown(_) => "UNKNOWN_ERROR",
        }
    }
}

/// Error context for adding additional information
#[derive(Debug)]
pub struct ErrorContext {
    pub error: Error,
    pub context: String,
    pub backtrace: Option<String>,
}

impl ErrorContext {
    pub fn new(error: Error, context: String) -> Self {
        Self {
            error,
            context,
            backtrace: None,
        }
    }

    pub fn with_backtrace(mut self, backtrace: String) -> Self {
        self.backtrace = Some(backtrace);
        self
    }
}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (context: {})", self.error, self.context)?;
        if let Some(ref bt) = self.backtrace {
            write!(f, "\nBacktrace:\n{}", bt)?;
        }
        Ok(())
    }
}

impl std::error::Error for ErrorContext {}

/// Extension trait for adding context to errors
pub trait ErrorExt<T> {
    fn with_context<C>(self, context: C) -> Result<T>
    where
        C: std::fmt::Display;
}

impl<T> ErrorExt<T> for Result<T> {
    fn with_context<C>(self, context: C) -> Result<T>
    where
        C: std::fmt::Display,
    {
        self.map_err(|e| {
            let context_str = format!("{}", context);
            Error::Unknown(format!("{}: {}", context_str, e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_recoverable() {
        assert!(Error::IoError("test".to_string()).is_recoverable());
        assert!(Error::NetworkError("test".to_string()).is_recoverable());
        assert!(Error::Timeout("test".to_string()).is_recoverable());
        assert!(!Error::MemoryError("test".to_string()).is_recoverable());
    }

    #[test]
    fn test_error_fatal() {
        assert!(Error::MemoryError("test".to_string()).is_fatal());
        assert!(Error::SecurityError("test".to_string()).is_fatal());
        assert!(Error::InvalidState("test".to_string()).is_fatal());
        assert!(!Error::IoError("test".to_string()).is_fatal());
    }

    #[test]
    fn test_error_code() {
        assert_eq!(Error::IoError("test".to_string()).error_code(), "IO_ERROR");
        assert_eq!(Error::NetworkError("test".to_string()).error_code(), "NETWORK_ERROR");
        assert_eq!(Error::Unknown("test".to_string()).error_code(), "UNKNOWN_ERROR");
    }

    #[test]
    fn test_error_context() {
        let error = Error::IoError("file not found".to_string());
        let context = ErrorContext::new(error, "loading configuration".to_string());
        assert!(context.to_string().contains("file not found"));
        assert!(context.to_string().contains("loading configuration"));
    }
}
