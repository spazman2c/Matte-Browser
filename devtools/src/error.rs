use thiserror::Error;

/// DevTools error type
#[derive(Error, Debug)]
pub enum Error {
    /// Inspector error
    #[error("Inspector error: {0}")]
    Inspector(String),
    
    /// DOM error
    #[error("DOM error: {0}")]
    Dom(String),
    
    /// Style error
    #[error("Style error: {0}")]
    Style(String),
    
    /// Console error
    #[error("Console error: {0}")]
    Console(String),
    
    /// Network error
    #[error("Network error: {0}")]
    Network(String),
    
    /// Performance error
    #[error("Performance error: {0}")]
    Performance(String),
    
    /// Evaluation error
    #[error("Evaluation error: {0}")]
    Evaluation(String),
    
    /// Source map error
    #[error("Source map error: {0}")]
    SourceMap(String),
    
    /// Stack trace error
    #[error("Stack trace error: {0}")]
    StackTrace(String),
    
    /// Filter error
    #[error("Filter error: {0}")]
    Filter(String),
    
    /// Element not found error
    #[error("Element not found: {0}")]
    ElementNotFound(String),
    
    /// Style not found error
    #[error("Style not found: {0}")]
    StyleNotFound(String),
    
    /// Console message not found error
    #[error("Console message not found: {0}")]
    ConsoleMessageNotFound(String),
    
    /// Network request not found error
    #[error("Network request not found: {0}")]
    NetworkRequestNotFound(String),
    
    /// Performance entry not found error
    #[error("Performance entry not found: {0}")]
    PerformanceEntryNotFound(String),
    
    /// Invalid selector error
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),
    
    /// Invalid CSS property error
    #[error("Invalid CSS property: {0}")]
    InvalidCssProperty(String),
    
    /// Invalid expression error
    #[error("Invalid expression: {0}")]
    InvalidExpression(String),
    
    /// Invalid source map error
    #[error("Invalid source map: {0}")]
    InvalidSourceMap(String),
    
    /// Invalid stack trace error
    #[error("Invalid stack trace: {0}")]
    InvalidStackTrace(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// UUID error
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),
}

/// Result type for DevTools operations
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create an inspector error
    pub fn inspector(message: String) -> Self {
        Error::Inspector(message)
    }
    
    /// Create a DOM error
    pub fn dom(message: String) -> Self {
        Error::Dom(message)
    }
    
    /// Create a style error
    pub fn style(message: String) -> Self {
        Error::Style(message)
    }
    
    /// Create a console error
    pub fn console(message: String) -> Self {
        Error::Console(message)
    }
    
    /// Create a network error
    pub fn network(message: String) -> Self {
        Error::Network(message)
    }
    
    /// Create a performance error
    pub fn performance(message: String) -> Self {
        Error::Performance(message)
    }
    
    /// Create an evaluation error
    pub fn evaluation(message: String) -> Self {
        Error::Evaluation(message)
    }
    
    /// Create a source map error
    pub fn source_map(message: String) -> Self {
        Error::SourceMap(message)
    }
    
    /// Create a stack trace error
    pub fn stack_trace(message: String) -> Self {
        Error::StackTrace(message)
    }
    
    /// Create a filter error
    pub fn filter(message: String) -> Self {
        Error::Filter(message)
    }
    
    /// Create an element not found error
    pub fn element_not_found(message: String) -> Self {
        Error::ElementNotFound(message)
    }
    
    /// Create a style not found error
    pub fn style_not_found(message: String) -> Self {
        Error::StyleNotFound(message)
    }
    
    /// Create a console message not found error
    pub fn console_message_not_found(message: String) -> Self {
        Error::ConsoleMessageNotFound(message)
    }
    
    /// Create a network request not found error
    pub fn network_request_not_found(message: String) -> Self {
        Error::NetworkRequestNotFound(message)
    }
    
    /// Create a performance entry not found error
    pub fn performance_entry_not_found(message: String) -> Self {
        Error::PerformanceEntryNotFound(message)
    }
    
    /// Create an invalid selector error
    pub fn invalid_selector(message: String) -> Self {
        Error::InvalidSelector(message)
    }
    
    /// Create an invalid CSS property error
    pub fn invalid_css_property(message: String) -> Self {
        Error::InvalidCssProperty(message)
    }
    
    /// Create an invalid expression error
    pub fn invalid_expression(message: String) -> Self {
        Error::InvalidExpression(message)
    }
    
    /// Create an invalid source map error
    pub fn invalid_source_map(message: String) -> Self {
        Error::InvalidSourceMap(message)
    }
    
    /// Create an invalid stack trace error
    pub fn invalid_stack_trace(message: String) -> Self {
        Error::InvalidStackTrace(message)
    }
    
    /// Create a serialization error
    pub fn serialization(message: String) -> Self {
        Error::Serialization(message)
    }
    
    /// Create a deserialization error
    pub fn deserialization(message: String) -> Self {
        Error::Deserialization(message)
    }
    
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Io(_) => true,
            Error::Network(_) => true,
            Error::Performance(_) => true,
            _ => false,
        }
    }
    
    /// Check if error is fatal
    pub fn is_fatal(&self) -> bool {
        match self {
            Error::ElementNotFound(_) => true,
            Error::StyleNotFound(_) => true,
            Error::ConsoleMessageNotFound(_) => true,
            Error::NetworkRequestNotFound(_) => true,
            Error::PerformanceEntryNotFound(_) => true,
            Error::InvalidSelector(_) => true,
            Error::InvalidCssProperty(_) => true,
            Error::InvalidExpression(_) => true,
            Error::InvalidSourceMap(_) => true,
            Error::InvalidStackTrace(_) => true,
            _ => false,
        }
    }
    
    /// Get error code
    pub fn code(&self) -> &'static str {
        match self {
            Error::Inspector(_) => "INSPECTOR_ERROR",
            Error::Dom(_) => "DOM_ERROR",
            Error::Style(_) => "STYLE_ERROR",
            Error::Console(_) => "CONSOLE_ERROR",
            Error::Network(_) => "NETWORK_ERROR",
            Error::Performance(_) => "PERFORMANCE_ERROR",
            Error::Evaluation(_) => "EVALUATION_ERROR",
            Error::SourceMap(_) => "SOURCE_MAP_ERROR",
            Error::StackTrace(_) => "STACK_TRACE_ERROR",
            Error::Filter(_) => "FILTER_ERROR",
            Error::ElementNotFound(_) => "ELEMENT_NOT_FOUND",
            Error::StyleNotFound(_) => "STYLE_NOT_FOUND",
            Error::ConsoleMessageNotFound(_) => "CONSOLE_MESSAGE_NOT_FOUND",
            Error::NetworkRequestNotFound(_) => "NETWORK_REQUEST_NOT_FOUND",
            Error::PerformanceEntryNotFound(_) => "PERFORMANCE_ENTRY_NOT_FOUND",
            Error::InvalidSelector(_) => "INVALID_SELECTOR",
            Error::InvalidCssProperty(_) => "INVALID_CSS_PROPERTY",
            Error::InvalidExpression(_) => "INVALID_EXPRESSION",
            Error::InvalidSourceMap(_) => "INVALID_SOURCE_MAP",
            Error::InvalidStackTrace(_) => "INVALID_STACK_TRACE",
            Error::Serialization(_) => "SERIALIZATION_ERROR",
            Error::Deserialization(_) => "DESERIALIZATION_ERROR",
            Error::Io(_) => "IO_ERROR",
            Error::Json(_) => "JSON_ERROR",
            Error::Uuid(_) => "UUID_ERROR",
        }
    }
    
    /// Get error message
    pub fn message(&self) -> &str {
        match self {
            Error::Inspector(msg) => msg,
            Error::Dom(msg) => msg,
            Error::Style(msg) => msg,
            Error::Console(msg) => msg,
            Error::Network(msg) => msg,
            Error::Performance(msg) => msg,
            Error::Evaluation(msg) => msg,
            Error::SourceMap(msg) => msg,
            Error::StackTrace(msg) => msg,
            Error::Filter(msg) => msg,
            Error::ElementNotFound(msg) => msg,
            Error::StyleNotFound(msg) => msg,
            Error::ConsoleMessageNotFound(msg) => msg,
            Error::NetworkRequestNotFound(msg) => msg,
            Error::PerformanceEntryNotFound(msg) => msg,
            Error::InvalidSelector(msg) => msg,
            Error::InvalidCssProperty(msg) => msg,
            Error::InvalidExpression(msg) => msg,
            Error::InvalidSourceMap(msg) => msg,
            Error::InvalidStackTrace(msg) => msg,
            Error::Serialization(msg) => msg,
            Error::Deserialization(msg) => msg,
            Error::Io(err) => err.to_string().as_str(),
            Error::Json(err) => err.to_string().as_str(),
            Error::Uuid(err) => err.to_string().as_str(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Inspector(msg) => write!(f, "Inspector error: {}", msg),
            Error::Dom(msg) => write!(f, "DOM error: {}", msg),
            Error::Style(msg) => write!(f, "Style error: {}", msg),
            Error::Console(msg) => write!(f, "Console error: {}", msg),
            Error::Network(msg) => write!(f, "Network error: {}", msg),
            Error::Performance(msg) => write!(f, "Performance error: {}", msg),
            Error::Evaluation(msg) => write!(f, "Evaluation error: {}", msg),
            Error::SourceMap(msg) => write!(f, "Source map error: {}", msg),
            Error::StackTrace(msg) => write!(f, "Stack trace error: {}", msg),
            Error::Filter(msg) => write!(f, "Filter error: {}", msg),
            Error::ElementNotFound(msg) => write!(f, "Element not found: {}", msg),
            Error::StyleNotFound(msg) => write!(f, "Style not found: {}", msg),
            Error::ConsoleMessageNotFound(msg) => write!(f, "Console message not found: {}", msg),
            Error::NetworkRequestNotFound(msg) => write!(f, "Network request not found: {}", msg),
            Error::PerformanceEntryNotFound(msg) => write!(f, "Performance entry not found: {}", msg),
            Error::InvalidSelector(msg) => write!(f, "Invalid selector: {}", msg),
            Error::InvalidCssProperty(msg) => write!(f, "Invalid CSS property: {}", msg),
            Error::InvalidExpression(msg) => write!(f, "Invalid expression: {}", msg),
            Error::InvalidSourceMap(msg) => write!(f, "Invalid source map: {}", msg),
            Error::InvalidStackTrace(msg) => write!(f, "Invalid stack trace: {}", msg),
            Error::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            Error::Deserialization(msg) => write!(f, "Deserialization error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Json(err) => write!(f, "JSON error: {}", err),
            Error::Uuid(err) => write!(f, "UUID error: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = Error::inspector("Test error".to_string());
        assert_eq!(error.code(), "INSPECTOR_ERROR");
        assert_eq!(error.message(), "Test error");
        assert!(!error.is_retryable());
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_retryable_error() {
        let error = Error::network("Connection failed".to_string());
        assert!(error.is_retryable());
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_fatal_error() {
        let error = Error::element_not_found("Element not found".to_string());
        assert!(!error.is_retryable());
        assert!(error.is_fatal());
    }

    #[test]
    fn test_error_display() {
        let error = Error::console("Console error".to_string());
        let display = format!("{}", error);
        assert_eq!(display, "Console error: Console error");
    }
}
