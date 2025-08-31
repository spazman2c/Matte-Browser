//! Common types and data structures for the Matte browser.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Unique identifier for browser tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TabId(pub u64);

impl TabId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl fmt::Display for TabId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for renderer processes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RendererId(pub u64);

impl RendererId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl fmt::Display for RendererId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// URL representation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl Url {
    pub fn new(scheme: String, host: String) -> Self {
        Self {
            scheme,
            host,
            port: None,
            path: "/".to_string(),
            query: None,
            fragment: None,
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn with_query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }

    pub fn with_fragment(mut self, fragment: String) -> Self {
        self.fragment = Some(fragment);
        self
    }

    pub fn origin(&self) -> String {
        if let Some(port) = self.port {
            format!("{}://{}:{}", self.scheme, self.host, port)
        } else {
            format!("{}://{}", self.scheme, self.host)
        }
    }

    pub fn to_string(&self) -> String {
        let mut url = self.origin();
        url.push_str(&self.path);
        if let Some(ref query) = self.query {
            url.push('?');
            url.push_str(query);
        }
        if let Some(ref fragment) = self.fragment {
            url.push('#');
            url.push_str(fragment);
        }
        url
    }
}

impl From<url::Url> for Url {
    fn from(url: url::Url) -> Self {
        Self {
            scheme: url.scheme().to_string(),
            host: url.host_str().unwrap_or("").to_string(),
            port: url.port(),
            path: url.path().to_string(),
            query: url.query().map(|q| q.to_string()),
            fragment: url.fragment().map(|f| f.to_string()),
        }
    }
}

impl TryFrom<&str> for Url {
    type Error = crate::error::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let url = url::Url::parse(s)
            .map_err(|e| crate::error::Error::ParseError(e.to_string()))?;
        Ok(url.into())
    }
}

/// Tab information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabInfo {
    pub id: TabId,
    pub title: String,
    pub url: Url,
    pub favicon: Option<String>,
    pub loading: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub renderer_id: Option<RendererId>,
}

impl TabInfo {
    pub fn new(id: TabId, url: Url) -> Self {
        Self {
            id,
            title: "New Tab".to_string(),
            url,
            favicon: None,
            loading: false,
            can_go_back: false,
            can_go_forward: false,
            renderer_id: None,
        }
    }
}

/// Browser window information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: u64,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub maximized: bool,
    pub fullscreen: bool,
    pub tabs: Vec<TabId>,
    pub active_tab: Option<TabId>,
}

impl WindowInfo {
    pub fn new(id: u64, width: u32, height: u32) -> Self {
        Self {
            id,
            title: "Matte Browser".to_string(),
            width,
            height,
            x: 0,
            y: 0,
            maximized: false,
            fullscreen: false,
            tabs: Vec::new(),
            active_tab: None,
        }
    }
}

/// Browser profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub name: String,
    pub data_directory: std::path::PathBuf,
    pub is_incognito: bool,
    pub settings: HashMap<String, serde_json::Value>,
}

impl ProfileInfo {
    pub fn new(name: String, data_directory: std::path::PathBuf) -> Self {
        Self {
            name,
            data_directory,
            is_incognito: false,
            settings: HashMap::new(),
        }
    }

    pub fn incognito() -> Self {
        Self {
            name: "Incognito".to_string(),
            data_directory: std::path::PathBuf::from("/tmp/matte-incognito"),
            is_incognito: true,
            settings: HashMap::new(),
        }
    }
}

/// Browser settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettings {
    pub homepage: String,
    pub search_engine: String,
    pub enable_javascript: bool,
    pub enable_plugins: bool,
    pub enable_images: bool,
    pub enable_cookies: bool,
    pub enable_popups: bool,
    pub enable_geolocation: bool,
    pub enable_notifications: bool,
    pub enable_autofill: bool,
    pub enable_password_saving: bool,
    pub enable_tracking_protection: bool,
    pub enable_do_not_track: bool,
    pub user_agent: String,
    pub language: String,
    pub timezone: String,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            homepage: "https://www.google.com".to_string(),
            search_engine: "https://www.google.com/search?q={}".to_string(),
            enable_javascript: true,
            enable_plugins: false,
            enable_images: true,
            enable_cookies: true,
            enable_popups: false,
            enable_geolocation: false,
            enable_notifications: false,
            enable_autofill: false,
            enable_password_saving: false,
            enable_tracking_protection: true,
            enable_do_not_track: true,
            user_agent: "Matte/0.1.0".to_string(),
            language: "en-US".to_string(),
            timezone: "UTC".to_string(),
        }
    }
}

/// Browser statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserStats {
    pub total_tabs: u32,
    pub total_windows: u32,
    pub total_renderers: u32,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub cache_size: u64,
    pub uptime: std::time::Duration,
}

impl Default for BrowserStats {
    fn default() -> Self {
        Self {
            total_tabs: 0,
            total_windows: 0,
            total_renderers: 0,
            memory_usage: 0,
            cpu_usage: 0.0,
            network_bytes_sent: 0,
            network_bytes_received: 0,
            cache_size: 0,
            uptime: std::time::Duration::ZERO,
        }
    }
}

/// Browser permissions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    Geolocation,
    Notifications,
    Microphone,
    Camera,
    Clipboard,
    Fullscreen,
    Payment,
    PersistentStorage,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Permission::Geolocation => write!(f, "geolocation"),
            Permission::Notifications => write!(f, "notifications"),
            Permission::Microphone => write!(f, "microphone"),
            Permission::Camera => write!(f, "camera"),
            Permission::Clipboard => write!(f, "clipboard"),
            Permission::Fullscreen => write!(f, "fullscreen"),
            Permission::Payment => write!(f, "payment"),
            Permission::PersistentStorage => write!(f, "persistent-storage"),
        }
    }
}

/// Permission state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionState {
    Granted,
    Denied,
    Prompt,
}

impl fmt::Display for PermissionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermissionState::Granted => write!(f, "granted"),
            PermissionState::Denied => write!(f, "denied"),
            PermissionState::Prompt => write!(f, "prompt"),
        }
    }
}

/// Site permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitePermissions {
    pub origin: String,
    pub permissions: HashMap<Permission, PermissionState>,
    pub last_updated: std::time::SystemTime,
}

impl SitePermissions {
    pub fn new(origin: String) -> Self {
        Self {
            origin,
            permissions: HashMap::new(),
            last_updated: std::time::SystemTime::now(),
        }
    }

    pub fn set_permission(&mut self, permission: Permission, state: PermissionState) {
        self.permissions.insert(permission, state);
        self.last_updated = std::time::SystemTime::now();
    }

    pub fn get_permission(&self, permission: &Permission) -> PermissionState {
        self.permissions.get(permission).cloned().unwrap_or(PermissionState::Prompt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_id() {
        let id = TabId::new(123);
        assert_eq!(id.to_string(), "123");
        assert_eq!(id, TabId(123));
    }

    #[test]
    fn test_url() {
        let url = Url::new("https".to_string(), "example.com".to_string())
            .with_port(8080)
            .with_path("/test".to_string())
            .with_query("param=value".to_string());
        
        assert_eq!(url.origin(), "https://example.com:8080");
        assert_eq!(url.to_string(), "https://example.com:8080/test?param=value");
    }

    #[test]
    fn test_url_from_str() {
        let url = Url::try_from("https://example.com:8080/test?param=value#fragment").unwrap();
        assert_eq!(url.scheme, "https");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, Some(8080));
        assert_eq!(url.path, "/test");
        assert_eq!(url.query, Some("param=value".to_string()));
        assert_eq!(url.fragment, Some("fragment".to_string()));
    }

    #[test]
    fn test_tab_info() {
        let url = Url::new("https".to_string(), "example.com".to_string());
        let tab = TabInfo::new(TabId::new(1), url);
        assert_eq!(tab.title, "New Tab");
        assert!(!tab.loading);
        assert!(!tab.can_go_back);
        assert!(!tab.can_go_forward);
    }

    #[test]
    fn test_browser_settings_default() {
        let settings = BrowserSettings::default();
        assert!(settings.enable_javascript);
        assert!(settings.enable_tracking_protection);
        assert!(settings.enable_do_not_track);
        assert!(!settings.enable_plugins);
        assert!(!settings.enable_popups);
    }

    #[test]
    fn test_permission_display() {
        assert_eq!(Permission::Geolocation.to_string(), "geolocation");
        assert_eq!(Permission::Camera.to_string(), "camera");
    }

    #[test]
    fn test_site_permissions() {
        let mut permissions = SitePermissions::new("https://example.com".to_string());
        permissions.set_permission(Permission::Geolocation, PermissionState::Granted);
        assert_eq!(permissions.get_permission(&Permission::Geolocation), PermissionState::Granted);
        assert_eq!(permissions.get_permission(&Permission::Camera), PermissionState::Prompt);
    }
}
