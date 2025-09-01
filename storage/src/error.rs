use thiserror::Error;

/// Storage error type
#[derive(Error, Debug)]
pub enum Error {
    /// Storage operation failed
    #[error("Storage error: {0}")]
    Storage(String),
    
    /// Database error
    #[error("Database error: {0}")]
    Database(String),
    
    /// Quota exceeded error
    #[error("Storage quota exceeded: {0}")]
    QuotaExceeded(String),
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),
    
    /// Index error
    #[error("Index error: {0}")]
    Index(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    
    /// File system error
    #[error("File system error: {0}")]
    FileSystem(String),
    
    /// Permission error
    #[error("Permission error: {0}")]
    Permission(String),
    
    /// Invalid key error
    #[error("Invalid key: {0}")]
    InvalidKey(String),
    
    /// Invalid value error
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    
    /// Key not found error
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    
    /// Database not found error
    #[error("Database not found: {0}")]
    DatabaseNotFound(String),
    
    /// Object store not found error
    #[error("Object store not found: {0}")]
    ObjectStoreNotFound(String),
    
    /// Index not found error
    #[error("Index not found: {0}")]
    IndexNotFound(String),
    
    /// Version error
    #[error("Version error: {0}")]
    Version(String),
    
    /// Constraint violation error
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    
    /// Timeout error
    #[error("Operation timeout: {0}")]
    Timeout(String),
    
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),
    
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

/// Result type for storage operations
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a storage error
    pub fn storage(message: String) -> Self {
        Error::Storage(message)
    }
    
    /// Create a database error
    pub fn database(message: String) -> Self {
        Error::Database(message)
    }
    
    /// Create a quota exceeded error
    pub fn quota_exceeded(message: String) -> Self {
        Error::QuotaExceeded(message)
    }
    
    /// Create a transaction error
    pub fn transaction(message: String) -> Self {
        Error::Transaction(message)
    }
    
    /// Create an index error
    pub fn index(message: String) -> Self {
        Error::Index(message)
    }
    
    /// Create a serialization error
    pub fn serialization(message: String) -> Self {
        Error::Serialization(message)
    }
    
    /// Create a deserialization error
    pub fn deserialization(message: String) -> Self {
        Error::Deserialization(message)
    }
    
    /// Create a file system error
    pub fn file_system(message: String) -> Self {
        Error::FileSystem(message)
    }
    
    /// Create a permission error
    pub fn permission(message: String) -> Self {
        Error::Permission(message)
    }
    
    /// Create an invalid key error
    pub fn invalid_key(message: String) -> Self {
        Error::InvalidKey(message)
    }
    
    /// Create an invalid value error
    pub fn invalid_value(message: String) -> Self {
        Error::InvalidValue(message)
    }
    
    /// Create a key not found error
    pub fn key_not_found(message: String) -> Self {
        Error::KeyNotFound(message)
    }
    
    /// Create a database not found error
    pub fn database_not_found(message: String) -> Self {
        Error::DatabaseNotFound(message)
    }
    
    /// Create an object store not found error
    pub fn object_store_not_found(message: String) -> Self {
        Error::ObjectStoreNotFound(message)
    }
    
    /// Create an index not found error
    pub fn index_not_found(message: String) -> Self {
        Error::IndexNotFound(message)
    }
    
    /// Create a version error
    pub fn version(message: String) -> Self {
        Error::Version(message)
    }
    
    /// Create a constraint violation error
    pub fn constraint_violation(message: String) -> Self {
        Error::ConstraintViolation(message)
    }
    
    /// Create a timeout error
    pub fn timeout(message: String) -> Self {
        Error::Timeout(message)
    }
    
    /// Create a connection error
    pub fn connection(message: String) -> Self {
        Error::Connection(message)
    }
    
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Timeout(_) => true,
            Error::Connection(_) => true,
            Error::Io(_) => true,
            _ => false,
        }
    }
    
    /// Check if error is fatal
    pub fn is_fatal(&self) -> bool {
        match self {
            Error::Permission(_) => true,
            Error::QuotaExceeded(_) => true,
            Error::ConstraintViolation(_) => true,
            Error::Version(_) => true,
            _ => false,
        }
    }
    
    /// Get error code
    pub fn code(&self) -> &'static str {
        match self {
            Error::Storage(_) => "STORAGE_ERROR",
            Error::Database(_) => "DATABASE_ERROR",
            Error::QuotaExceeded(_) => "QUOTA_EXCEEDED",
            Error::Transaction(_) => "TRANSACTION_ERROR",
            Error::Index(_) => "INDEX_ERROR",
            Error::Serialization(_) => "SERIALIZATION_ERROR",
            Error::Deserialization(_) => "DESERIALIZATION_ERROR",
            Error::FileSystem(_) => "FILE_SYSTEM_ERROR",
            Error::Permission(_) => "PERMISSION_ERROR",
            Error::InvalidKey(_) => "INVALID_KEY",
            Error::InvalidValue(_) => "INVALID_VALUE",
            Error::KeyNotFound(_) => "KEY_NOT_FOUND",
            Error::DatabaseNotFound(_) => "DATABASE_NOT_FOUND",
            Error::ObjectStoreNotFound(_) => "OBJECT_STORE_NOT_FOUND",
            Error::IndexNotFound(_) => "INDEX_NOT_FOUND",
            Error::Version(_) => "VERSION_ERROR",
            Error::ConstraintViolation(_) => "CONSTRAINT_VIOLATION",
            Error::Timeout(_) => "TIMEOUT",
            Error::Connection(_) => "CONNECTION_ERROR",
            Error::Io(_) => "IO_ERROR",
            Error::Json(_) => "JSON_ERROR",
            Error::Uuid(_) => "UUID_ERROR",
        }
    }
    
    /// Get error message
    pub fn message(&self) -> &str {
        match self {
            Error::Storage(msg) => msg,
            Error::Database(msg) => msg,
            Error::QuotaExceeded(msg) => msg,
            Error::Transaction(msg) => msg,
            Error::Index(msg) => msg,
            Error::Serialization(msg) => msg,
            Error::Deserialization(msg) => msg,
            Error::FileSystem(msg) => msg,
            Error::Permission(msg) => msg,
            Error::InvalidKey(msg) => msg,
            Error::InvalidValue(msg) => msg,
            Error::KeyNotFound(msg) => msg,
            Error::DatabaseNotFound(msg) => msg,
            Error::ObjectStoreNotFound(msg) => msg,
            Error::IndexNotFound(msg) => msg,
            Error::Version(msg) => msg,
            Error::ConstraintViolation(msg) => msg,
            Error::Timeout(msg) => msg,
            Error::Connection(msg) => msg,
            Error::Io(err) => err.to_string().as_str(),
            Error::Json(err) => err.to_string().as_str(),
            Error::Uuid(err) => err.to_string().as_str(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Storage(msg) => write!(f, "Storage error: {}", msg),
            Error::Database(msg) => write!(f, "Database error: {}", msg),
            Error::QuotaExceeded(msg) => write!(f, "Storage quota exceeded: {}", msg),
            Error::Transaction(msg) => write!(f, "Transaction error: {}", msg),
            Error::Index(msg) => write!(f, "Index error: {}", msg),
            Error::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            Error::Deserialization(msg) => write!(f, "Deserialization error: {}", msg),
            Error::FileSystem(msg) => write!(f, "File system error: {}", msg),
            Error::Permission(msg) => write!(f, "Permission error: {}", msg),
            Error::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            Error::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            Error::KeyNotFound(msg) => write!(f, "Key not found: {}", msg),
            Error::DatabaseNotFound(msg) => write!(f, "Database not found: {}", msg),
            Error::ObjectStoreNotFound(msg) => write!(f, "Object store not found: {}", msg),
            Error::IndexNotFound(msg) => write!(f, "Index not found: {}", msg),
            Error::Version(msg) => write!(f, "Version error: {}", msg),
            Error::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
            Error::Timeout(msg) => write!(f, "Operation timeout: {}", msg),
            Error::Connection(msg) => write!(f, "Connection error: {}", msg),
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
        let error = Error::storage("Test error".to_string());
        assert_eq!(error.code(), "STORAGE_ERROR");
        assert_eq!(error.message(), "Test error");
        assert!(!error.is_retryable());
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_retryable_error() {
        let error = Error::timeout("Operation timed out".to_string());
        assert!(error.is_retryable());
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_fatal_error() {
        let error = Error::permission("Access denied".to_string());
        assert!(!error.is_retryable());
        assert!(error.is_fatal());
    }

    #[test]
    fn test_error_display() {
        let error = Error::database("Connection failed".to_string());
        let display = format!("{}", error);
        assert_eq!(display, "Database error: Connection failed");
    }
}
