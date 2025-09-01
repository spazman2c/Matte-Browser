use crate::error::{Error, Result};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use parking_lot::RwLock;
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write, BufReader, BufWriter};
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};

/// Cache entry status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheStatus {
    Fresh,
    Stale,
    Expired,
    Invalid,
}

/// Cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Entry key (URL)
    pub key: String,
    /// Response data
    pub data: Vec<u8>,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// HTTP status code
    pub status_code: u16,
    /// Creation time
    pub created: SystemTime,
    /// Last modified time
    pub last_modified: Option<SystemTime>,
    /// Expiration time
    pub expires: Option<SystemTime>,
    /// ETag
    pub etag: Option<String>,
    /// Cache control directives
    pub cache_control: CacheControl,
    /// Content type
    pub content_type: String,
    /// Content length
    pub content_length: usize,
    /// Compression type
    pub compression: Option<String>,
    /// Access count
    pub access_count: u64,
    /// Last access time
    pub last_accessed: SystemTime,
    /// Size in bytes
    pub size: usize,
}

/// Cache control directives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheControl {
    /// Max age in seconds
    pub max_age: Option<Duration>,
    /// Shared max age in seconds
    pub s_maxage: Option<Duration>,
    /// No cache flag
    pub no_cache: bool,
    /// No store flag
    pub no_store: bool,
    /// Must revalidate flag
    pub must_revalidate: bool,
    /// Proxy revalidate flag
    pub proxy_revalidate: bool,
    /// Public flag
    pub public: bool,
    /// Private flag
    pub private: bool,
    /// Immutable flag
    pub immutable: bool,
    /// Stale while revalidate in seconds
    pub stale_while_revalidate: Option<Duration>,
    /// Stale if error in seconds
    pub stale_if_error: Option<Duration>,
}

/// Cache partition
#[derive(Debug, Clone)]
pub struct CachePartition {
    /// Partition name (domain)
    pub name: String,
    /// Maximum size in bytes
    pub max_size: usize,
    /// Current size in bytes
    pub current_size: usize,
    /// Maximum entries
    pub max_entries: usize,
    /// Current entries
    pub current_entries: usize,
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
    /// Entries
    pub entries: HashMap<String, CacheEntry>,
    /// Access order (for LRU)
    pub access_order: VecDeque<String>,
}

/// Eviction policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    LRU,    // Least Recently Used
    LFU,    // Least Frequently Used
    FIFO,   // First In, First Out
    TTL,    // Time To Live
    Size,   // Size-based
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total hits
    pub hits: u64,
    /// Total misses
    pub misses: u64,
    /// Hit ratio
    pub hit_ratio: f64,
    /// Total size in bytes
    pub total_size: usize,
    /// Total entries
    pub total_entries: usize,
    /// Evictions
    pub evictions: u64,
    /// Expirations
    pub expirations: u64,
    /// Last reset time
    pub last_reset: SystemTime,
}

/// Cache warming entry
#[derive(Debug, Clone)]
pub struct CacheWarmingEntry {
    /// URL to warm
    pub url: String,
    /// Priority (higher = more important)
    pub priority: u8,
    /// Expected content type
    pub expected_content_type: Option<String>,
    /// Time to live
    pub ttl: Option<Duration>,
    /// Created time
    pub created: SystemTime,
}

/// Cache analytics
#[derive(Debug, Clone)]
pub struct CacheAnalytics {
    /// Hit rate by domain
    pub hit_rate_by_domain: HashMap<String, f64>,
    /// Hit rate by content type
    pub hit_rate_by_content_type: HashMap<String, f64>,
    /// Average response time
    pub avg_response_time: Duration,
    /// Cache efficiency
    pub cache_efficiency: f64,
    /// Storage utilization
    pub storage_utilization: f64,
    /// Eviction rate
    pub eviction_rate: f64,
    /// Warming success rate
    pub warming_success_rate: f64,
}

/// Memory cache
pub struct MemoryCache {
    /// Cache partitions
    partitions: Arc<RwLock<HashMap<String, CachePartition>>>,
    /// Global statistics
    stats: Arc<RwLock<CacheStats>>,
    /// Warming queue
    warming_queue: Arc<RwLock<VecDeque<CacheWarmingEntry>>>,
    /// Analytics
    analytics: Arc<RwLock<CacheAnalytics>>,
    /// Configuration
    config: CacheConfig,
}

/// Disk cache
pub struct DiskCache {
    /// Cache directory
    cache_dir: PathBuf,
    /// Index file
    index_file: PathBuf,
    /// Cache index
    index: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Statistics
    stats: Arc<RwLock<CacheStats>>,
    /// Configuration
    config: CacheConfig,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum total size in bytes
    pub max_total_size: usize,
    /// Maximum memory size in bytes
    pub max_memory_size: usize,
    /// Maximum disk size in bytes
    pub max_disk_size: usize,
    /// Default TTL
    pub default_ttl: Duration,
    /// Enable memory cache
    pub enable_memory_cache: bool,
    /// Enable disk cache
    pub enable_disk_cache: bool,
    /// Enable cache warming
    pub enable_cache_warming: bool,
    /// Enable analytics
    pub enable_analytics: bool,
    /// Compression threshold
    pub compression_threshold: usize,
    /// Eviction policy
    pub default_eviction_policy: EvictionPolicy,
    /// Partition by domain
    pub partition_by_domain: bool,
    /// Cache directory
    pub cache_directory: Option<PathBuf>,
}

/// Cache manager
pub struct CacheManager {
    /// Memory cache
    memory_cache: Option<MemoryCache>,
    /// Disk cache
    disk_cache: Option<DiskCache>,
    /// Configuration
    config: CacheConfig,
    /// Statistics
    stats: Arc<RwLock<CacheStats>>,
    /// Warming manager
    warming_manager: Option<CacheWarmingManager>,
}

/// Cache warming manager
pub struct CacheWarmingManager {
    /// Warming queue
    queue: Arc<RwLock<VecDeque<CacheWarmingEntry>>>,
    /// Warming workers
    workers: Vec<tokio::task::JoinHandle<()>>,
    /// Configuration
    config: CacheConfig,
}

impl CacheEntry {
    /// Create new cache entry
    pub fn new(key: String, data: Vec<u8>, headers: HashMap<String, String>, status_code: u16) -> Self {
        let content_length = data.len();
        let now = SystemTime::now();
        
        Self {
            key,
            data,
            headers,
            status_code,
            created: now,
            last_modified: None,
            expires: None,
            etag: None,
            cache_control: CacheControl::default(),
            content_type: "application/octet-stream".to_string(),
            content_length,
            compression: None,
            access_count: 0,
            last_accessed: now,
            size: content_length,
        }
    }

    /// Check if entry is fresh
    pub fn is_fresh(&self) -> bool {
        if let Some(expires) = self.expires {
            SystemTime::now() < expires
        } else {
            true
        }
    }

    /// Check if entry is stale
    pub fn is_stale(&self) -> bool {
        if let Some(expires) = self.expires {
            SystemTime::now() >= expires
        } else {
            false
        }
    }

    /// Check if entry is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            SystemTime::now() > expires
        } else {
            false
        }
    }

    /// Get cache status
    pub fn status(&self) -> CacheStatus {
        if self.is_expired() {
            CacheStatus::Expired
        } else if self.is_stale() {
            CacheStatus::Stale
        } else if self.is_fresh() {
            CacheStatus::Fresh
        } else {
            CacheStatus::Invalid
        }
    }

    /// Update access information
    pub fn update_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = SystemTime::now();
    }

    /// Calculate age
    pub fn age(&self) -> Duration {
        self.created.elapsed().unwrap_or(Duration::ZERO)
    }

    /// Calculate time to live
    pub fn ttl(&self) -> Option<Duration> {
        self.expires.map(|expires| {
            expires.duration_since(SystemTime::now()).unwrap_or(Duration::ZERO)
        })
    }
}

impl CacheControl {
    /// Create default cache control
    pub fn default() -> Self {
        Self {
            max_age: None,
            s_maxage: None,
            no_cache: false,
            no_store: false,
            must_revalidate: false,
            proxy_revalidate: false,
            public: false,
            private: false,
            immutable: false,
            stale_while_revalidate: None,
            stale_if_error: None,
        }
    }

    /// Parse from header string
    pub fn from_header(header: &str) -> Self {
        let mut cache_control = CacheControl::default();
        let directives: Vec<&str> = header.split(',').map(|s| s.trim()).collect();
        
        for directive in directives {
            let parts: Vec<&str> = directive.split('=').collect();
            match parts[0].to_lowercase().as_str() {
                "max-age" => {
                    if parts.len() > 1 {
                        if let Ok(seconds) = parts[1].parse::<u64>() {
                            cache_control.max_age = Some(Duration::from_secs(seconds));
                        }
                    }
                }
                "s-maxage" => {
                    if parts.len() > 1 {
                        if let Ok(seconds) = parts[1].parse::<u64>() {
                            cache_control.s_maxage = Some(Duration::from_secs(seconds));
                        }
                    }
                }
                "no-cache" => cache_control.no_cache = true,
                "no-store" => cache_control.no_store = true,
                "must-revalidate" => cache_control.must_revalidate = true,
                "proxy-revalidate" => cache_control.proxy_revalidate = true,
                "public" => cache_control.public = true,
                "private" => cache_control.private = true,
                "immutable" => cache_control.immutable = true,
                "stale-while-revalidate" => {
                    if parts.len() > 1 {
                        if let Ok(seconds) = parts[1].parse::<u64>() {
                            cache_control.stale_while_revalidate = Some(Duration::from_secs(seconds));
                        }
                    }
                }
                "stale-if-error" => {
                    if parts.len() > 1 {
                        if let Ok(seconds) = parts[1].parse::<u64>() {
                            cache_control.stale_if_error = Some(Duration::from_secs(seconds));
                        }
                    }
                }
                _ => {}
            }
        }
        
        cache_control
    }

    /// Check if entry is cacheable
    pub fn is_cacheable(&self) -> bool {
        !self.no_store && !self.private
    }

    /// Check if entry requires revalidation
    pub fn requires_revalidation(&self) -> bool {
        self.no_cache || self.must_revalidate || self.proxy_revalidate
    }
}

impl CachePartition {
    /// Create new cache partition
    pub fn new(name: String, max_size: usize, max_entries: usize, eviction_policy: EvictionPolicy) -> Self {
        Self {
            name,
            max_size,
            current_size: 0,
            max_entries,
            current_entries: 0,
            eviction_policy,
            entries: HashMap::new(),
            access_order: VecDeque::new(),
        }
    }

    /// Add entry to partition
    pub fn add_entry(&mut self, key: String, entry: CacheEntry) -> Option<CacheEntry> {
        let entry_size = entry.size;
        
        // Check if we need to evict entries
        while (self.current_size + entry_size > self.max_size || self.current_entries >= self.max_entries) 
               && !self.entries.is_empty() {
            self.evict_entry();
        }
        
        // Add entry
        let old_entry = self.entries.insert(key.clone(), entry);
        if let Some(ref old_entry) = old_entry {
            self.current_size -= old_entry.size;
        } else {
            self.current_entries += 1;
        }
        
        self.current_size += entry_size;
        self.access_order.push_back(key);
        
        old_entry
    }

    /// Get entry from partition
    pub fn get_entry(&mut self, key: &str) -> Option<&mut CacheEntry> {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.update_access();
            
            // Update access order for LRU
            if self.eviction_policy == EvictionPolicy::LRU {
                if let Some(pos) = self.access_order.iter().position(|k| k == key) {
                    self.access_order.remove(pos);
                }
                self.access_order.push_back(key.to_string());
            }
            
            Some(entry)
        } else {
            None
        }
    }

    /// Remove entry from partition
    pub fn remove_entry(&mut self, key: &str) -> Option<CacheEntry> {
        if let Some(entry) = self.entries.remove(key) {
            self.current_size -= entry.size;
            self.current_entries -= 1;
            
            // Remove from access order
            if let Some(pos) = self.access_order.iter().position(|k| k == key) {
                self.access_order.remove(pos);
            }
            
            Some(entry)
        } else {
            None
        }
    }

    /// Evict entry based on policy
    fn evict_entry(&mut self) {
        match self.eviction_policy {
            EvictionPolicy::LRU => {
                if let Some(key) = self.access_order.pop_front() {
                    if let Some(entry) = self.entries.remove(&key) {
                        self.current_size -= entry.size;
                        self.current_entries -= 1;
                    }
                }
            }
            EvictionPolicy::LFU => {
                let mut least_frequent = None;
                let mut min_count = u64::MAX;
                
                for (key, entry) in &self.entries {
                    if entry.access_count < min_count {
                        min_count = entry.access_count;
                        least_frequent = Some(key.clone());
                    }
                }
                
                if let Some(key) = least_frequent {
                    if let Some(entry) = self.entries.remove(&key) {
                        self.current_size -= entry.size;
                        self.current_entries -= 1;
                        
                        if let Some(pos) = self.access_order.iter().position(|k| k == &key) {
                            self.access_order.remove(pos);
                        }
                    }
                }
            }
            EvictionPolicy::FIFO => {
                if let Some(key) = self.access_order.pop_front() {
                    if let Some(entry) = self.entries.remove(&key) {
                        self.current_size -= entry.size;
                        self.current_entries -= 1;
                    }
                }
            }
            EvictionPolicy::TTL => {
                let mut oldest = None;
                let mut oldest_time = SystemTime::now();
                
                for (key, entry) in &self.entries {
                    if entry.created < oldest_time {
                        oldest_time = entry.created;
                        oldest = Some(key.clone());
                    }
                }
                
                if let Some(key) = oldest {
                    if let Some(entry) = self.entries.remove(&key) {
                        self.current_size -= entry.size;
                        self.current_entries -= 1;
                        
                        if let Some(pos) = self.access_order.iter().position(|k| k == &key) {
                            self.access_order.remove(pos);
                        }
                    }
                }
            }
            EvictionPolicy::Size => {
                let mut largest = None;
                let mut max_size = 0;
                
                for (key, entry) in &self.entries {
                    if entry.size > max_size {
                        max_size = entry.size;
                        largest = Some(key.clone());
                    }
                }
                
                if let Some(key) = largest {
                    if let Some(entry) = self.entries.remove(&key) {
                        self.current_size -= entry.size;
                        self.current_entries -= 1;
                        
                        if let Some(pos) = self.access_order.iter().position(|k| k == &key) {
                            self.access_order.remove(pos);
                        }
                    }
                }
            }
        }
    }

    /// Clean expired entries
    pub fn clean_expired(&mut self) -> usize {
        let mut expired_keys = Vec::new();
        
        for (key, entry) in &self.entries {
            if entry.is_expired() {
                expired_keys.push(key.clone());
            }
        }
        
        let expired_count = expired_keys.len();
        
        for key in expired_keys {
            if let Some(entry) = self.entries.remove(&key) {
                self.current_size -= entry.size;
                self.current_entries -= 1;
                
                if let Some(pos) = self.access_order.iter().position(|k| k == &key) {
                    self.access_order.remove(pos);
                }
            }
        }
        
        expired_count
    }
}

impl CacheStats {
    /// Create new cache statistics
    pub fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
            hit_ratio: 0.0,
            total_size: 0,
            total_entries: 0,
            evictions: 0,
            expirations: 0,
            last_reset: SystemTime::now(),
        }
    }

    /// Record hit
    pub fn record_hit(&mut self) {
        self.hits += 1;
        self.update_hit_ratio();
    }

    /// Record miss
    pub fn record_miss(&mut self) {
        self.misses += 1;
        self.update_hit_ratio();
    }

    /// Record eviction
    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }

    /// Record expiration
    pub fn record_expiration(&mut self) {
        self.expirations += 1;
    }

    /// Update hit ratio
    fn update_hit_ratio(&mut self) {
        let total = self.hits + self.misses;
        if total > 0 {
            self.hit_ratio = self.hits as f64 / total as f64;
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.hit_ratio = 0.0;
        self.evictions = 0;
        self.expirations = 0;
        self.last_reset = SystemTime::now();
    }
}

impl MemoryCache {
    /// Create new memory cache
    pub fn new(config: CacheConfig) -> Self {
        Self {
            partitions: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CacheStats::new())),
            warming_queue: Arc::new(RwLock::new(VecDeque::new())),
            analytics: Arc::new(RwLock::new(CacheAnalytics::new())),
            config,
        }
    }

    /// Get or create partition
    fn get_or_create_partition(&self, domain: &str) -> CachePartition {
        let mut partitions = self.partitions.write();
        
        if let Some(partition) = partitions.get(domain) {
            partition.clone()
        } else {
            let partition = CachePartition::new(
                domain.to_string(),
                self.config.max_memory_size / 10, // Divide by 10 for domain partitioning
                1000, // Max entries per partition
                self.config.default_eviction_policy,
            );
            partitions.insert(domain.to_string(), partition.clone());
            partition
        }
    }

    /// Get entry from cache
    pub fn get(&self, url: &str) -> Option<CacheEntry> {
        let domain = self.extract_domain(url);
        let mut partition = self.get_or_create_partition(&domain);
        
        if let Some(entry) = partition.get_entry(url) {
            if entry.is_fresh() {
                self.stats.write().record_hit();
                Some(entry.clone())
            } else {
                self.stats.write().record_miss();
                None
            }
        } else {
            self.stats.write().record_miss();
            None
        }
    }

    /// Put entry in cache
    pub fn put(&self, url: &str, entry: CacheEntry) {
        let domain = self.extract_domain(url);
        let mut partition = self.get_or_create_partition(&domain);
        
        partition.add_entry(url.to_string(), entry);
        
        // Update partitions
        let mut partitions = self.partitions.write();
        partitions.insert(domain, partition);
        
        // Update statistics
        let mut stats = self.stats.write();
        stats.total_size = partitions.values().map(|p| p.current_size).sum();
        stats.total_entries = partitions.values().map(|p| p.current_entries).sum();
    }

    /// Remove entry from cache
    pub fn remove(&self, url: &str) -> Option<CacheEntry> {
        let domain = self.extract_domain(url);
        let mut partitions = self.partitions.write();
        
        if let Some(partition) = partitions.get_mut(&domain) {
            partition.remove_entry(url)
        } else {
            None
        }
    }

    /// Clean expired entries
    pub fn clean_expired(&self) -> usize {
        let mut partitions = self.partitions.write();
        let mut total_expired = 0;
        
        for partition in partitions.values_mut() {
            total_expired += partition.clean_expired();
        }
        
        // Update statistics
        let mut stats = self.stats.write();
        stats.expirations += total_expired as u64;
        stats.total_size = partitions.values().map(|p| p.current_size).sum();
        stats.total_entries = partitions.values().map(|p| p.current_entries).sum();
        
        total_expired
    }

    /// Extract domain from URL
    fn extract_domain(&self, url: &str) -> String {
        if let Ok(parsed_url) = url::Url::parse(url) {
            parsed_url.host_str().unwrap_or("unknown").to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> CacheStats {
        self.stats.read().clone()
    }

    /// Get analytics
    pub fn get_analytics(&self) -> CacheAnalytics {
        self.analytics.read().clone()
    }
}

impl DiskCache {
    /// Create new disk cache
    pub fn new(config: CacheConfig) -> Result<Self> {
        let cache_dir = config.cache_directory
            .unwrap_or_else(|| PathBuf::from("./cache"));
        let index_file = cache_dir.join("index.json");
        
        // Create cache directory if it doesn't exist
        fs::create_dir_all(&cache_dir)?;
        
        // Load or create index
        let index = if index_file.exists() {
            let file = File::open(&index_file)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| HashMap::new())
        } else {
            HashMap::new()
        };
        
        Ok(Self {
            cache_dir,
            index_file,
            index: Arc::new(RwLock::new(index)),
            stats: Arc::new(RwLock::new(CacheStats::new())),
            config,
        })
    }

    /// Get entry from disk cache
    pub fn get(&self, url: &str) -> Option<CacheEntry> {
        let index = self.index.read();
        
        if let Some(entry) = index.get(url) {
            if entry.is_fresh() {
                // Try to read data from disk
                if let Ok(data) = self.read_data(url) {
                    let mut entry = entry.clone();
                    entry.data = data;
                    self.stats.write().record_hit();
                    return Some(entry);
                }
            }
        }
        
        self.stats.write().record_miss();
        None
    }

    /// Put entry in disk cache
    pub fn put(&self, url: &str, entry: &mut CacheEntry) -> Result<()> {
        // Write data to disk
        self.write_data(url, &entry.data)?;
        
        // Update index
        let mut index = self.index.write();
        index.insert(url.to_string(), entry.clone());
        
        // Save index to disk
        self.save_index(&index)?;
        
        // Update statistics
        let mut stats = self.stats.write();
        stats.total_size = index.values().map(|e| e.size).sum();
        stats.total_entries = index.len() as u64;
        
        Ok(())
    }

    /// Remove entry from disk cache
    pub fn remove(&self, url: &str) -> Result<()> {
        // Remove data file
        let data_file = self.get_data_path(url);
        if data_file.exists() {
            fs::remove_file(data_file)?;
        }
        
        // Remove from index
        let mut index = self.index.write();
        index.remove(url);
        
        // Save index to disk
        self.save_index(&index)?;
        
        Ok(())
    }

    /// Read data from disk
    fn read_data(&self, url: &str) -> Result<Vec<u8>> {
        let data_file = self.get_data_path(url);
        let mut file = File::open(data_file)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }

    /// Write data to disk
    fn write_data(&self, url: &str, data: &[u8]) -> Result<()> {
        let data_file = self.get_data_path(url);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(data_file)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }

    /// Get data file path
    fn get_data_path(&self, url: &str) -> PathBuf {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        self.cache_dir.join(&hash[..8])
    }

    /// Save index to disk
    fn save_index(&self, index: &HashMap<String, CacheEntry>) -> Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.index_file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, index)?;
        Ok(())
    }

    /// Clean expired entries
    pub fn clean_expired(&self) -> Result<usize> {
        let mut index = self.index.write();
        let mut expired_keys = Vec::new();
        
        for (key, entry) in index.iter() {
            if entry.is_expired() {
                expired_keys.push(key.clone());
            }
        }
        
        let expired_count = expired_keys.len();
        
        for key in expired_keys {
            self.remove(&key)?;
        }
        
        // Update statistics
        let mut stats = self.stats.write();
        stats.expirations += expired_count as u64;
        stats.total_size = index.values().map(|e| e.size).sum();
        stats.total_entries = index.len() as u64;
        
        Ok(expired_count)
    }

    /// Get statistics
    pub fn get_stats(&self) -> CacheStats {
        self.stats.read().clone()
    }
}

impl CacheManager {
    /// Create new cache manager
    pub fn new(config: CacheConfig) -> Result<Self> {
        let memory_cache = if config.enable_memory_cache {
            Some(MemoryCache::new(config.clone()))
        } else {
            None
        };
        
        let disk_cache = if config.enable_disk_cache {
            Some(DiskCache::new(config.clone())?)
        } else {
            None
        };
        
        let warming_manager = if config.enable_cache_warming {
            Some(CacheWarmingManager::new(config.clone()))
        } else {
            None
        };
        
        Ok(Self {
            memory_cache,
            disk_cache,
            config,
            stats: Arc::new(RwLock::new(CacheStats::new())),
            warming_manager,
        })
    }

    /// Get entry from cache
    pub fn get(&self, url: &str) -> Option<CacheEntry> {
        // Try memory cache first
        if let Some(ref memory_cache) = self.memory_cache {
            if let Some(entry) = memory_cache.get(url) {
                return Some(entry);
            }
        }
        
        // Try disk cache
        if let Some(ref disk_cache) = self.disk_cache {
            if let Some(entry) = disk_cache.get(url) {
                // Store in memory cache for faster access
                if let Some(ref memory_cache) = self.memory_cache {
                    memory_cache.put(url, entry.clone());
                }
                return Some(entry);
            }
        }
        
        None
    }

    /// Put entry in cache
    pub fn put(&self, url: &str, mut entry: CacheEntry) -> Result<()> {
        // Store in memory cache
        if let Some(ref memory_cache) = self.memory_cache {
            memory_cache.put(url, entry.clone());
        }
        
        // Store in disk cache
        if let Some(ref disk_cache) = self.disk_cache {
            disk_cache.put(url, &mut entry)?;
        }
        
        Ok(())
    }

    /// Remove entry from cache
    pub fn remove(&self, url: &str) -> Result<()> {
        // Remove from memory cache
        if let Some(ref memory_cache) = self.memory_cache {
            memory_cache.remove(url);
        }
        
        // Remove from disk cache
        if let Some(ref disk_cache) = self.disk_cache {
            disk_cache.remove(url)?;
        }
        
        Ok(())
    }

    /// Clean expired entries
    pub fn clean_expired(&self) -> Result<usize> {
        let mut total_expired = 0;
        
        // Clean memory cache
        if let Some(ref memory_cache) = self.memory_cache {
            total_expired += memory_cache.clean_expired();
        }
        
        // Clean disk cache
        if let Some(ref disk_cache) = self.disk_cache {
            total_expired += disk_cache.clean_expired()?;
        }
        
        Ok(total_expired)
    }

    /// Add warming entry
    pub fn add_warming_entry(&self, entry: CacheWarmingEntry) {
        if let Some(ref warming_manager) = self.warming_manager {
            warming_manager.add_entry(entry);
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> CacheStats {
        let mut stats = self.stats.read().clone();
        
        // Combine statistics from all caches
        if let Some(ref memory_cache) = self.memory_cache {
            let mem_stats = memory_cache.get_stats();
            stats.hits += mem_stats.hits;
            stats.misses += mem_stats.misses;
            stats.total_size += mem_stats.total_size;
            stats.total_entries += mem_stats.total_entries;
            stats.evictions += mem_stats.evictions;
            stats.expirations += mem_stats.expirations;
        }
        
        if let Some(ref disk_cache) = self.disk_cache {
            let disk_stats = disk_cache.get_stats();
            stats.hits += disk_stats.hits;
            stats.misses += disk_stats.misses;
            stats.total_size += disk_stats.total_size;
            stats.total_entries += disk_stats.total_entries;
            stats.evictions += disk_stats.evictions;
            stats.expirations += disk_stats.expirations;
        }
        
        // Update hit ratio
        let total = stats.hits + stats.misses;
        if total > 0 {
            stats.hit_ratio = stats.hits as f64 / total as f64;
        }
        
        stats
    }

    /// Get analytics
    pub fn get_analytics(&self) -> CacheAnalytics {
        if let Some(ref memory_cache) = self.memory_cache {
            memory_cache.get_analytics()
        } else {
            CacheAnalytics::new()
        }
    }
}

impl CacheWarmingManager {
    /// Create new cache warming manager
    pub fn new(config: CacheConfig) -> Self {
        let queue = Arc::new(RwLock::new(VecDeque::new()));
        let workers = Vec::new();
        
        Self {
            queue,
            workers,
            config,
        }
    }

    /// Add warming entry
    pub fn add_entry(&self, entry: CacheWarmingEntry) {
        self.queue.write().push_back(entry);
    }

    /// Start warming workers
    pub fn start_workers(&mut self, worker_count: usize) {
        for _ in 0..worker_count {
            let queue = self.queue.clone();
            let config = self.config.clone();
            
            let handle = tokio::spawn(async move {
                // TODO: Implement warming worker logic
                // This would fetch URLs and cache them
            });
            
            self.workers.push(handle);
        }
    }

    /// Stop warming workers
    pub fn stop_workers(&mut self) {
        for worker in self.workers.drain(..) {
            worker.abort();
        }
    }
}

impl CacheAnalytics {
    /// Create new cache analytics
    pub fn new() -> Self {
        Self {
            hit_rate_by_domain: HashMap::new(),
            hit_rate_by_content_type: HashMap::new(),
            avg_response_time: Duration::ZERO,
            cache_efficiency: 0.0,
            storage_utilization: 0.0,
            eviction_rate: 0.0,
            warming_success_rate: 0.0,
        }
    }

    /// Update analytics
    pub fn update(&mut self, stats: &CacheStats) {
        // Update cache efficiency
        let total_requests = stats.hits + stats.misses;
        if total_requests > 0 {
            self.cache_efficiency = stats.hits as f64 / total_requests as f64;
        }
        
        // Update eviction rate
        if stats.total_entries > 0 {
            self.eviction_rate = stats.evictions as f64 / stats.total_entries as f64;
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_total_size: 100 * 1024 * 1024, // 100 MB
            max_memory_size: 50 * 1024 * 1024,  // 50 MB
            max_disk_size: 500 * 1024 * 1024,   // 500 MB
            default_ttl: Duration::from_secs(3600), // 1 hour
            enable_memory_cache: true,
            enable_disk_cache: true,
            enable_cache_warming: false,
            enable_analytics: true,
            compression_threshold: 1024, // 1 KB
            default_eviction_policy: EvictionPolicy::LRU,
            partition_by_domain: true,
            cache_directory: None,
        }
    }
}
