use thiserror::Error;

/// JavaScript engine error types
#[derive(Error, Debug)]
pub enum Error {
    /// Lexical analysis error
    #[error("Lexical error at position {position}: {message}")]
    Lexical {
        position: usize,
        message: String,
    },

    /// Syntax parsing error
    #[error("Syntax error at position {position}: {message}")]
    Syntax {
        position: usize,
        message: String,
    },

    /// UTF-8 encoding error
    #[error("UTF-8 encoding error: {message}")]
    Utf8 {
        message: String,
    },

    /// Source map generation error
    #[error("Source map error: {message}")]
    SourceMap {
        message: String,
    },

    /// AST construction error
    #[error("AST error: {message}")]
    Ast {
        message: String,
    },

    /// General parsing error
    #[error("Parsing error: {message}")]
    Parsing {
        message: String,
    },

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// UTF-8 error
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

/// Result type for JavaScript engine operations
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a new lexical error
    pub fn lexical(position: usize, message: impl Into<String>) -> Self {
        Self::Lexical {
            position,
            message: message.into(),
        }
    }

    /// Create a new syntax error
    pub fn syntax(position: usize, message: impl Into<String>) -> Self {
        Self::Syntax {
            position,
            message: message.into(),
        }
    }

    /// Create a new UTF-8 error
    pub fn utf8(message: impl Into<String>) -> Self {
        Self::Utf8 {
            message: message.into(),
        }
    }

    /// Create a new source map error
    pub fn source_map(message: impl Into<String>) -> Self {
        Self::SourceMap {
            message: message.into(),
        }
    }

    /// Create a new AST error
    pub fn ast(message: impl Into<String>) -> Self {
        Self::Ast {
            message: message.into(),
        }
    }

    /// Create a new parsing error
    pub fn parsing(message: impl Into<String>) -> Self {
        Self::Parsing {
            message: message.into(),
        }
    }
}
