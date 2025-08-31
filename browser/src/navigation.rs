//! Navigation system for the Matte browser.
//! 
//! This module provides URL parsing, navigation state management,
//! and History API implementation.

use crate::error::{Error, Result};
use common::types::{TabId, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{debug, info};

/// Navigation state for a tab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationState {
    /// Current URL
    pub current_url: Url,
    /// Navigation history
    pub history: Vec<HistoryEntry>,
    /// Current history index
    pub current_index: usize,
    /// Navigation timing information
    pub timing: NavigationTiming,
    /// Navigation state object (for History API)
    pub state: Option<serde_json::Value>,
    /// Whether navigation is in progress
    pub is_navigating: bool,
    /// Navigation error, if any
    pub error: Option<NavigationError>,
}

impl NavigationState {
    /// Create a new navigation state
    pub fn new(initial_url: Url) -> Self {
        let entry = HistoryEntry {
            url: initial_url.clone(),
            title: String::new(),
            timestamp: SystemTime::now(),
            state: None,
        };

        Self {
            current_url: initial_url,
            history: vec![entry],
            current_index: 0,
            timing: NavigationTiming::new(),
            state: None,
            is_navigating: false,
            error: None,
        }
    }

    /// Navigate to a new URL
    pub fn navigate(&mut self, url: Url) -> Result<()> {
        info!("Navigating to: {:?}", url);
        
        self.is_navigating = true;
        self.timing.start_navigation();
        self.error = None;

        // Validate URL - check if scheme and host are not empty
        if url.scheme.is_empty() || url.host.is_empty() {
            self.error = Some(NavigationError::InvalidUrl);
            self.is_navigating = false;
            return Err(Error::ParseError("Invalid URL".to_string()));
        }

        // Add to history
        let entry = HistoryEntry {
            url: url.clone(),
            title: String::new(),
            timestamp: SystemTime::now(),
            state: None,
        };

        // Remove any forward history
        self.history.truncate(self.current_index + 1);
        self.history.push(entry);
        self.current_index = self.history.len() - 1;
        self.current_url = url;

        self.timing.finish_navigation();
        self.is_navigating = false;
        
        Ok(())
    }

    /// Navigate back in history
    pub fn go_back(&mut self) -> Result<()> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.current_url = self.history[self.current_index].url.clone();
            self.state = self.history[self.current_index].state.clone();
            info!("Navigated back to: {:?}", self.current_url);
            Ok(())
        } else {
            Err(Error::InvalidState("No back history available".to_string()))
        }
    }

    /// Navigate forward in history
    pub fn go_forward(&mut self) -> Result<()> {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            self.current_url = self.history[self.current_index].url.clone();
            self.state = self.history[self.current_index].state.clone();
            info!("Navigated forward to: {:?}", self.current_url);
            Ok(())
        } else {
            Err(Error::InvalidState("No forward history available".to_string()))
        }
    }

    /// Check if back navigation is available
    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    /// Check if forward navigation is available
    pub fn can_go_forward(&self) -> bool {
        self.current_index < self.history.len() - 1
    }

    /// Get the current history entry
    pub fn current_entry(&self) -> Option<&HistoryEntry> {
        self.history.get(self.current_index)
    }

    /// Update the title of the current entry
    pub fn set_title(&mut self, title: String) {
        if let Some(entry) = self.history.get_mut(self.current_index) {
            entry.title = title;
        }
    }

    /// Get the current title
    pub fn title(&self) -> String {
        self.current_entry()
            .map(|entry| entry.title.clone())
            .unwrap_or_default()
    }
}

/// History entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// URL of the entry
    pub url: Url,
    /// Page title
    pub title: String,
    /// Timestamp when visited
    pub timestamp: SystemTime,
    /// History state object
    pub state: Option<serde_json::Value>,
}

/// Navigation timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationTiming {
    /// Navigation start time
    pub navigation_start: Option<SystemTime>,
    /// Navigation end time
    pub navigation_end: Option<SystemTime>,
    /// DNS lookup start time
    pub dns_start: Option<SystemTime>,
    /// DNS lookup end time
    pub dns_end: Option<SystemTime>,
    /// Connection start time
    pub connect_start: Option<SystemTime>,
    /// Connection end time
    pub connect_end: Option<SystemTime>,
    /// Request start time
    pub request_start: Option<SystemTime>,
    /// Response start time
    pub response_start: Option<SystemTime>,
    /// Response end time
    pub response_end: Option<SystemTime>,
    /// DOM content loaded time
    pub dom_content_loaded: Option<SystemTime>,
    /// Load complete time
    pub load_complete: Option<SystemTime>,
}

impl NavigationTiming {
    /// Create new navigation timing
    pub fn new() -> Self {
        Self {
            navigation_start: None,
            navigation_end: None,
            dns_start: None,
            dns_end: None,
            connect_start: None,
            connect_end: None,
            request_start: None,
            response_start: None,
            response_end: None,
            dom_content_loaded: None,
            load_complete: None,
        }
    }

    /// Start navigation timing
    pub fn start_navigation(&mut self) {
        self.navigation_start = Some(SystemTime::now());
        self.navigation_end = None;
        self.dns_start = None;
        self.dns_end = None;
        self.connect_start = None;
        self.connect_end = None;
        self.request_start = None;
        self.response_start = None;
        self.response_end = None;
        self.dom_content_loaded = None;
        self.load_complete = None;
    }

    /// Finish navigation timing
    pub fn finish_navigation(&mut self) {
        self.navigation_end = Some(SystemTime::now());
    }

    /// Get navigation duration
    pub fn navigation_duration(&self) -> Option<Duration> {
        if let (Some(start), Some(end)) = (self.navigation_start, self.navigation_end) {
            end.duration_since(start).ok()
        } else {
            None
        }
    }

    /// Get DNS lookup duration
    pub fn dns_duration(&self) -> Option<Duration> {
        if let (Some(start), Some(end)) = (self.dns_start, self.dns_end) {
            end.duration_since(start).ok()
        } else {
            None
        }
    }

    /// Get connection duration
    pub fn connect_duration(&self) -> Option<Duration> {
        if let (Some(start), Some(end)) = (self.connect_start, self.connect_end) {
            end.duration_since(start).ok()
        } else {
            None
        }
    }

    /// Get request duration
    pub fn request_duration(&self) -> Option<Duration> {
        if let (Some(start), Some(end)) = (self.request_start, self.response_start) {
            end.duration_since(start).ok()
        } else {
            None
        }
    }

    /// Get response duration
    pub fn response_duration(&self) -> Option<Duration> {
        if let (Some(start), Some(end)) = (self.response_start, self.response_end) {
            end.duration_since(start).ok()
        } else {
            None
        }
    }
}

impl Default for NavigationTiming {
    fn default() -> Self {
        Self::new()
    }
}

/// Navigation error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavigationError {
    /// Invalid URL
    InvalidUrl,
    /// Network error
    NetworkError(String),
    /// DNS resolution failed
    DnsError(String),
    /// SSL/TLS error
    SslError(String),
    /// HTTP error
    HttpError(u16, String),
    /// Timeout
    Timeout,
    /// Access denied
    AccessDenied,
    /// Content blocked
    ContentBlocked,
}

/// History API implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryApi {
    /// Current state object
    pub state: Option<serde_json::Value>,
    /// History length
    pub length: usize,
}

impl HistoryApi {
    /// Create new History API
    pub fn new() -> Self {
        Self {
            state: None,
            length: 1,
        }
    }

    /// Push a new state
    pub fn push_state(&mut self, state: Option<serde_json::Value>, title: String, url: Option<String>) -> Result<()> {
        debug!("pushState: title='{}', url={:?}", title, url);
        
        self.state = state;
        self.length += 1;
        
        Ok(())
    }

    /// Replace current state
    pub fn replace_state(&mut self, state: Option<serde_json::Value>, title: String, url: Option<String>) -> Result<()> {
        debug!("replaceState: title='{}', url={:?}", title, url);
        
        self.state = state;
        
        Ok(())
    }

    /// Get current state
    pub fn state(&self) -> Option<&serde_json::Value> {
        self.state.as_ref()
    }

    /// Get history length
    pub fn length(&self) -> usize {
        self.length
    }
}

impl Default for HistoryApi {
    fn default() -> Self {
        Self::new()
    }
}

/// Navigation manager for handling multiple tabs
pub struct NavigationManager {
    /// Navigation states for each tab
    states: HashMap<TabId, NavigationState>,
    /// History API instances for each tab
    history_apis: HashMap<TabId, HistoryApi>,
}

impl NavigationManager {
    /// Create a new navigation manager
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            history_apis: HashMap::new(),
        }
    }

    /// Create navigation state for a new tab
    pub fn create_tab(&mut self, tab_id: TabId, initial_url: Url) -> Result<()> {
        let navigation_state = NavigationState::new(initial_url);
        let history_api = HistoryApi::new();
        
        self.states.insert(tab_id, navigation_state);
        self.history_apis.insert(tab_id, history_api);
        
        info!("Created navigation state for tab: {}", tab_id);
        Ok(())
    }

    /// Remove navigation state for a tab
    pub fn remove_tab(&mut self, tab_id: &TabId) {
        self.states.remove(tab_id);
        self.history_apis.remove(tab_id);
        
        info!("Removed navigation state for tab: {}", tab_id);
    }

    /// Navigate a tab to a new URL
    pub fn navigate_tab(&mut self, tab_id: &TabId, url: Url) -> Result<()> {
        if let Some(state) = self.states.get_mut(tab_id) {
            state.navigate(url)
        } else {
            Err(Error::InvalidState(format!("No navigation state for tab: {}", tab_id)))
        }
    }

    /// Go back in tab history
    pub fn go_back(&mut self, tab_id: &TabId) -> Result<()> {
        if let Some(state) = self.states.get_mut(tab_id) {
            state.go_back()
        } else {
            Err(Error::InvalidState(format!("No navigation state for tab: {}", tab_id)))
        }
    }

    /// Go forward in tab history
    pub fn go_forward(&mut self, tab_id: &TabId) -> Result<()> {
        if let Some(state) = self.states.get_mut(tab_id) {
            state.go_forward()
        } else {
            Err(Error::InvalidState(format!("No navigation state for tab: {}", tab_id)))
        }
    }

    /// Get navigation state for a tab
    pub fn get_state(&self, tab_id: &TabId) -> Option<&NavigationState> {
        self.states.get(tab_id)
    }

    /// Get mutable navigation state for a tab
    pub fn get_state_mut(&mut self, tab_id: &TabId) -> Option<&mut NavigationState> {
        self.states.get_mut(tab_id)
    }

    /// Get history API for a tab
    pub fn get_history_api(&self, tab_id: &TabId) -> Option<&HistoryApi> {
        self.history_apis.get(tab_id)
    }

    /// Get mutable history API for a tab
    pub fn get_history_api_mut(&mut self, tab_id: &TabId) -> Option<&mut HistoryApi> {
        self.history_apis.get_mut(tab_id)
    }

    /// Check if a tab can go back
    pub fn can_go_back(&self, tab_id: &TabId) -> bool {
        self.states.get(tab_id)
            .map(|state| state.can_go_back())
            .unwrap_or(false)
    }

    /// Check if a tab can go forward
    pub fn can_go_forward(&self, tab_id: &TabId) -> bool {
        self.states.get(tab_id)
            .map(|state| state.can_go_forward())
            .unwrap_or(false)
    }

    /// Get current URL for a tab
    pub fn get_current_url(&self, tab_id: &TabId) -> Option<&Url> {
        self.states.get(tab_id)
            .map(|state| &state.current_url)
    }

    /// Get current title for a tab
    pub fn get_current_title(&self, tab_id: &TabId) -> String {
        self.states.get(tab_id)
            .map(|state| state.title())
            .unwrap_or_default()
    }

    /// Set title for a tab
    pub fn set_title(&mut self, tab_id: &TabId, title: String) -> Result<()> {
        if let Some(state) = self.states.get_mut(tab_id) {
            state.set_title(title);
            Ok(())
        } else {
            Err(Error::InvalidState(format!("No navigation state for tab: {}", tab_id)))
        }
    }
}

impl Default for NavigationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::types::Url;

    #[test]
    fn test_navigation_state_creation() {
        let url = Url::try_from("https://example.com").unwrap();
        let state = NavigationState::new(url.clone());
        
        assert_eq!(state.current_url, url);
        assert_eq!(state.current_index, 0);
        assert_eq!(state.history.len(), 1);
        assert!(!state.is_navigating);
        assert!(state.error.is_none());
    }

    #[test]
    fn test_navigation() {
        let url1 = Url::try_from("https://example.com").unwrap();
        let url2 = Url::try_from("https://example.com/page1").unwrap();
        
        let mut state = NavigationState::new(url1.clone());
        
        // Navigate to new URL
        assert!(state.navigate(url2.clone()).is_ok());
        assert_eq!(state.current_url, url2);
        assert_eq!(state.current_index, 1);
        assert_eq!(state.history.len(), 2);
        assert!(!state.is_navigating);
        
        // Can go back
        assert!(state.can_go_back());
        assert!(!state.can_go_forward());
    }

    #[test]
    fn test_back_forward_navigation() {
        let url1 = Url::try_from("https://example.com").unwrap();
        let url2 = Url::try_from("https://example.com/page1").unwrap();
        let url3 = Url::try_from("https://example.com/page2").unwrap();
        
        let mut state = NavigationState::new(url1.clone());
        state.navigate(url2.clone()).unwrap();
        state.navigate(url3.clone()).unwrap();
        
        // Go back
        assert!(state.go_back().is_ok());
        assert_eq!(state.current_url, url2);
        assert_eq!(state.current_index, 1);
        
        // Go back again
        assert!(state.go_back().is_ok());
        assert_eq!(state.current_url, url1);
        assert_eq!(state.current_index, 0);
        
        // Can't go back further
        assert!(!state.can_go_back());
        assert!(state.go_back().is_err());
        
        // Go forward
        assert!(state.go_forward().is_ok());
        assert_eq!(state.current_url, url2);
        assert_eq!(state.current_index, 1);
        
        // Go forward again
        assert!(state.go_forward().is_ok());
        assert_eq!(state.current_url, url3);
        assert_eq!(state.current_index, 2);
        
        // Can't go forward further
        assert!(!state.can_go_forward());
        assert!(state.go_forward().is_err());
    }

    #[test]
    fn test_navigation_timing() {
        let mut timing = NavigationTiming::new();
        
        // Start navigation
        timing.start_navigation();
        assert!(timing.navigation_start.is_some());
        assert!(timing.navigation_end.is_none());
        
        // Add a small delay to ensure measurable duration
        std::thread::sleep(Duration::from_millis(1));
        
        // Finish navigation
        timing.finish_navigation();
        assert!(timing.navigation_end.is_some());
        
        // Check duration
        let duration = timing.navigation_duration();
        assert!(duration.is_some());
        assert!(duration.unwrap() > Duration::from_nanos(0));
    }

    #[test]
    fn test_history_api() {
        let mut history = HistoryApi::new();
        
        // Initial state
        assert_eq!(history.length(), 1);
        assert!(history.state().is_none());
        
        // Push state
        let state = serde_json::json!({"page": 1});
        assert!(history.push_state(Some(state.clone()), "Page 1".to_string(), None).is_ok());
        assert_eq!(history.length(), 2);
        assert_eq!(history.state(), Some(&state));
        
        // Replace state
        let new_state = serde_json::json!({"page": 2});
        assert!(history.replace_state(Some(new_state.clone()), "Page 2".to_string(), None).is_ok());
        assert_eq!(history.length(), 2); // Length doesn't change for replace
        assert_eq!(history.state(), Some(&new_state));
    }

    #[test]
    fn test_navigation_manager() {
        let mut manager = NavigationManager::new();
        let tab_id = TabId::new(1);
        let url = Url::try_from("https://example.com").unwrap();
        
        // Create tab
        assert!(manager.create_tab(tab_id, url.clone()).is_ok());
        
        // Get state
        let state = manager.get_state(&tab_id);
        assert!(state.is_some());
        assert_eq!(state.unwrap().current_url, url);
        
        // Navigate
        let new_url = Url::try_from("https://example.com/page1").unwrap();
        assert!(manager.navigate_tab(&tab_id, new_url.clone()).is_ok());
        
        // Check navigation
        assert_eq!(manager.get_current_url(&tab_id), Some(&new_url));
        assert!(manager.can_go_back(&tab_id));
        assert!(!manager.can_go_forward(&tab_id));
        
        // Remove tab
        manager.remove_tab(&tab_id);
        assert!(manager.get_state(&tab_id).is_none());
    }

    #[test]
    fn test_title_management() {
        let mut manager = NavigationManager::new();
        let tab_id = TabId::new(1);
        let url = Url::try_from("https://example.com").unwrap();
        
        manager.create_tab(tab_id, url).unwrap();
        
        // Set title
        let title = "Example Page".to_string();
        assert!(manager.set_title(&tab_id, title.clone()).is_ok());
        assert_eq!(manager.get_current_title(&tab_id), title);
    }
}
