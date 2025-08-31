//! Tab manager for the Matte browser

use common::{error::Result, TabId, TabInfo, Url};
use tracing::{debug, error, info, warn};
use std::collections::HashMap;

/// Tab manager for handling browser tabs
pub struct TabManager {
    /// Map of tab ID to tab info
    tabs: HashMap<TabId, TabInfo>,
    
    /// Next tab ID
    next_tab_id: u64,
}

impl TabManager {
    /// Create a new tab manager
    pub async fn new() -> Result<Self> {
        info!("Initializing tab manager");
        
        Ok(Self {
            tabs: HashMap::new(),
            next_tab_id: 1,
        })
    }
    
    /// Create a new tab
    pub async fn create_tab(&mut self, window_id: u64, url: Option<String>) -> Result<TabId> {
        let tab_id = TabId::new(self.next_tab_id);
        self.next_tab_id += 1;
        
        let url = if let Some(url_str) = url {
            Url::try_from(url_str.as_str())?
        } else {
            Url::new("https".to_string(), "www.google.com".to_string())
        };
        
        let tab_info = TabInfo::new(tab_id, url);
        
        info!("Creating tab {} in window {}", tab_id, window_id);
        self.tabs.insert(tab_id, tab_info);
        
        info!("Created tab {} successfully", tab_id);
        Ok(tab_id)
    }
    
    /// Close a tab
    pub async fn close_tab(&mut self, tab_id: TabId) -> Result<()> {
        info!("Closing tab {}", tab_id);
        
        if let Some(tab_info) = self.tabs.remove(&tab_id) {
            info!("Closed tab {} successfully", tab_info.id);
            Ok(())
        } else {
            Err(common::error::Error::NotFound(
                format!("Tab with ID {} not found", tab_id)
            ))
        }
    }
    
    /// Get tab info
    pub async fn get_tab(&self, tab_id: TabId) -> Result<&TabInfo> {
        self.tabs.get(&tab_id)
            .ok_or_else(|| common::error::Error::NotFound(
                format!("Tab with ID {} not found", tab_id)
            ))
    }
    
    /// Get tab info mutably
    pub async fn get_tab_mut(&mut self, tab_id: TabId) -> Result<&mut TabInfo> {
        self.tabs.get_mut(&tab_id)
            .ok_or_else(|| common::error::Error::NotFound(
                format!("Tab with ID {} not found", tab_id)
            ))
    }
    
    /// Navigate a tab to a URL
    pub async fn navigate_tab(&mut self, tab_id: TabId, url: String) -> Result<()> {
        info!("Navigating tab {} to {}", tab_id, url);
        
        let parsed_url = Url::try_from(url.as_str())?;
        
        if let Some(tab_info) = self.tabs.get_mut(&tab_id) {
            tab_info.url = parsed_url;
            tab_info.loading = true;
            info!("Navigated tab {} successfully", tab_id);
            Ok(())
        } else {
            Err(common::error::Error::NotFound(
                format!("Tab with ID {} not found", tab_id)
            ))
        }
    }
    
    /// Update tab loading state
    pub async fn set_tab_loading(&mut self, tab_id: TabId, loading: bool) -> Result<()> {
        if let Some(tab_info) = self.tabs.get_mut(&tab_id) {
            tab_info.loading = loading;
            debug!("Set tab {} loading state to {}", tab_id, loading);
            Ok(())
        } else {
            Err(common::error::Error::NotFound(
                format!("Tab with ID {} not found", tab_id)
            ))
        }
    }
    
    /// Update tab title
    pub async fn set_tab_title(&mut self, tab_id: TabId, title: String) -> Result<()> {
        if let Some(tab_info) = self.tabs.get_mut(&tab_id) {
            tab_info.title = title;
            debug!("Set tab {} title", tab_id);
            Ok(())
        } else {
            Err(common::error::Error::NotFound(
                format!("Tab with ID {} not found", tab_id)
            ))
        }
    }
    
    /// Update tab favicon
    pub async fn set_tab_favicon(&mut self, tab_id: TabId, favicon: String) -> Result<()> {
        if let Some(tab_info) = self.tabs.get_mut(&tab_id) {
            tab_info.favicon = Some(favicon);
            debug!("Set tab {} favicon", tab_id);
            Ok(())
        } else {
            Err(common::error::Error::NotFound(
                format!("Tab with ID {} not found", tab_id)
            ))
        }
    }
    
    /// Get all tabs
    pub async fn get_all_tabs(&self) -> Vec<&TabInfo> {
        self.tabs.values().collect()
    }
    
    /// Get tab count
    pub async fn tab_count(&self) -> usize {
        self.tabs.len()
    }
    
    /// Check if tab exists
    pub async fn has_tab(&self, tab_id: TabId) -> bool {
        self.tabs.contains_key(&tab_id)
    }
    
    /// Get next tab ID
    pub async fn next_tab_id(&self) -> u64 {
        self.next_tab_id
    }
    
    /// Shutdown the tab manager
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down tab manager");
        
        let tab_count = self.tabs.len();
        self.tabs.clear();
        
        info!("Tab manager shutdown complete (closed {} tabs)", tab_count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tab_manager_creation() {
        let manager = TabManager::new().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_create_tab() {
        let mut manager = TabManager::new().await.unwrap();
        
        let tab_id = manager.create_tab(1, None).await.unwrap();
        assert_eq!(manager.tab_count().await, 1);
        assert!(manager.has_tab(tab_id).await);
    }

    #[tokio::test]
    async fn test_create_tab_with_url() {
        let mut manager = TabManager::new().await.unwrap();
        
        let tab_id = manager.create_tab(1, Some("https://example.com".to_string())).await.unwrap();
        assert_eq!(manager.tab_count().await, 1);
        
        let tab_info = manager.get_tab(tab_id).await.unwrap();
        assert_eq!(tab_info.url.host, "example.com");
    }

    #[tokio::test]
    async fn test_close_tab() {
        let mut manager = TabManager::new().await.unwrap();
        
        let tab_id = manager.create_tab(1, None).await.unwrap();
        assert_eq!(manager.tab_count().await, 1);
        
        assert!(manager.close_tab(tab_id).await.is_ok());
        assert_eq!(manager.tab_count().await, 0);
        assert!(!manager.has_tab(tab_id).await);
    }

    #[tokio::test]
    async fn test_navigate_tab() {
        let mut manager = TabManager::new().await.unwrap();
        
        let tab_id = manager.create_tab(1, None).await.unwrap();
        assert!(manager.navigate_tab(tab_id, "https://example.com".to_string()).await.is_ok());
        
        let tab_info = manager.get_tab(tab_id).await.unwrap();
        assert_eq!(tab_info.url.host, "example.com");
        assert!(tab_info.loading);
    }

    #[tokio::test]
    async fn test_set_tab_title() {
        let mut manager = TabManager::new().await.unwrap();
        
        let tab_id = manager.create_tab(1, None).await.unwrap();
        assert!(manager.set_tab_title(tab_id, "New Title".to_string()).await.is_ok());
        
        let tab_info = manager.get_tab(tab_id).await.unwrap();
        assert_eq!(tab_info.title, "New Title");
    }
}
