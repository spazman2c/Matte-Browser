//! Main browser application

use common::{error::Result, TabId, WindowInfo, BrowserSettings, BrowserStats};
use tracing::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    window_manager::WindowManager,
    tab_manager::TabManager,
    profile_manager::ProfileManager,
    settings_manager::SettingsManager,
};

/// Main browser application
pub struct BrowserApp {
    /// Window manager
    window_manager: Arc<RwLock<WindowManager>>,
    
    /// Tab manager
    tab_manager: Arc<RwLock<TabManager>>,
    
    /// Profile manager
    profile_manager: Arc<RwLock<ProfileManager>>,
    
    /// Settings manager
    settings_manager: Arc<RwLock<SettingsManager>>,
    
    /// Browser statistics
    stats: Arc<RwLock<BrowserStats>>,
    
    /// Application settings
    settings: BrowserSettings,
    
    /// Running state
    running: bool,
}

impl BrowserApp {
    /// Create a new browser application
    pub async fn new() -> Result<Self> {
        info!("Initializing browser application");
        
        // Initialize managers
        let settings_manager = Arc::new(RwLock::new(SettingsManager::new().await?));
        let profile_manager = Arc::new(RwLock::new(ProfileManager::new().await?));
        let window_manager = Arc::new(RwLock::new(WindowManager::new().await?));
        let tab_manager = Arc::new(RwLock::new(TabManager::new().await?));
        
        // Load settings
        let settings = {
            let settings_mgr = settings_manager.read().await;
            settings_mgr.get_settings().await?
        };
        
        // Initialize statistics
        let stats = Arc::new(RwLock::new(BrowserStats::default()));
        
        info!("Browser application initialized successfully");
        
        Ok(Self {
            window_manager,
            tab_manager,
            profile_manager,
            settings_manager,
            stats,
            settings,
            running: false,
        })
    }
    
    /// Run the browser application
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting browser event loop");
        
        // Create event loop
        let event_loop = EventLoop::new()
            .map_err(|e| common::error::Error::PlatformError(format!("Failed to create event loop: {}", e)))?;
        
        // Create initial window
        let window = self.create_initial_window(&event_loop).await?;
        
        self.running = true;
        
        // Run the event loop
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);
            
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    info!("Window close requested: {:?}", window_id);
                    elwt.exit();
                }
                
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    window_id,
                } => {
                    debug!("Window resized: {:?} -> {:?}", window_id, new_size);
                    // Handle window resize
                }
                
                Event::WindowEvent {
                    event: WindowEvent::Moved(position),
                    window_id,
                } => {
                    debug!("Window moved: {:?} -> {:?}", window_id, position);
                    // Handle window move
                }
                
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { event, .. },
                    window_id,
                } => {
                    debug!("Keyboard input: {:?} -> {:?}", window_id, event);
                    // Handle keyboard input
                }
                
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { state, button, .. },
                    window_id,
                } => {
                    debug!("Mouse input: {:?} -> {:?} {:?}", window_id, state, button);
                    // Handle mouse input
                }
                
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    window_id,
                } => {
                    debug!("Cursor moved: {:?} -> {:?}", window_id, position);
                    // Handle cursor movement
                }
                
                Event::WindowEvent {
                    event: WindowEvent::MouseWheel { delta, .. },
                    window_id,
                } => {
                    debug!("Mouse wheel: {:?} -> {:?}", window_id, delta);
                    // Handle mouse wheel
                }
                
                Event::AboutToWait => {
                    // Handle about to wait
                }
                
                Event::DeviceEvent { .. } => {
                    // Handle device events
                }
                
                Event::Suspended => {
                    // Handle suspended
                }
                
                Event::Resumed => {
                    // Handle resumed
                }
                
                _ => {
                    // Handle other events
                }
            }
        });
        
        self.running = false;
        info!("Browser event loop stopped");
        Ok(())
    }
    
    /// Create the initial browser window
    async fn create_initial_window(&self, event_loop: &EventLoop<()>) -> Result<Window> {
        info!("Creating initial browser window");
        
        let window = WindowBuilder::new()
            .with_title("Matte Browser")
            .with_inner_size(winit::dpi::LogicalSize::new(1200.0, 800.0))
            .with_min_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            .with_resizable(true)
            .with_decorations(true)
            .with_transparent(false)
            .with_visible(true)
            .build(event_loop)
            .map_err(|e| common::error::Error::PlatformError(format!("Failed to create window: {}", e)))?;
        
        // Register window with window manager
        {
            let mut window_mgr = self.window_manager.write().await;
            let window_id = window.id();
            window_mgr.add_window(window_id, WindowInfo::new(1, 1200, 800)).await?;
        }
        
        info!("Initial browser window created successfully");
        Ok(window)
    }
    
    /// Create a new tab
    pub async fn create_tab(&self, window_id: u64, url: Option<String>) -> Result<TabId> {
        info!("Creating new tab in window {}", window_id);
        
        let tab_id = {
            let mut tab_mgr = self.tab_manager.write().await;
            tab_mgr.create_tab(window_id, url).await?
        };
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_tabs += 1;
        }
        
        info!("Created tab {} in window {}", tab_id, window_id);
        Ok(tab_id)
    }
    
    /// Close a tab
    pub async fn close_tab(&self, tab_id: TabId) -> Result<()> {
        info!("Closing tab {}", tab_id);
        
        {
            let mut tab_mgr = self.tab_manager.write().await;
            tab_mgr.close_tab(tab_id).await?;
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_tabs = stats.total_tabs.saturating_sub(1);
        }
        
        info!("Closed tab {}", tab_id);
        Ok(())
    }
    
    /// Navigate a tab to a URL
    pub async fn navigate_tab(&self, tab_id: TabId, url: String) -> Result<()> {
        info!("Navigating tab {} to {}", tab_id, url);
        
        let mut tab_mgr = self.tab_manager.write().await;
        tab_mgr.navigate_tab(tab_id, url).await?;
        
        info!("Navigated tab {} successfully", tab_id);
        Ok(())
    }
    
    /// Get browser statistics
    pub async fn get_stats(&self) -> BrowserStats {
        self.stats.read().await.clone()
    }
    
    /// Get browser settings
    pub fn get_settings(&self) -> &BrowserSettings {
        &self.settings
    }
    
    /// Update browser settings
    pub async fn update_settings(&mut self, new_settings: BrowserSettings) -> Result<()> {
        info!("Updating browser settings");
        
        // Update settings manager
        {
            let mut settings_mgr = self.settings_manager.write().await;
            settings_mgr.update_settings(new_settings.clone()).await?;
        }
        
        self.settings = new_settings;
        
        info!("Browser settings updated successfully");
        Ok(())
    }
    
    /// Check if the browser is running
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    /// Shutdown the browser
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down browser application");
        
        self.running = false;
        
        // Cleanup resources
        {
            let mut tab_mgr = self.tab_manager.write().await;
            tab_mgr.shutdown().await?;
        }
        
        {
            let mut window_mgr = self.window_manager.write().await;
            window_mgr.shutdown().await?;
        }
        
        {
            let mut profile_mgr = self.profile_manager.write().await;
            profile_mgr.shutdown().await?;
        }
        
        {
            let mut settings_mgr = self.settings_manager.write().await;
            settings_mgr.shutdown().await?;
        }
        
        info!("Browser application shutdown complete");
        Ok(())
    }
}

impl Drop for BrowserApp {
    fn drop(&mut self) {
        if self.running {
            warn!("BrowserApp dropped while still running");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_browser_app_creation() {
        let app = BrowserApp::new().await;
        assert!(app.is_ok());
    }

    #[tokio::test]
    async fn test_browser_app_settings() {
        let app = BrowserApp::new().await.unwrap();
        let settings = app.get_settings();
        assert_eq!(settings.homepage, "https://www.google.com");
        assert!(settings.enable_javascript);
    }

    #[tokio::test]
    async fn test_browser_app_stats() {
        let app = BrowserApp::new().await.unwrap();
        let stats = app.get_stats().await;
        assert_eq!(stats.total_tabs, 0);
        assert_eq!(stats.total_windows, 0);
    }
}
