use std::error::Error as StdError;
use std::fmt;

/// Graphics error type
#[derive(Debug)]
pub enum Error {
    /// Generic graphics error
    Graphics(String),
    /// Rendering error
    Rendering(String),
    /// Compositor error
    Compositor(String),
    /// Hardware acceleration error
    HardwareAcceleration(String),
    /// Window management error
    WindowManagement(String),
    /// Shader compilation error
    ShaderCompilation(String),
    /// Texture error
    Texture(String),
    /// Buffer error
    Buffer(String),
    /// GPU context error
    GpuContext(String),
    /// Vsync error
    Vsync(String),
    /// Layer management error
    LayerManagement(String),
    /// IO error
    Io(std::io::Error),
    /// Serialization error
    Serialization(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Graphics(msg) => write!(f, "Graphics error: {}", msg),
            Error::Rendering(msg) => write!(f, "Rendering error: {}", msg),
            Error::Compositor(msg) => write!(f, "Compositor error: {}", msg),
            Error::HardwareAcceleration(msg) => write!(f, "Hardware acceleration error: {}", msg),
            Error::WindowManagement(msg) => write!(f, "Window management error: {}", msg),
            Error::ShaderCompilation(msg) => write!(f, "Shader compilation error: {}", msg),
            Error::Texture(msg) => write!(f, "Texture error: {}", msg),
            Error::Buffer(msg) => write!(f, "Buffer error: {}", msg),
            Error::GpuContext(msg) => write!(f, "GPU context error: {}", msg),
            Error::Vsync(msg) => write!(f, "Vsync error: {}", msg),
            Error::LayerManagement(msg) => write!(f, "Layer management error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Serialization(msg) => write!(f, "Serialization error: {}", msg),
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}

impl From<Box<dyn StdError + Send + Sync>> for Error {
    fn from(err: Box<dyn StdError + Send + Sync>) -> Self {
        Error::Graphics(err.to_string())
    }
}

/// Result type for graphics operations
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a new graphics error
    pub fn graphics(msg: String) -> Self {
        Error::Graphics(msg)
    }

    /// Create a new rendering error
    pub fn rendering(msg: String) -> Self {
        Error::Rendering(msg)
    }

    /// Create a new compositor error
    pub fn compositor(msg: String) -> Self {
        Error::Compositor(msg)
    }

    /// Create a new hardware acceleration error
    pub fn hardware_acceleration(msg: String) -> Self {
        Error::HardwareAcceleration(msg)
    }

    /// Create a new window management error
    pub fn window_management(msg: String) -> Self {
        Error::WindowManagement(msg)
    }

    /// Create a new shader compilation error
    pub fn shader_compilation(msg: String) -> Self {
        Error::ShaderCompilation(msg)
    }

    /// Create a new texture error
    pub fn texture(msg: String) -> Self {
        Error::Texture(msg)
    }

    /// Create a new buffer error
    pub fn buffer(msg: String) -> Self {
        Error::Buffer(msg)
    }

    /// Create a new GPU context error
    pub fn gpu_context(msg: String) -> Self {
        Error::GpuContext(msg)
    }

    /// Create a new vsync error
    pub fn vsync(msg: String) -> Self {
        Error::Vsync(msg)
    }

    /// Create a new layer management error
    pub fn layer_management(msg: String) -> Self {
        Error::LayerManagement(msg)
    }
}
