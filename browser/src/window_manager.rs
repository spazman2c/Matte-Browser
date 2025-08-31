//! Window manager for the Matte browser

use common::{error::Result, WindowInfo};
use tracing::{debug, error, info, warn};
use std::collections::HashMap;
use winit::window::WindowId;

/// Window manager for handling multiple browser windows
pub struct WindowManager {
    /// Map of window ID to window info
    windows: HashMap<WindowId, WindowInfo>,
    
    /// Next window ID
    next_window_id: u64,
}

impl WindowManager {
    /// Create a new window manager
    pub async fn new() -> Result<Self> {
        info!("Initializing window manager");
        
        Ok(Self {
            windows: HashMap::new(),
            next_window_id: 1,
        })
    }
    
    /// Add a new window
    pub async fn add_window(&mut self, window_id: WindowId, info: WindowInfo) -> Result<()> {
        info!("Adding window {} with ID {:?}", info.id, window_id);
        
        if self.windows.contains_key(&window_id) {
            return Err(common::error::Error::InvalidState(
                format!("Window with ID {:?} already exists", window_id)
            ));
        }
        
        self.windows.insert(window_id, info);
        self.next_window_id += 1;
        
        info!("Added window successfully");
        Ok(())
    }
    
    /// Remove a window
    pub async fn remove_window(&mut self, window_id: WindowId) -> Result<()> {
        info!("Removing window {:?}", window_id);
        
        if let Some(window_info) = self.windows.remove(&window_id) {
            info!("Removed window {} successfully", window_info.id);
            Ok(())
        } else {
            Err(common::error::Error::NotFound(
                format!("Window with ID {:?} not found", window_id)
            ))
        }
    }
    
    /// Get window info
    pub async fn get_window(&self, window_id: WindowId) -> Result<&WindowInfo> {
        self.windows.get(&window_id)
            .ok_or_else(|| common::error::Error::NotFound(
                format!("Window with ID {:?} not found", window_id)
            ))
    }
    
    /// Get window info mutably
    pub async fn get_window_mut(&mut self, window_id: WindowId) -> Result<&mut WindowInfo> {
        self.windows.get_mut(&window_id)
            .ok_or_else(|| common::error::Error::NotFound(
                format!("Window with ID {:?} not found", window_id)
            ))
    }
    
    /// Update window info
    pub async fn update_window(&mut self, window_id: WindowId, info: WindowInfo) -> Result<()> {
        info!("Updating window {:?}", window_id);
        
        if let Some(existing_info) = self.windows.get_mut(&window_id) {
            *existing_info = info;
            info!("Updated window successfully");
            Ok(())
        } else {
            Err(common::error::Error::NotFound(
                format!("Window with ID {:?} not found", window_id)
            ))
        }
    }
    
    /// Get all windows
    pub async fn get_all_windows(&self) -> Vec<&WindowInfo> {
        self.windows.values().collect()
    }
    
    /// Get window count
    pub async fn window_count(&self) -> usize {
        self.windows.len()
    }
    
    /// Check if window exists
    pub async fn has_window(&self, window_id: WindowId) -> bool {
        self.windows.contains_key(&window_id)
    }
    
    /// Get next window ID
    pub async fn next_window_id(&self) -> u64 {
        self.next_window_id
    }
    
    /// Shutdown the window manager
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down window manager");
        
        let window_count = self.windows.len();
        self.windows.clear();
        
        info!("Window manager shutdown complete (closed {} windows)", window_count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::window::WindowId;

    #[tokio::test]
    async fn test_window_manager_creation() {
        let manager = WindowManager::new().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_add_window() {
        let mut manager = WindowManager::new().await.unwrap();
        let window_id = unsafe { WindowId::dummy() };
        let window_info = WindowInfo::new(1, 800, 600);
        
        assert!(manager.add_window(window_id, window_info).await.is_ok());
        assert_eq!(manager.window_count().await, 1);
    }

    #[tokio::test]
    async fn test_remove_window() {
        let mut manager = WindowManager::new().await.unwrap();
        let window_id = unsafe { WindowId::dummy() };
        let window_info = WindowInfo::new(1, 800, 600);
        
        manager.add_window(window_id, window_info).await.unwrap();
        assert_eq!(manager.window_count().await, 1);
        
        assert!(manager.remove_window(window_id).await.is_ok());
        assert_eq!(manager.window_count().await, 0);
    }

    #[tokio::test]
    async fn test_get_window() {
        let mut manager = WindowManager::new().await.unwrap();
        let window_id = unsafe { WindowId::dummy() };
        let window_info = WindowInfo::new(1, 800, 600);
        
        manager.add_window(window_id, window_info).await.unwrap();
        
        let retrieved_info = manager.get_window(window_id).await.unwrap();
        assert_eq!(retrieved_info.id, 1);
        assert_eq!(retrieved_info.width, 800);
        assert_eq!(retrieved_info.height, 600);
    }
}
