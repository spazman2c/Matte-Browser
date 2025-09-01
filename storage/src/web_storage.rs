use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// Web Storage manager
pub struct WebStorageManager {
    /// Local storage instances
    local_storage: Arc<RwLock<HashMap<String, LocalStorage>>>,
    /// Session storage instances
    session_storage: Arc<RwLock<HashMap<String, SessionStorage>>>,
    /// Storage quota manager
    quota_manager: Arc<RwLock<StorageQuotaManager>>,
    /// Storage partitioning manager
    partitioning_manager: Arc<RwLock<StoragePartitioningManager>>,
    /// Storage directory
    storage_directory: PathBuf,
}

/// Local storage
pub struct LocalStorage {
    /// Storage origin
    origin: String,
    /// Storage data
    data: HashMap<String, StorageItem>,
    /// Storage file path
    file_path: PathBuf,
    /// Last modified time
    last_modified: u64,
    /// Storage size in bytes
    size: usize,
}

/// Session storage
pub struct SessionStorage {
    /// Storage origin
    origin: String,
    /// Storage data
    data: HashMap<String, StorageItem>,
    /// Session ID
    session_id: String,
    /// Last modified time
    last_modified: u64,
    /// Storage size in bytes
    size: usize,
}

/// Storage item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageItem {
    /// Item key
    pub key: String,
    /// Item value
    pub value: String,
    /// Creation time
    pub created: u64,
    /// Last modified time
    pub modified: u64,
    /// Item size in bytes
    pub size: usize,
}

/// Storage quota manager
pub struct StorageQuotaManager {
    /// Global storage quota
    global_quota: usize,
    /// Per-origin storage quota
    per_origin_quota: usize,
    /// Current global usage
    global_usage: usize,
    /// Per-origin usage
    origin_usage: HashMap<String, usize>,
    /// Quota exceeded origins
    quota_exceeded: HashMap<String, bool>,
}

/// Storage partitioning manager
pub struct StoragePartitioningManager {
    /// Storage partitions
    partitions: HashMap<String, StoragePartition>,
    /// Partition policies
    policies: HashMap<String, PartitionPolicy>,
    /// Default partition
    default_partition: String,
}

/// Storage partition
#[derive(Debug, Clone)]
pub struct StoragePartition {
    /// Partition ID
    pub id: String,
    /// Partition name
    pub name: String,
    /// Partition description
    pub description: String,
    /// Partition quota
    pub quota: usize,
    /// Partition usage
    pub usage: usize,
    /// Partition origins
    pub origins: Vec<String>,
    /// Partition enabled
    pub enabled: bool,
}

/// Partition policy
#[derive(Debug, Clone)]
pub struct PartitionPolicy {
    /// Policy name
    pub name: String,
    /// Policy type
    pub policy_type: PartitionPolicyType,
    /// Policy rules
    pub rules: Vec<PartitionRule>,
    /// Policy enabled
    pub enabled: bool,
}

/// Partition policy type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PartitionPolicyType {
    /// Origin-based partitioning
    OriginBased,
    /// Domain-based partitioning
    DomainBased,
    /// Subdomain-based partitioning
    SubdomainBased,
    /// Custom partitioning
    Custom,
}

/// Partition rule
#[derive(Debug, Clone)]
pub struct PartitionRule {
    /// Rule pattern
    pub pattern: String,
    /// Rule partition
    pub partition: String,
    /// Rule priority
    pub priority: u32,
    /// Rule enabled
    pub enabled: bool,
}

/// Storage event
#[derive(Debug, Clone)]
pub struct StorageEvent {
    /// Event type
    pub event_type: StorageEventType,
    /// Storage key
    pub key: Option<String>,
    /// Old value
    pub old_value: Option<String>,
    /// New value
    pub new_value: Option<String>,
    /// Storage URL
    pub url: String,
    /// Storage origin
    pub origin: String,
    /// Event timestamp
    pub timestamp: u64,
}

/// Storage event type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StorageEventType {
    /// Storage set event
    Set,
    /// Storage remove event
    Remove,
    /// Storage clear event
    Clear,
    /// Storage quota exceeded event
    QuotaExceeded,
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    /// Total storage size
    pub total_size: usize,
    /// Local storage size
    pub local_storage_size: usize,
    /// Session storage size
    pub session_storage_size: usize,
    /// Number of origins
    pub origin_count: usize,
    /// Number of items
    pub item_count: usize,
    /// Quota usage percentage
    pub quota_usage_percentage: f64,
}

impl WebStorageManager {
    /// Create new web storage manager
    pub fn new(storage_directory: PathBuf) -> Result<Self> {
        // Create storage directory if it doesn't exist
        fs::create_dir_all(&storage_directory)
            .map_err(|e| Error::storage(format!("Failed to create storage directory: {}", e)))?;
        
        let quota_manager = Arc::new(RwLock::new(StorageQuotaManager::new()));
        let partitioning_manager = Arc::new(RwLock::new(StoragePartitioningManager::new()));
        
        Ok(Self {
            local_storage: Arc::new(RwLock::new(HashMap::new())),
            session_storage: Arc::new(RwLock::new(HashMap::new())),
            quota_manager,
            partitioning_manager,
            storage_directory,
        })
    }

    /// Get local storage for origin
    pub async fn get_local_storage(&self, origin: &str) -> Result<Arc<RwLock<LocalStorage>>> {
        let mut storage = self.local_storage.write();
        
        if let Some(local_storage) = storage.get(origin) {
            return Ok(local_storage.clone());
        }
        
        // Create new local storage
        let local_storage = Arc::new(RwLock::new(LocalStorage::new(origin, &self.storage_directory)?));
        storage.insert(origin.to_string(), local_storage.clone());
        
        Ok(local_storage)
    }

    /// Get session storage for origin
    pub async fn get_session_storage(&self, origin: &str, session_id: &str) -> Result<Arc<RwLock<SessionStorage>>> {
        let key = format!("{}:{}", origin, session_id);
        let mut storage = self.session_storage.write();
        
        if let Some(session_storage) = storage.get(&key) {
            return Ok(session_storage.clone());
        }
        
        // Create new session storage
        let session_storage = Arc::new(RwLock::new(SessionStorage::new(origin, session_id)?));
        storage.insert(key, session_storage.clone());
        
        Ok(session_storage)
    }

    /// Set local storage item
    pub async fn set_local_storage_item(&self, origin: &str, key: &str, value: &str) -> Result<()> {
        let storage = self.get_local_storage(origin).await?;
        let mut storage_guard = storage.write();
        
        // Check quota
        self.check_quota(origin, key, value).await?;
        
        // Set item
        storage_guard.set_item(key, value)?;
        
        // Update quota usage
        self.update_quota_usage(origin, key, value).await?;
        
        Ok(())
    }

    /// Get local storage item
    pub async fn get_local_storage_item(&self, origin: &str, key: &str) -> Result<Option<String>> {
        let storage = self.get_local_storage(origin).await?;
        let storage_guard = storage.read();
        
        Ok(storage_guard.get_item(key))
    }

    /// Remove local storage item
    pub async fn remove_local_storage_item(&self, origin: &str, key: &str) -> Result<()> {
        let storage = self.get_local_storage(origin).await?;
        let mut storage_guard = storage.write();
        
        storage_guard.remove_item(key)?;
        
        Ok(())
    }

    /// Clear local storage
    pub async fn clear_local_storage(&self, origin: &str) -> Result<()> {
        let storage = self.get_local_storage(origin).await?;
        let mut storage_guard = storage.write();
        
        storage_guard.clear()?;
        
        Ok(())
    }

    /// Set session storage item
    pub async fn set_session_storage_item(&self, origin: &str, session_id: &str, key: &str, value: &str) -> Result<()> {
        let storage = self.get_session_storage(origin, session_id).await?;
        let mut storage_guard = storage.write();
        
        // Check quota
        self.check_quota(origin, key, value).await?;
        
        // Set item
        storage_guard.set_item(key, value)?;
        
        // Update quota usage
        self.update_quota_usage(origin, key, value).await?;
        
        Ok(())
    }

    /// Get session storage item
    pub async fn get_session_storage_item(&self, origin: &str, session_id: &str, key: &str) -> Result<Option<String>> {
        let storage = self.get_session_storage(origin, session_id).await?;
        let storage_guard = storage.read();
        
        Ok(storage_guard.get_item(key))
    }

    /// Remove session storage item
    pub async fn remove_session_storage_item(&self, origin: &str, session_id: &str, key: &str) -> Result<()> {
        let storage = self.get_session_storage(origin, session_id).await?;
        let mut storage_guard = storage.write();
        
        storage_guard.remove_item(key)?;
        
        Ok(())
    }

    /// Clear session storage
    pub async fn clear_session_storage(&self, origin: &str, session_id: &str) -> Result<()> {
        let storage = self.get_session_storage(origin, session_id).await?;
        let mut storage_guard = storage.write();
        
        storage_guard.clear()?;
        
        Ok(())
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> Result<StorageStats> {
        let local_storage = self.local_storage.read();
        let session_storage = self.session_storage.read();
        let quota_manager = self.quota_manager.read();
        
        let mut total_size = 0;
        let mut local_storage_size = 0;
        let mut session_storage_size = 0;
        let mut origin_count = 0;
        let mut item_count = 0;
        
        // Calculate local storage stats
        for storage in local_storage.values() {
            let storage_guard = storage.read();
            local_storage_size += storage_guard.size;
            total_size += storage_guard.size;
            origin_count += 1;
            item_count += storage_guard.data.len();
        }
        
        // Calculate session storage stats
        for storage in session_storage.values() {
            let storage_guard = storage.read();
            session_storage_size += storage_guard.size;
            total_size += storage_guard.size;
            item_count += storage_guard.data.len();
        }
        
        let quota_usage_percentage = if quota_manager.global_quota > 0 {
            (quota_manager.global_usage as f64 / quota_manager.global_quota as f64) * 100.0
        } else {
            0.0
        };
        
        Ok(StorageStats {
            total_size,
            local_storage_size,
            session_storage_size,
            origin_count,
            item_count,
            quota_usage_percentage,
        })
    }

    /// Check storage quota
    async fn check_quota(&self, origin: &str, key: &str, value: &str) -> Result<()> {
        let quota_manager = self.quota_manager.read();
        
        let item_size = key.len() + value.len();
        let current_usage = quota_manager.origin_usage.get(origin).unwrap_or(&0);
        let new_usage = current_usage + item_size;
        
        if new_usage > quota_manager.per_origin_quota {
            return Err(Error::storage("Storage quota exceeded".to_string()));
        }
        
        if quota_manager.global_usage + item_size > quota_manager.global_quota {
            return Err(Error::storage("Global storage quota exceeded".to_string()));
        }
        
        Ok(())
    }

    /// Update quota usage
    async fn update_quota_usage(&self, origin: &str, key: &str, value: &str) -> Result<()> {
        let mut quota_manager = self.quota_manager.write();
        
        let item_size = key.len() + value.len();
        
        // Update origin usage
        let current_usage = quota_manager.origin_usage.get(origin).unwrap_or(&0);
        quota_manager.origin_usage.insert(origin.to_string(), current_usage + item_size);
        
        // Update global usage
        quota_manager.global_usage += item_size;
        
        Ok(())
    }

    /// Get storage partition for origin
    pub async fn get_storage_partition(&self, origin: &str) -> Result<String> {
        let partitioning_manager = self.partitioning_manager.read();
        
        // Apply partitioning policies
        for policy in partitioning_manager.policies.values() {
            if !policy.enabled {
                continue;
            }
            
            for rule in &policy.rules {
                if !rule.enabled {
                    continue;
                }
                
                if self.matches_pattern(origin, &rule.pattern) {
                    return Ok(rule.partition.clone());
                }
            }
        }
        
        // Return default partition
        Ok(partitioning_manager.default_partition.clone())
    }

    /// Check if origin matches pattern
    fn matches_pattern(&self, origin: &str, pattern: &str) -> bool {
        // Simple pattern matching - can be enhanced with regex
        if pattern == "*" {
            return true;
        }
        
        if pattern.starts_with("*.") {
            let domain = &pattern[2..];
            return origin.ends_with(domain);
        }
        
        if pattern.ends_with(".*") {
            let domain = &pattern[..pattern.len() - 2];
            return origin.starts_with(domain);
        }
        
        origin == pattern
    }

    /// Clean up expired session storage
    pub async fn cleanup_expired_sessions(&self) -> Result<()> {
        let mut session_storage = self.session_storage.write();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let expired_sessions: Vec<String> = session_storage
            .iter()
            .filter_map(|(key, storage)| {
                let storage_guard = storage.read();
                if current_time - storage_guard.last_modified > 24 * 60 * 60 { // 24 hours
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect();
        
        for key in expired_sessions {
            session_storage.remove(&key);
        }
        
        Ok(())
    }
}

impl LocalStorage {
    /// Create new local storage
    pub fn new(origin: &str, storage_directory: &Path) -> Result<Self> {
        let file_path = storage_directory.join(format!("local_storage_{}.json", origin.replace("://", "_")));
        
        let data = if file_path.exists() {
            Self::load_from_file(&file_path)?
        } else {
            HashMap::new()
        };
        
        let size = Self::calculate_size(&data);
        let last_modified = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(Self {
            origin: origin.to_string(),
            data,
            file_path,
            last_modified,
            size,
        })
    }

    /// Set item
    pub fn set_item(&mut self, key: &str, value: &str) -> Result<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let item = StorageItem {
            key: key.to_string(),
            value: value.to_string(),
            created: current_time,
            modified: current_time,
            size: key.len() + value.len(),
        };
        
        // Update size
        if let Some(existing_item) = self.data.get(key) {
            self.size -= existing_item.size;
        }
        
        self.data.insert(key.to_string(), item);
        self.size += key.len() + value.len();
        self.last_modified = current_time;
        
        // Save to file
        self.save_to_file()?;
        
        Ok(())
    }

    /// Get item
    pub fn get_item(&self, key: &str) -> Option<String> {
        self.data.get(key).map(|item| item.value.clone())
    }

    /// Remove item
    pub fn remove_item(&mut self, key: &str) -> Result<()> {
        if let Some(item) = self.data.remove(key) {
            self.size -= item.size;
            self.last_modified = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            // Save to file
            self.save_to_file()?;
        }
        
        Ok(())
    }

    /// Clear all items
    pub fn clear(&mut self) -> Result<()> {
        self.data.clear();
        self.size = 0;
        self.last_modified = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Save to file
        self.save_to_file()?;
        
        Ok(())
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    /// Get length
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Load from file
    fn load_from_file(file_path: &Path) -> Result<HashMap<String, StorageItem>> {
        if !file_path.exists() {
            return Ok(HashMap::new());
        }
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| Error::storage(format!("Failed to read storage file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| Error::storage(format!("Failed to parse storage file: {}", e)))
    }

    /// Save to file
    fn save_to_file(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.data)
            .map_err(|e| Error::storage(format!("Failed to serialize storage data: {}", e)))?;
        
        fs::write(&self.file_path, content)
            .map_err(|e| Error::storage(format!("Failed to write storage file: {}", e)))
    }

    /// Calculate size
    fn calculate_size(data: &HashMap<String, StorageItem>) -> usize {
        data.values().map(|item| item.size).sum()
    }
}

impl SessionStorage {
    /// Create new session storage
    pub fn new(origin: &str, session_id: &str) -> Result<Self> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(Self {
            origin: origin.to_string(),
            data: HashMap::new(),
            session_id: session_id.to_string(),
            last_modified: current_time,
            size: 0,
        })
    }

    /// Set item
    pub fn set_item(&mut self, key: &str, value: &str) -> Result<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let item = StorageItem {
            key: key.to_string(),
            value: value.to_string(),
            created: current_time,
            modified: current_time,
            size: key.len() + value.len(),
        };
        
        // Update size
        if let Some(existing_item) = self.data.get(key) {
            self.size -= existing_item.size;
        }
        
        self.data.insert(key.to_string(), item);
        self.size += key.len() + value.len();
        self.last_modified = current_time;
        
        Ok(())
    }

    /// Get item
    pub fn get_item(&self, key: &str) -> Option<String> {
        self.data.get(key).map(|item| item.value.clone())
    }

    /// Remove item
    pub fn remove_item(&mut self, key: &str) -> Result<()> {
        if let Some(item) = self.data.remove(key) {
            self.size -= item.size;
            self.last_modified = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
        
        Ok(())
    }

    /// Clear all items
    pub fn clear(&mut self) -> Result<()> {
        self.data.clear();
        self.size = 0;
        self.last_modified = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    /// Get length
    pub fn length(&self) -> usize {
        self.data.len()
    }
}

impl StorageQuotaManager {
    /// Create new storage quota manager
    pub fn new() -> Self {
        Self {
            global_quota: 100 * 1024 * 1024, // 100 MB
            per_origin_quota: 10 * 1024 * 1024, // 10 MB
            global_usage: 0,
            origin_usage: HashMap::new(),
            quota_exceeded: HashMap::new(),
        }
    }

    /// Set global quota
    pub fn set_global_quota(&mut self, quota: usize) {
        self.global_quota = quota;
    }

    /// Set per-origin quota
    pub fn set_per_origin_quota(&mut self, quota: usize) {
        self.per_origin_quota = quota;
    }

    /// Get global quota
    pub fn global_quota(&self) -> usize {
        self.global_quota
    }

    /// Get per-origin quota
    pub fn per_origin_quota(&self) -> usize {
        self.per_origin_quota
    }

    /// Get global usage
    pub fn global_usage(&self) -> usize {
        self.global_usage
    }

    /// Get origin usage
    pub fn origin_usage(&self, origin: &str) -> usize {
        self.origin_usage.get(origin).unwrap_or(&0).clone()
    }

    /// Check if quota exceeded
    pub fn is_quota_exceeded(&self, origin: &str) -> bool {
        self.quota_exceeded.get(origin).unwrap_or(&false).clone()
    }
}

impl StoragePartitioningManager {
    /// Create new storage partitioning manager
    pub fn new() -> Self {
        let mut partitions = HashMap::new();
        
        // Add default partition
        partitions.insert("default".to_string(), StoragePartition {
            id: "default".to_string(),
            name: "Default Partition".to_string(),
            description: "Default storage partition".to_string(),
            quota: 100 * 1024 * 1024, // 100 MB
            usage: 0,
            origins: Vec::new(),
            enabled: true,
        });
        
        Self {
            partitions,
            policies: HashMap::new(),
            default_partition: "default".to_string(),
        }
    }

    /// Add partition
    pub fn add_partition(&mut self, partition: StoragePartition) {
        self.partitions.insert(partition.id.clone(), partition);
    }

    /// Remove partition
    pub fn remove_partition(&mut self, partition_id: &str) {
        self.partitions.remove(partition_id);
    }

    /// Get partition
    pub fn get_partition(&self, partition_id: &str) -> Option<&StoragePartition> {
        self.partitions.get(partition_id)
    }

    /// Add policy
    pub fn add_policy(&mut self, policy: PartitionPolicy) {
        self.policies.insert(policy.name.clone(), policy);
    }

    /// Remove policy
    pub fn remove_policy(&mut self, policy_name: &str) {
        self.policies.remove(policy_name);
    }

    /// Get policy
    pub fn get_policy(&self, policy_name: &str) -> Option<&PartitionPolicy> {
        self.policies.get(policy_name)
    }
}
