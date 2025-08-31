//! Extension host framework for Matte Browser

use common::{error::Result, TabId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Extension manifest structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    /// Extension name
    pub name: String,
    
    /// Extension version
    pub version: String,
    
    /// Extension description
    pub description: Option<String>,
    
    /// Extension author
    pub author: Option<String>,
    
    /// Extension homepage
    pub homepage_url: Option<String>,
    
    /// Extension permissions
    pub permissions: Vec<String>,
    
    /// Content scripts
    pub content_scripts: Option<Vec<ContentScript>>,
    
    /// Background scripts
    pub background_scripts: Option<Vec<String>>,
    
    /// Browser action (toolbar button)
    pub browser_action: Option<BrowserAction>,
    
    /// Page action (address bar button)
    pub page_action: Option<PageAction>,
    
    /// Web accessible resources
    pub web_accessible_resources: Option<Vec<String>>,
    
    /// Minimum browser version required
    pub minimum_browser_version: Option<String>,
    
    /// Extension ID (auto-generated)
    pub id: Option<String>,
}

/// Content script configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentScript {
    /// Script files to inject
    pub js: Vec<String>,
    
    /// CSS files to inject
    pub css: Option<Vec<String>>,
    
    /// URL patterns to match
    pub matches: Vec<String>,
    
    /// Whether to run at document start
    pub run_at: Option<String>, // "document_start", "document_end", "document_idle"
    
    /// Whether to run in all frames
    pub all_frames: Option<bool>,
    
    /// Whether to run in isolated world
    pub world: Option<String>, // "MAIN", "ISOLATED"
}

/// Browser action configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserAction {
    /// Default title
    pub default_title: Option<String>,
    
    /// Default popup
    pub default_popup: Option<String>,
    
    /// Default icon
    pub default_icon: Option<String>,
}

/// Page action configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageAction {
    /// Default title
    pub default_title: Option<String>,
    
    /// Default popup
    pub default_popup: Option<String>,
    
    /// Default icon
    pub default_icon: Option<String>,
}

/// Extension state
#[derive(Debug, Clone)]
pub enum ExtensionState {
    /// Extension is loading
    Loading,
    
    /// Extension is active
    Active,
    
    /// Extension is disabled
    Disabled,
    
    /// Extension has an error
    Error(String),
}

/// Extension instance
pub struct Extension {
    /// Extension manifest
    pub manifest: ExtensionManifest,
    
    /// Extension directory
    pub directory: PathBuf,
    
    /// Extension state
    pub state: ExtensionState,
    
    /// Extension ID
    pub id: String,
    
    /// Background script context
    pub background_context: Option<BackgroundContext>,
    
    /// Content script contexts
    pub content_contexts: HashMap<TabId, Vec<ContentScriptContext>>,
    
    /// Extension storage
    pub storage: ExtensionStorage,
}

/// Background script context
pub struct BackgroundContext {
    /// Background script engine
    pub engine: ExtensionScriptEngine,
    
    /// Message handlers
    pub message_handlers: HashMap<String, Box<dyn Fn(serde_json::Value) + Send + Sync>>,
    
    /// Event listeners
    pub event_listeners: HashMap<String, Vec<Box<dyn Fn(serde_json::Value) + Send + Sync>>>,
}

/// Content script context
#[derive(Debug)]
pub struct ContentScriptContext {
    /// Content script engine
    pub engine: ExtensionScriptEngine,
    
    /// Tab ID
    pub tab_id: TabId,
    
    /// Script files
    pub scripts: Vec<String>,
    
    /// CSS files
    pub css_files: Vec<String>,
}

/// Extension script engine
#[derive(Debug)]
pub struct ExtensionScriptEngine {
    /// Engine type
    pub engine_type: ScriptEngineType,
    
    /// Engine context
    pub context: Box<dyn std::any::Any + Send + Sync>,
}

/// Script engine types
#[derive(Debug, Clone)]
pub enum ScriptEngineType {
    /// JavaScript engine
    JavaScript,
    
    /// WebAssembly engine
    WebAssembly,
}

/// Extension storage
#[derive(Debug, Default)]
pub struct ExtensionStorage {
    /// Local storage
    pub local: HashMap<String, serde_json::Value>,
    
    /// Sync storage
    pub sync: HashMap<String, serde_json::Value>,
    
    /// Session storage
    pub session: HashMap<String, serde_json::Value>,
}

/// Extension host manager
pub struct ExtensionHost {
    /// Loaded extensions
    extensions: HashMap<String, Arc<RwLock<Extension>>>,
    
    /// Extension directories
    extension_directories: Vec<PathBuf>,
    
    /// Extension permissions
    permissions: HashMap<String, Vec<String>>,
    
    /// Extension message router
    message_router: Arc<RwLock<ExtensionMessageRouter>>,
    
    /// Extension storage manager
    storage_manager: Arc<RwLock<ExtensionStorageManager>>,
}

/// Extension message router
pub struct ExtensionMessageRouter {
    /// Message handlers
    handlers: HashMap<String, Box<dyn Fn(String, serde_json::Value) -> Result<serde_json::Value> + Send + Sync>>,
    
    /// Message queue
    message_queue: Vec<ExtensionMessage>,
}

/// Extension message
#[derive(Debug)]
pub struct ExtensionMessage {
    /// Source extension ID
    pub source_id: String,
    
    /// Target extension ID
    pub target_id: Option<String>,
    
    /// Message type
    pub message_type: String,
    
    /// Message data
    pub data: serde_json::Value,
    
    /// Tab ID (if applicable)
    pub tab_id: Option<TabId>,
}

/// Extension storage manager
pub struct ExtensionStorageManager {
    /// Storage backends
    storage_backends: HashMap<String, Box<dyn ExtensionStorageBackend + Send + Sync>>,
}

/// Extension storage backend trait
pub trait ExtensionStorageBackend {
    /// Get a value
    fn get(&self, key: &str) -> Result<Option<serde_json::Value>>;
    
    /// Set a value
    fn set(&self, key: &str, value: serde_json::Value) -> Result<()>;
    
    /// Remove a value
    fn remove(&self, key: &str) -> Result<()>;
    
    /// Clear all values
    fn clear(&self) -> Result<()>;
}

impl ExtensionHost {
    /// Create a new extension host
    pub async fn new() -> Result<Self> {
        info!("Initializing extension host");
        
        let extension_directories = vec![
            Self::get_default_extension_directory().await?,
        ];
        
        let message_router = Arc::new(RwLock::new(ExtensionMessageRouter::new()));
        let storage_manager = Arc::new(RwLock::new(ExtensionStorageManager::new()));
        
        let mut host = Self {
            extensions: HashMap::new(),
            extension_directories,
            permissions: HashMap::new(),
            message_router,
            storage_manager,
        };
        
        // Load existing extensions
        host.load_extensions().await?;
        
        info!("Extension host initialized successfully");
        Ok(host)
    }
    
    /// Get default extension directory
    async fn get_default_extension_directory() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| common::error::Error::ConfigError("Could not find config directory".to_string()))?;
        path.push("matte-browser");
        path.push("extensions");
        
        // Create directory if it doesn't exist
        tokio::fs::create_dir_all(&path).await
            .map_err(|e| common::error::Error::IoError(e.to_string()))?;
        
        Ok(path)
    }
    
    /// Load extensions from directories
    pub async fn load_extensions(&mut self) -> Result<()> {
        info!("Loading extensions from directories");
        
        let directories = self.extension_directories.clone();
        for directory in directories {
            if let Ok(mut entries) = tokio::fs::read_dir(&directory).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Err(e) = self.load_extension(&path).await {
                            warn!("Failed to load extension from {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
        
        info!("Loaded {} extensions", self.extensions.len());
        Ok(())
    }
    
    /// Load a single extension
    pub async fn load_extension(&mut self, directory: &Path) -> Result<()> {
        info!("Loading extension from {:?}", directory);
        
        // Read manifest file
        let manifest_path = directory.join("manifest.json");
        let manifest_content = tokio::fs::read_to_string(&manifest_path).await
            .map_err(|e| common::error::Error::IoError(e.to_string()))?;
        
        let mut manifest: ExtensionManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| common::error::Error::ConfigError(format!("Invalid manifest: {}", e)))?;
        
        // Generate extension ID if not present
        if manifest.id.is_none() {
            manifest.id = Some(self.generate_extension_id(&manifest.name));
        }
        
        let extension_id = manifest.id.as_ref().unwrap().clone();
        
        // Validate extension
        self.validate_extension(&manifest).await?;
        
        // Create extension instance
        let extension = Extension {
            manifest: manifest.clone(),
            directory: directory.to_path_buf(),
            state: ExtensionState::Loading,
            id: extension_id.clone(),
            background_context: None,
            content_contexts: HashMap::new(),
            storage: ExtensionStorage::default(),
        };
        
        // Initialize extension
        let mut extension = Arc::new(RwLock::new(extension));
        self.initialize_extension(&mut extension).await?;
        
        // Store extension
        self.extensions.insert(extension_id.clone(), extension);
        
        info!("Extension '{}' loaded successfully", manifest.name);
        Ok(())
    }
    
    /// Generate extension ID
    fn generate_extension_id(&self, name: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// Validate extension
    async fn validate_extension(&self, manifest: &ExtensionManifest) -> Result<()> {
        // Check required fields
        if manifest.name.is_empty() {
            return Err(common::error::Error::ConfigError("Extension name is required".to_string()));
        }
        
        if manifest.version.is_empty() {
            return Err(common::error::Error::ConfigError("Extension version is required".to_string()));
        }
        
        // Check permissions
        for permission in &manifest.permissions {
            if !self.is_valid_permission(permission) {
                return Err(common::error::Error::ConfigError(
                    format!("Invalid permission: {}", permission)
                ));
            }
        }
        
        // Check minimum browser version
        if let Some(min_version) = &manifest.minimum_browser_version {
            if !self.check_version_compatibility(min_version) {
                return Err(common::error::Error::ConfigError(
                    format!("Extension requires browser version >= {}", min_version)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Check if permission is valid
    fn is_valid_permission(&self, permission: &str) -> bool {
        let valid_permissions = vec![
            "activeTab",
            "alarms",
            "bookmarks",
            "browsingData",
            "cookies",
            "downloads",
            "history",
            "identity",
            "notifications",
            "storage",
            "tabs",
            "webNavigation",
            "webRequest",
            "webRequestBlocking",
        ];
        
        valid_permissions.contains(&permission)
    }
    
    /// Check version compatibility
    fn check_version_compatibility(&self, _required_version: &str) -> bool {
        // Simple version comparison for now
        // TODO: Implement proper semantic versioning
        true
    }
    
    /// Initialize extension
    async fn initialize_extension(&self, extension: &mut Arc<RwLock<Extension>>) -> Result<()> {
        let mut ext = extension.write().await;
        
        // Initialize background scripts
        if let Some(background_scripts) = &ext.manifest.background_scripts {
            ext.background_context = Some(self.create_background_context(background_scripts).await?);
        }
        
        // Load extension storage
        ext.storage = self.load_extension_storage(&ext.id).await?;
        
        // Set state to active
        ext.state = ExtensionState::Active;
        
        Ok(())
    }
    
    /// Create background context
    async fn create_background_context(&self, _scripts: &[String]) -> Result<BackgroundContext> {
        let engine = ExtensionScriptEngine {
            engine_type: ScriptEngineType::JavaScript,
            context: Box::new(()), // Placeholder for actual JS engine
        };
        
        let message_handlers = HashMap::new();
        let event_listeners = HashMap::new();
        
        Ok(BackgroundContext {
            engine,
            message_handlers,
            event_listeners,
        })
    }
    
    /// Load extension storage
    async fn load_extension_storage(&self, _extension_id: &str) -> Result<ExtensionStorage> {
        // TODO: Implement actual storage loading
        Ok(ExtensionStorage::default())
    }
    
    /// Get extension by ID
    pub async fn get_extension(&self, extension_id: &str) -> Option<Arc<RwLock<Extension>>> {
        self.extensions.get(extension_id).cloned()
    }
    
    /// Get all extensions
    pub async fn get_extensions(&self) -> Vec<Arc<RwLock<Extension>>> {
        self.extensions.values().cloned().collect()
    }
    
    /// Enable extension
    pub async fn enable_extension(&mut self, extension_id: &str) -> Result<()> {
        if let Some(extension) = self.extensions.get(extension_id) {
            let mut ext = extension.write().await;
            ext.state = ExtensionState::Active;
            info!("Extension '{}' enabled", ext.manifest.name);
        }
        Ok(())
    }
    
    /// Disable extension
    pub async fn disable_extension(&mut self, extension_id: &str) -> Result<()> {
        if let Some(extension) = self.extensions.get(extension_id) {
            let mut ext = extension.write().await;
            ext.state = ExtensionState::Disabled;
            info!("Extension '{}' disabled", ext.manifest.name);
        }
        Ok(())
    }
    
    /// Unload extension
    pub async fn unload_extension(&mut self, extension_id: &str) -> Result<()> {
        if let Some(extension) = self.extensions.remove(extension_id) {
            let ext = extension.read().await;
            info!("Extension '{}' unloaded", ext.manifest.name);
        }
        Ok(())
    }
    
    /// Inject content scripts for a tab
    pub async fn inject_content_scripts(&mut self, tab_id: TabId, url: &str) -> Result<()> {
        for extension in self.extensions.values() {
            let ext = extension.read().await;
            
            if let ExtensionState::Active = ext.state {
                if let Some(content_scripts) = &ext.manifest.content_scripts {
                    for script in content_scripts {
                        if self.matches_url_pattern(url, &script.matches) {
                            // TODO: Actually inject the scripts
                            debug!("Would inject content script for extension '{}' in tab {}", 
                                   ext.manifest.name, tab_id);
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Check if URL matches patterns
    fn matches_url_pattern(&self, url: &str, patterns: &[String]) -> bool {
        // Simple pattern matching for now
        // TODO: Implement proper URL pattern matching
        patterns.iter().any(|pattern| url.contains(pattern))
    }
    
    /// Send message to extension
    pub async fn send_message(&self, extension_id: &str, message: ExtensionMessage) -> Result<()> {
        let _router = self.message_router.read().await;
        // TODO: Implement message routing
        debug!("Sending message to extension '{}': {:?}", extension_id, message);
        Ok(())
    }
    
    /// Shutdown extension host
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down extension host");
        
        // Save extension storage
        for extension in self.extensions.values() {
            let ext = extension.read().await;
            self.save_extension_storage(&ext.id, &ext.storage).await?;
        }
        
        // Clear extensions
        self.extensions.clear();
        
        info!("Extension host shutdown complete");
        Ok(())
    }
    
    /// Save extension storage
    async fn save_extension_storage(&self, extension_id: &str, _storage: &ExtensionStorage) -> Result<()> {
        // TODO: Implement actual storage saving
        debug!("Saving storage for extension '{}'", extension_id);
        Ok(())
    }
}

impl ExtensionMessageRouter {
    /// Create new message router
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            message_queue: Vec::new(),
        }
    }
    
    /// Register message handler
    pub fn register_handler<F>(&mut self, message_type: String, handler: F)
    where
        F: Fn(String, serde_json::Value) -> Result<serde_json::Value> + Send + Sync + 'static,
    {
        self.handlers.insert(message_type, Box::new(handler));
    }
    
    /// Route message
    pub async fn route_message(&mut self, message: ExtensionMessage) -> Result<serde_json::Value> {
        if let Some(handler) = self.handlers.get(&message.message_type) {
            handler(message.source_id, message.data)
        } else {
            Err(common::error::Error::ConfigError(
                format!("No handler for message type: {}", message.message_type)
            ))
        }
    }
}

impl ExtensionStorageManager {
    /// Create new storage manager
    pub fn new() -> Self {
        Self {
            storage_backends: HashMap::new(),
        }
    }
    
    /// Register storage backend
    pub fn register_backend(&mut self, name: String, backend: Box<dyn ExtensionStorageBackend + Send + Sync>) {
        self.storage_backends.insert(name, backend);
    }
    
    /// Get storage backend
    pub fn get_backend(&self, name: &str) -> Option<&Box<dyn ExtensionStorageBackend + Send + Sync>> {
        self.storage_backends.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_extension_host_creation() {
        let host = ExtensionHost::new().await;
        assert!(host.is_ok());
    }

    #[tokio::test]
    async fn test_extension_validation() {
        let host = ExtensionHost::new().await.unwrap();
        
        let valid_manifest = ExtensionManifest {
            name: "Test Extension".to_string(),
            version: "1.0.0".to_string(),
            description: Some("A test extension".to_string()),
            author: None,
            homepage_url: None,
            permissions: vec!["storage".to_string()],
            content_scripts: None,
            background_scripts: None,
            browser_action: None,
            page_action: None,
            web_accessible_resources: None,
            minimum_browser_version: None,
            id: None,
        };
        
        // This should not panic
        let _ = host.validate_extension(&valid_manifest).await;
    }

    #[tokio::test]
    async fn test_extension_id_generation() {
        let host = ExtensionHost::new().await.unwrap();
        let id1 = host.generate_extension_id("Test Extension");
        let id2 = host.generate_extension_id("Test Extension");
        let id3 = host.generate_extension_id("Different Extension");
        
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[tokio::test]
    async fn test_permission_validation() {
        let host = ExtensionHost::new().await.unwrap();
        
        assert!(host.is_valid_permission("storage"));
        assert!(host.is_valid_permission("tabs"));
        assert!(!host.is_valid_permission("invalid_permission"));
    }
}
