//! Matte Browser - Main entry point

use common::{Config, ProcessType, Version, init as init_common};
use tracing::{error, info, warn};
use std::process;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod app;
mod error;
mod navigation;
mod window_manager;
mod tab_manager;
mod profile_manager;
mod settings_manager;

use app::BrowserApp;

/// Main function - entry point for the Matte browser
#[tokio::main]
async fn main() {
    // Initialize the browser
    if let Err(e) = run_browser().await {
        error!("Failed to start Matte browser: {}", e);
        process::exit(1);
    }
}

/// Run the browser application
async fn run_browser() -> common::Result<()> {
    // Initialize common library
    let config = Config {
        process_type: ProcessType::Browser,
        enable_logging: true,
        log_level: log::LevelFilter::Info,
        ..Default::default()
    };
    
    init_common(config)?;
    
    info!("Starting Matte Browser v{}", Version::current());
    info!("Platform: {} {}", std::env::consts::OS, std::env::consts::ARCH);
    
    // Create the browser application
    let mut app = BrowserApp::new().await?;
    
    // Run the event loop
    app.run().await?;
    
    info!("Matte Browser shutting down");
    Ok(())
}

/// Handle command line arguments
fn parse_args() -> clap::ArgMatches {
    use clap::{Command, Arg};
    
    Command::new("Matte Browser")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A fast, private, and developer-friendly web browser")
        .arg(
            Arg::new("url")
                .help("URL to open")
                .value_name("URL")
                .index(1)
        )
        .arg(
            Arg::new("incognito")
                .short('i')
                .long("incognito")
                .help("Start in incognito mode")
        )
        .arg(
            Arg::new("profile")
                .short('p')
                .long("profile")
                .help("Profile to use")
                .value_name("PROFILE")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug mode")
        )
        .get_matches()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_browser_initialization() {
        // Test that the browser can be initialized
        let config = Config {
            process_type: ProcessType::Browser,
            enable_logging: false, // Disable logging for tests
            ..Default::default()
        };
        
        assert!(init_common(config).is_ok());
    }
}
