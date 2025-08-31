//! Common utilities and types shared across the Matte browser.

pub mod crash;
pub mod error;
pub mod ipc;
pub mod platform;
pub mod privilege;
pub mod types;
pub mod utils;

pub use error::{Error, Result};
pub use types::*;

use std::fmt;

/// Browser version information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build: String,
}

impl Version {
    pub fn current() -> Self {
        Self {
            major: 0,
            minor: 1,
            patch: 0,
            build: "0.1.0".to_string(),
        }
    }

    pub fn new(major: u32, minor: u32, patch: u32, build: String) -> Self {
        Self {
            major,
            minor,
            patch,
            build,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if !self.build.is_empty() {
            write!(f, "-{}", self.build)?;
        }
        Ok(())
    }
}

/// Process types in the browser architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessType {
    Browser,
    Renderer,
    Network,
    GPU,
    Utility,
}

impl fmt::Display for ProcessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessType::Browser => write!(f, "browser"),
            ProcessType::Renderer => write!(f, "renderer"),
            ProcessType::Network => write!(f, "network"),
            ProcessType::GPU => write!(f, "gpu"),
            ProcessType::Utility => write!(f, "utility"),
        }
    }
}

/// Browser configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub version: Version,
    pub process_type: ProcessType,
    pub enable_logging: bool,
    pub log_level: log::LevelFilter,
    pub enable_crash_reporting: bool,
    pub enable_telemetry: bool,
    pub data_directory: std::path::PathBuf,
    pub temp_directory: std::path::PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: Version::current(),
            process_type: ProcessType::Browser,
            enable_logging: true,
            log_level: log::LevelFilter::Info,
            enable_crash_reporting: true,
            enable_telemetry: false,
            data_directory: default_data_directory(),
            temp_directory: default_temp_directory(),
        }
    }
}

fn default_data_directory() -> std::path::PathBuf {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("C:\\Users\\%USERNAME%\\AppData\\Roaming\\Matte"))
            .join("Matte")
    }
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME")
            .map(|home| std::path::PathBuf::from(home).join("Library/Application Support/Matte"))
            .unwrap_or_else(|_| std::path::PathBuf::from("~/Library/Application Support/Matte"))
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var("XDG_DATA_HOME")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::var("HOME")
                    .map(|home| std::path::PathBuf::from(home).join(".local/share/matte"))
                    .unwrap_or_else(|_| std::path::PathBuf::from("~/.local/share/matte"))
            })
    }
}

fn default_temp_directory() -> std::path::PathBuf {
    std::env::temp_dir().join("matte")
}

/// Initialize the common library with configuration
pub fn init(config: Config) -> Result<()> {
    // Initialize logging
    if config.enable_logging {
        // Use tracing-subscriber for logging
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_target(false)
            .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
            .init();
    }

    // Create directories
    std::fs::create_dir_all(&config.data_directory)
        .map_err(|e| error::Error::IoError(format!("Failed to create data directory: {}", e)))?;
    
    std::fs::create_dir_all(&config.temp_directory)
        .map_err(|e| error::Error::IoError(format!("Failed to create temp directory: {}", e)))?;

    tracing::info!("Matte browser initialized (version: {})", config.version);
    tracing::info!("Process type: {}", config.process_type);
    tracing::info!("Data directory: {:?}", config.data_directory);
    tracing::info!("Temp directory: {:?}", config.temp_directory);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_display() {
        let version = Version::new(1, 2, 3, "alpha".to_string());
        assert_eq!(version.to_string(), "1.2.3-alpha");
        
        let version = Version::new(1, 2, 3, "".to_string());
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_process_type_display() {
        assert_eq!(ProcessType::Browser.to_string(), "browser");
        assert_eq!(ProcessType::Renderer.to_string(), "renderer");
        assert_eq!(ProcessType::Network.to_string(), "network");
        assert_eq!(ProcessType::GPU.to_string(), "gpu");
        assert_eq!(ProcessType::Utility.to_string(), "utility");
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.version, Version::current());
        assert_eq!(config.process_type, ProcessType::Browser);
        assert!(config.enable_logging);
        assert_eq!(config.log_level, log::LevelFilter::Info);
        assert!(config.enable_crash_reporting);
        assert!(!config.enable_telemetry);
    }
}
