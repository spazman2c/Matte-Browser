use thiserror::Error;

/// Accessibility error type
#[derive(Error, Debug)]
pub enum Error {
    /// Accessibility tree error
    #[error("Accessibility tree error: {0}")]
    AccessibilityTree(String),
    
    /// Input handler error
    #[error("Input handler error: {0}")]
    InputHandler(String),
    
    /// Keyboard error
    #[error("Keyboard error: {0}")]
    Keyboard(String),
    
    /// Mouse error
    #[error("Mouse error: {0}")]
    Mouse(String),
    
    /// Touch error
    #[error("Touch error: {0}")]
    Touch(String),
    
    /// Gesture error
    #[error("Gesture error: {0}")]
    Gesture(String),
    
    /// Navigation error
    #[error("Navigation error: {0}")]
    Navigation(String),
    
    /// Focus error
    #[error("Focus error: {0}")]
    Focus(String),
    
    /// ARIA error
    #[error("ARIA error: {0}")]
    Aria(String),
    
    /// Event error
    #[error("Event error: {0}")]
    Event(String),
    
    /// Node not found error
    #[error("Node not found: {0}")]
    NodeNotFound(String),
    
    /// Invalid role error
    #[error("Invalid role: {0}")]
    InvalidRole(String),
    
    /// Invalid state error
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    /// Invalid input error
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// Invalid gesture error
    #[error("Invalid gesture: {0}")]
    InvalidGesture(String),
    
    /// Invalid key binding error
    #[error("Invalid key binding: {0}")]
    InvalidKeyBinding(String),
    
    /// Invalid event error
    #[error("Invalid event: {0}")]
    InvalidEvent(String),
    
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

/// Result type for accessibility operations
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create an accessibility tree error
    pub fn accessibility_tree(message: String) -> Self {
        Error::AccessibilityTree(message)
    }
    
    /// Create an input handler error
    pub fn input_handler(message: String) -> Self {
        Error::InputHandler(message)
    }
    
    /// Create a keyboard error
    pub fn keyboard(message: String) -> Self {
        Error::Keyboard(message)
    }
    
    /// Create a mouse error
    pub fn mouse(message: String) -> Self {
        Error::Mouse(message)
    }
    
    /// Create a touch error
    pub fn touch(message: String) -> Self {
        Error::Touch(message)
    }
    
    /// Create a gesture error
    pub fn gesture(message: String) -> Self {
        Error::Gesture(message)
    }
    
    /// Create a navigation error
    pub fn navigation(message: String) -> Self {
        Error::Navigation(message)
    }
    
    /// Create a focus error
    pub fn focus(message: String) -> Self {
        Error::Focus(message)
    }
    
    /// Create an ARIA error
    pub fn aria(message: String) -> Self {
        Error::Aria(message)
    }
    
    /// Create an event error
    pub fn event(message: String) -> Self {
        Error::Event(message)
    }
    
    /// Create a node not found error
    pub fn node_not_found(message: String) -> Self {
        Error::NodeNotFound(message)
    }
    
    /// Create an invalid role error
    pub fn invalid_role(message: String) -> Self {
        Error::InvalidRole(message)
    }
    
    /// Create an invalid state error
    pub fn invalid_state(message: String) -> Self {
        Error::InvalidState(message)
    }
    
    /// Create an invalid input error
    pub fn invalid_input(message: String) -> Self {
        Error::InvalidInput(message)
    }
    
    /// Create an invalid gesture error
    pub fn invalid_gesture(message: String) -> Self {
        Error::InvalidGesture(message)
    }
    
    /// Create an invalid key binding error
    pub fn invalid_key_binding(message: String) -> Self {
        Error::InvalidKeyBinding(message)
    }
    
    /// Create an invalid event error
    pub fn invalid_event(message: String) -> Self {
        Error::InvalidEvent(message)
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
            Error::Event(_) => true,
            Error::InputHandler(_) => true,
            _ => false,
        }
    }
    
    /// Check if error is fatal
    pub fn is_fatal(&self) -> bool {
        match self {
            Error::NodeNotFound(_) => true,
            Error::InvalidRole(_) => true,
            Error::InvalidState(_) => true,
            Error::InvalidInput(_) => true,
            Error::InvalidGesture(_) => true,
            Error::InvalidKeyBinding(_) => true,
            Error::InvalidEvent(_) => true,
            _ => false,
        }
    }
    
    /// Get error code
    pub fn code(&self) -> &'static str {
        match self {
            Error::AccessibilityTree(_) => "ACCESSIBILITY_TREE_ERROR",
            Error::InputHandler(_) => "INPUT_HANDLER_ERROR",
            Error::Keyboard(_) => "KEYBOARD_ERROR",
            Error::Mouse(_) => "MOUSE_ERROR",
            Error::Touch(_) => "TOUCH_ERROR",
            Error::Gesture(_) => "GESTURE_ERROR",
            Error::Navigation(_) => "NAVIGATION_ERROR",
            Error::Focus(_) => "FOCUS_ERROR",
            Error::Aria(_) => "ARIA_ERROR",
            Error::Event(_) => "EVENT_ERROR",
            Error::NodeNotFound(_) => "NODE_NOT_FOUND",
            Error::InvalidRole(_) => "INVALID_ROLE",
            Error::InvalidState(_) => "INVALID_STATE",
            Error::InvalidInput(_) => "INVALID_INPUT",
            Error::InvalidGesture(_) => "INVALID_GESTURE",
            Error::InvalidKeyBinding(_) => "INVALID_KEY_BINDING",
            Error::InvalidEvent(_) => "INVALID_EVENT",
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
            Error::AccessibilityTree(msg) => msg,
            Error::InputHandler(msg) => msg,
            Error::Keyboard(msg) => msg,
            Error::Mouse(msg) => msg,
            Error::Touch(msg) => msg,
            Error::Gesture(msg) => msg,
            Error::Navigation(msg) => msg,
            Error::Focus(msg) => msg,
            Error::Aria(msg) => msg,
            Error::Event(msg) => msg,
            Error::NodeNotFound(msg) => msg,
            Error::InvalidRole(msg) => msg,
            Error::InvalidState(msg) => msg,
            Error::InvalidInput(msg) => msg,
            Error::InvalidGesture(msg) => msg,
            Error::InvalidKeyBinding(msg) => msg,
            Error::InvalidEvent(msg) => msg,
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
            Error::AccessibilityTree(msg) => write!(f, "Accessibility tree error: {}", msg),
            Error::InputHandler(msg) => write!(f, "Input handler error: {}", msg),
            Error::Keyboard(msg) => write!(f, "Keyboard error: {}", msg),
            Error::Mouse(msg) => write!(f, "Mouse error: {}", msg),
            Error::Touch(msg) => write!(f, "Touch error: {}", msg),
            Error::Gesture(msg) => write!(f, "Gesture error: {}", msg),
            Error::Navigation(msg) => write!(f, "Navigation error: {}", msg),
            Error::Focus(msg) => write!(f, "Focus error: {}", msg),
            Error::Aria(msg) => write!(f, "ARIA error: {}", msg),
            Error::Event(msg) => write!(f, "Event error: {}", msg),
            Error::NodeNotFound(msg) => write!(f, "Node not found: {}", msg),
            Error::InvalidRole(msg) => write!(f, "Invalid role: {}", msg),
            Error::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Error::InvalidGesture(msg) => write!(f, "Invalid gesture: {}", msg),
            Error::InvalidKeyBinding(msg) => write!(f, "Invalid key binding: {}", msg),
            Error::InvalidEvent(msg) => write!(f, "Invalid event: {}", msg),
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
        let error = Error::accessibility_tree("Test error".to_string());
        assert_eq!(error.code(), "ACCESSIBILITY_TREE_ERROR");
        assert_eq!(error.message(), "Test error");
        assert!(!error.is_retryable());
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_retryable_error() {
        let error = Error::event("Event processing failed".to_string());
        assert!(error.is_retryable());
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_fatal_error() {
        let error = Error::node_not_found("Node not found".to_string());
        assert!(!error.is_retryable());
        assert!(error.is_fatal());
    }

    #[test]
    fn test_error_display() {
        let error = Error::keyboard("Keyboard error".to_string());
        let display = format!("{}", error);
        assert_eq!(display, "Keyboard error: Keyboard error");
    }
}
