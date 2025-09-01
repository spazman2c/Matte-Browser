use crate::error::{Error, Result};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use std::time::{Duration, Instant};

/// Memory pool types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PoolType {
    /// Nursery for short-lived objects
    Nursery,
    /// Small object pool (8-64 bytes)
    Small,
    /// Medium object pool (64-512 bytes)
    Medium,
    /// Large object pool (512+ bytes)
    Large,
    /// String pool for string objects
    String,
    /// Array pool for array objects
    Array,
    /// Object pool for plain objects
    Object,
    /// Function pool for function objects
    Function,
}

/// Memory pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Pool type
    pub pool_type: PoolType,
    /// Object size in bytes
    pub object_size: usize,
    /// Number of objects per pool
    pub objects_per_pool: usize,
    /// Maximum number of pools
    pub max_pools: usize,
    /// Whether the pool is enabled
    pub enabled: bool,
    /// Growth factor for pool expansion
    pub growth_factor: f64,
    /// Shrink threshold (percentage of used objects)
    pub shrink_threshold: f64,
}

/// Memory pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    /// Pool type
    pub pool_type: PoolType,
    /// Total number of pools
    pub total_pools: usize,
    /// Total number of objects allocated
    pub total_objects: usize,
    /// Number of objects currently in use
    pub objects_in_use: usize,
    /// Number of objects available
    pub objects_available: usize,
    /// Total memory allocated in bytes
    pub total_memory: usize,
    /// Memory in use in bytes
    pub memory_in_use: usize,
    /// Allocation count
    pub allocation_count: u64,
    /// Deallocation count
    pub deallocation_count: u64,
    /// Hit rate (successful allocations)
    pub hit_rate: f64,
    /// Average allocation time in microseconds
    pub avg_allocation_time_us: f64,
    /// Last allocation time
    pub last_allocation_time_us: f64,
}

/// Memory pool entry
#[derive(Debug, Clone)]
pub struct PoolEntry {
    /// Entry identifier
    pub id: u64,
    /// Pool type
    pub pool_type: PoolType,
    /// Object size
    pub size: usize,
    /// Allocation timestamp
    pub allocated_at: Instant,
    /// Last access timestamp
    pub last_accessed: Instant,
    /// Whether the entry is in use
    pub is_in_use: bool,
    /// Object data
    pub data: Vec<u8>,
    /// Reference count
    pub reference_count: u32,
}

/// Memory pool implementation
#[derive(Debug)]
pub struct MemoryPool {
    /// Pool configuration
    config: PoolConfig,
    /// Pool entries
    entries: Arc<RwLock<VecDeque<PoolEntry>>>,
    /// Pool statistics
    stats: Arc<RwLock<PoolStats>>,
    /// Next entry ID
    next_entry_id: Arc<RwLock<u64>>,
    /// Pool expansion history
    expansion_history: Arc<RwLock<Vec<Instant>>>,
}

/// Nursery for short-lived objects
#[derive(Debug)]
pub struct Nursery {
    /// Memory pools for different object types
    pools: Arc<RwLock<HashMap<PoolType, MemoryPool>>>,
    /// Nursery configuration
    config: NurseryConfig,
    /// Nursery statistics
    stats: Arc<RwLock<NurseryStats>>,
    /// Promotion threshold
    promotion_threshold: u32,
}

/// Nursery configuration
#[derive(Debug, Clone)]
pub struct NurseryConfig {
    /// Maximum nursery size in bytes
    pub max_size: usize,
    /// Promotion threshold (survival count)
    pub promotion_threshold: u32,
    /// Collection frequency
    pub collection_frequency: f64,
    /// Whether nursery is enabled
    pub enabled: bool,
    /// Growth factor
    pub growth_factor: f64,
}

/// Nursery statistics
#[derive(Debug, Clone)]
pub struct NurseryStats {
    /// Total objects allocated
    pub total_objects: u64,
    /// Objects promoted to old generation
    pub promoted_objects: u64,
    /// Objects collected
    pub collected_objects: u64,
    /// Current nursery size in bytes
    pub current_size: usize,
    /// Peak nursery size in bytes
    pub peak_size: usize,
    /// Promotion rate
    pub promotion_rate: f64,
    /// Collection count
    pub collection_count: u64,
    /// Average collection time in milliseconds
    pub avg_collection_time_ms: f64,
}

/// Memory pool manager
pub struct MemoryPoolManager {
    /// Nursery for short-lived objects
    nursery: Arc<RwLock<Nursery>>,
    /// Memory pools for different object types
    pools: Arc<RwLock<HashMap<PoolType, MemoryPool>>>,
    /// Manager configuration
    config: ManagerConfig,
    /// Manager statistics
    stats: Arc<RwLock<ManagerStats>>,
}

/// Manager configuration
#[derive(Debug, Clone)]
pub struct ManagerConfig {
    /// Whether pooling is enabled
    pub enabled: bool,
    /// Default pool configurations
    pub pool_configs: HashMap<PoolType, PoolConfig>,
    /// Nursery configuration
    pub nursery_config: NurseryConfig,
    /// Memory pressure threshold
    pub memory_pressure_threshold: f64,
    /// Pool cleanup interval in seconds
    pub cleanup_interval: f64,
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStats {
    /// Total allocations
    pub total_allocations: u64,
    /// Total deallocations
    pub total_deallocations: u64,
    /// Current memory usage in bytes
    pub current_memory: usize,
    /// Peak memory usage in bytes
    pub peak_memory: usize,
    /// Memory efficiency (used/total)
    pub memory_efficiency: f64,
    /// Average allocation time in microseconds
    pub avg_allocation_time_us: f64,
    /// Pool hit rate
    pub pool_hit_rate: f64,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            pool_type: PoolType::Small,
            object_size: 64,
            objects_per_pool: 1000,
            max_pools: 10,
            enabled: true,
            growth_factor: 2.0,
            shrink_threshold: 0.3,
        }
    }
}

impl Default for NurseryConfig {
    fn default() -> Self {
        Self {
            max_size: 10 * 1024 * 1024, // 10MB
            promotion_threshold: 3,
            collection_frequency: 0.1,
            enabled: true,
            growth_factor: 1.5,
        }
    }
}

impl Default for ManagerConfig {
    fn default() -> Self {
        let mut pool_configs = HashMap::new();
        
        // Small object pool
        pool_configs.insert(PoolType::Small, PoolConfig {
            pool_type: PoolType::Small,
            object_size: 64,
            objects_per_pool: 1000,
            max_pools: 10,
            enabled: true,
            growth_factor: 2.0,
            shrink_threshold: 0.3,
        });
        
        // Medium object pool
        pool_configs.insert(PoolType::Medium, PoolConfig {
            pool_type: PoolType::Medium,
            object_size: 256,
            objects_per_pool: 500,
            max_pools: 5,
            enabled: true,
            growth_factor: 1.5,
            shrink_threshold: 0.4,
        });
        
        // Large object pool
        pool_configs.insert(PoolType::Large, PoolConfig {
            pool_type: PoolType::Large,
            object_size: 1024,
            objects_per_pool: 100,
            max_pools: 3,
            enabled: true,
            growth_factor: 1.2,
            shrink_threshold: 0.5,
        });
        
        // String pool
        pool_configs.insert(PoolType::String, PoolConfig {
            pool_type: PoolType::String,
            object_size: 128,
            objects_per_pool: 2000,
            max_pools: 15,
            enabled: true,
            growth_factor: 2.0,
            shrink_threshold: 0.2,
        });
        
        // Array pool
        pool_configs.insert(PoolType::Array, PoolConfig {
            pool_type: PoolType::Array,
            object_size: 512,
            objects_per_pool: 300,
            max_pools: 8,
            enabled: true,
            growth_factor: 1.8,
            shrink_threshold: 0.3,
        });
        
        // Object pool
        pool_configs.insert(PoolType::Object, PoolConfig {
            pool_type: PoolType::Object,
            object_size: 256,
            objects_per_pool: 400,
            max_pools: 6,
            enabled: true,
            growth_factor: 1.6,
            shrink_threshold: 0.35,
        });
        
        // Function pool
        pool_configs.insert(PoolType::Function, PoolConfig {
            pool_type: PoolType::Function,
            object_size: 1024,
            objects_per_pool: 150,
            max_pools: 4,
            enabled: true,
            growth_factor: 1.4,
            shrink_threshold: 0.4,
        });

        Self {
            enabled: true,
            pool_configs,
            nursery_config: NurseryConfig::default(),
            memory_pressure_threshold: 0.8,
            cleanup_interval: 30.0,
        }
    }
}

impl MemoryPool {
    /// Create a new memory pool
    pub fn new(config: PoolConfig) -> Self {
        let stats = PoolStats {
            pool_type: config.pool_type,
            total_pools: 0,
            total_objects: 0,
            objects_in_use: 0,
            objects_available: 0,
            total_memory: 0,
            memory_in_use: 0,
            allocation_count: 0,
            deallocation_count: 0,
            hit_rate: 0.0,
            avg_allocation_time_us: 0.0,
            last_allocation_time_us: 0.0,
        };

        let mut entries = VecDeque::new();
        
        // Pre-allocate initial pool
        for _ in 0..config.objects_per_pool {
            entries.push_back(PoolEntry {
                id: 0, // Will be set during allocation
                pool_type: config.pool_type,
                size: config.object_size,
                allocated_at: Instant::now(),
                last_accessed: Instant::now(),
                is_in_use: false,
                data: vec![0; config.object_size],
                reference_count: 0,
            });
        }

        Self {
            config,
            entries: Arc::new(RwLock::new(entries)),
            stats: Arc::new(RwLock::new(stats)),
            next_entry_id: Arc::new(RwLock::new(1)),
            expansion_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Allocate an object from the pool
    pub fn allocate(&self, data: Vec<u8>) -> Result<u64> {
        if !self.config.enabled {
            return Err(Error::parsing("Pool is disabled".to_string()));
        }

        let start_time = Instant::now();
        
        let mut entries = self.entries.write();
        let mut stats = self.stats.write();
        let mut next_id = self.next_entry_id.write();
        
        // Try to find an available entry
        let entry_index = entries.iter().position(|entry| !entry.is_in_use);
        
        if let Some(index) = entry_index {
            // Use existing entry
            let entry = &mut entries[index];
            entry.id = *next_id;
            entry.is_in_use = true;
            entry.allocated_at = Instant::now();
            entry.last_accessed = Instant::now();
            entry.data = data;
            entry.reference_count = 1;
            
            *next_id += 1;
            
            // Update statistics
            stats.objects_in_use += 1;
            stats.objects_available -= 1;
            stats.allocation_count += 1;
            stats.memory_in_use += entry.size;
            
            let allocation_time = start_time.elapsed().as_micros() as f64;
            stats.last_allocation_time_us = allocation_time;
            stats.avg_allocation_time_us = 
                (stats.avg_allocation_time_us * (stats.allocation_count - 1) as f64 + allocation_time) / stats.allocation_count as f64;
            
            Ok(entry.id)
        } else {
            // Need to expand pool
            self.expand_pool(&mut entries, &mut stats)?;
            
            // Try allocation again
            self.allocate(data)
        }
    }

    /// Deallocate an object back to the pool
    pub fn deallocate(&self, entry_id: u64) -> Result<()> {
        let mut entries = self.entries.write();
        let mut stats = self.stats.write();
        
        if let Some(entry) = entries.iter_mut().find(|e| e.id == entry_id && e.is_in_use) {
            entry.is_in_use = false;
            entry.reference_count = 0;
            entry.data.clear();
            entry.data.resize(self.config.object_size, 0);
            
            // Update statistics
            stats.objects_in_use -= 1;
            stats.objects_available += 1;
            stats.deallocation_count += 1;
            stats.memory_in_use -= entry.size;
            
            // Update hit rate
            stats.hit_rate = stats.allocation_count as f64 / (stats.allocation_count + stats.deallocation_count) as f64;
            
            Ok(())
        } else {
            Err(Error::parsing(format!("Entry {} not found or not in use", entry_id)))
        }
    }

    /// Expand the pool
    fn expand_pool(&self, entries: &mut VecDeque<PoolEntry>, stats: &mut PoolStats) -> Result<()> {
        if stats.total_pools >= self.config.max_pools {
            return Err(Error::parsing("Maximum number of pools reached".to_string()));
        }

        let new_pool_size = (self.config.objects_per_pool as f64 * self.config.growth_factor) as usize;
        
        // Add new entries
        for _ in 0..new_pool_size {
            entries.push_back(PoolEntry {
                id: 0,
                pool_type: self.config.pool_type,
                size: self.config.object_size,
                allocated_at: Instant::now(),
                last_accessed: Instant::now(),
                is_in_use: false,
                data: vec![0; self.config.object_size],
                reference_count: 0,
            });
        }
        
        // Update statistics
        stats.total_pools += 1;
        stats.total_objects += new_pool_size;
        stats.objects_available += new_pool_size;
        stats.total_memory += new_pool_size * self.config.object_size;
        
        // Record expansion
        let mut expansion_history = self.expansion_history.write();
        expansion_history.push(Instant::now());
        
        Ok(())
    }

    /// Shrink the pool if needed
    pub fn shrink_pool(&self) -> Result<()> {
        let mut entries = self.entries.write();
        let mut stats = self.stats.write();
        
        let usage_rate = stats.objects_in_use as f64 / stats.total_objects as f64;
        
        if usage_rate < self.config.shrink_threshold && stats.total_pools > 1 {
            // Remove one pool worth of entries
            let pool_size = self.config.objects_per_pool;
            let entries_to_remove: Vec<_> = entries
                .iter()
                .filter(|e| !e.is_in_use)
                .take(pool_size)
                .map(|e| e.id)
                .collect();
            
            entries.retain(|e| !entries_to_remove.contains(&e.id));
            
            // Update statistics
            stats.total_pools -= 1;
            stats.total_objects -= pool_size;
            stats.objects_available -= pool_size;
            stats.total_memory -= pool_size * self.config.object_size;
        }
        
        Ok(())
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> PoolStats {
        self.stats.read().clone()
    }

    /// Get pool entry by ID
    pub fn get_entry(&self, entry_id: u64) -> Option<PoolEntry> {
        let entries = self.entries.read();
        entries.iter().find(|e| e.id == entry_id).cloned()
    }

    /// Clear the pool
    pub fn clear(&self) {
        let mut entries = self.entries.write();
        let mut stats = self.stats.write();
        
        entries.clear();
        *stats = PoolStats {
            pool_type: self.config.pool_type,
            total_pools: 0,
            total_objects: 0,
            objects_in_use: 0,
            objects_available: 0,
            total_memory: 0,
            memory_in_use: 0,
            allocation_count: 0,
            deallocation_count: 0,
            hit_rate: 0.0,
            avg_allocation_time_us: 0.0,
            last_allocation_time_us: 0.0,
        };
    }
}

impl Nursery {
    /// Create a new nursery
    pub fn new(config: NurseryConfig) -> Self {
        let mut pools = HashMap::new();
        
        // Create pools for different object types
        for pool_type in &[PoolType::Small, PoolType::String, PoolType::Array, PoolType::Object] {
            let pool_config = PoolConfig {
                pool_type: *pool_type,
                object_size: match pool_type {
                    PoolType::Small => 64,
                    PoolType::String => 128,
                    PoolType::Array => 512,
                    PoolType::Object => 256,
                    _ => 64,
                },
                objects_per_pool: match pool_type {
                    PoolType::Small => 1000,
                    PoolType::String => 2000,
                    PoolType::Array => 300,
                    PoolType::Object => 400,
                    _ => 1000,
                },
                max_pools: match pool_type {
                    PoolType::Small => 10,
                    PoolType::String => 15,
                    PoolType::Array => 8,
                    PoolType::Object => 6,
                    _ => 10,
                },
                enabled: true,
                growth_factor: 2.0,
                shrink_threshold: 0.3,
            };
            
            pools.insert(*pool_type, MemoryPool::new(pool_config));
        }

        let stats = NurseryStats {
            total_objects: 0,
            promoted_objects: 0,
            collected_objects: 0,
            current_size: 0,
            peak_size: 0,
            promotion_rate: 0.0,
            collection_count: 0,
            avg_collection_time_ms: 0.0,
        };

        Self {
            pools: Arc::new(RwLock::new(pools)),
            config,
            stats: Arc::new(RwLock::new(stats)),
            promotion_threshold: config.promotion_threshold,
        }
    }

    /// Allocate an object in the nursery
    pub fn allocate(&self, pool_type: PoolType, data: Vec<u8>) -> Result<u64> {
        if !self.config.enabled {
            return Err(Error::parsing("Nursery is disabled".to_string()));
        }

        let pools = self.pools.read();
        if let Some(pool) = pools.get(&pool_type) {
            let entry_id = pool.allocate(data)?;
            
            // Update nursery statistics
            let mut stats = self.stats.write();
            stats.total_objects += 1;
            stats.current_size += data.len();
            stats.peak_size = stats.peak_size.max(stats.current_size);
            
            Ok(entry_id)
        } else {
            Err(Error::parsing(format!("Pool type {:?} not found in nursery", pool_type)))
        }
    }

    /// Promote an object from nursery to old generation
    pub fn promote_object(&self, entry_id: u64, pool_type: PoolType) -> Result<u64> {
        let pools = self.pools.read();
        if let Some(pool) = pools.get(&pool_type) {
            if let Some(entry) = pool.get_entry(entry_id) {
                // In a real implementation, this would move the object to the old generation
                // For now, we'll just update statistics
                let mut stats = self.stats.write();
                stats.promoted_objects += 1;
                stats.current_size -= entry.size;
                stats.promotion_rate = stats.promoted_objects as f64 / stats.total_objects as f64;
                
                Ok(entry_id)
            } else {
                Err(Error::parsing(format!("Entry {} not found", entry_id)))
            }
        } else {
            Err(Error::parsing(format!("Pool type {:?} not found", pool_type)))
        }
    }

    /// Collect nursery (remove short-lived objects)
    pub async fn collect(&self) -> Result<NurseryStats> {
        let start_time = Instant::now();
        
        let mut pools = self.pools.write();
        let mut stats = self.stats.write();
        
        let mut collected_count = 0;
        let mut collected_size = 0;
        
        // Collect from all pools
        for pool in pools.values_mut() {
            // In a real implementation, this would check object age and reference count
            // For now, we'll simulate collection
            let pool_stats = pool.get_stats();
            collected_count += pool_stats.objects_in_use / 2; // Simulate 50% collection
            collected_size += pool_stats.memory_in_use / 2;
        }
        
        // Update statistics
        stats.collected_objects += collected_count;
        stats.current_size -= collected_size;
        stats.collection_count += 1;
        
        let collection_time = start_time.elapsed().as_millis() as f64;
        stats.avg_collection_time_ms = 
            (stats.avg_collection_time_ms * (stats.collection_count - 1) as f64 + collection_time) / stats.collection_count as f64;
        
        Ok(stats.clone())
    }

    /// Get nursery statistics
    pub fn get_stats(&self) -> NurseryStats {
        self.stats.read().clone()
    }

    /// Clear the nursery
    pub fn clear(&self) {
        let mut pools = self.pools.write();
        for pool in pools.values_mut() {
            pool.clear();
        }
        
        let mut stats = self.stats.write();
        *stats = NurseryStats {
            total_objects: 0,
            promoted_objects: 0,
            collected_objects: 0,
            current_size: 0,
            peak_size: 0,
            promotion_rate: 0.0,
            collection_count: 0,
            avg_collection_time_ms: 0.0,
        };
    }
}

impl MemoryPoolManager {
    /// Create a new memory pool manager
    pub fn new(config: ManagerConfig) -> Self {
        let nursery = Nursery::new(config.nursery_config.clone());
        
        let mut pools = HashMap::new();
        for (pool_type, pool_config) in &config.pool_configs {
            pools.insert(*pool_type, MemoryPool::new(pool_config.clone()));
        }

        let stats = ManagerStats {
            total_allocations: 0,
            total_deallocations: 0,
            current_memory: 0,
            peak_memory: 0,
            memory_efficiency: 0.0,
            avg_allocation_time_us: 0.0,
            pool_hit_rate: 0.0,
        };

        Self {
            nursery: Arc::new(RwLock::new(nursery)),
            pools: Arc::new(RwLock::new(pools)),
            config,
            stats: Arc::new(RwLock::new(stats)),
        }
    }

    /// Allocate memory from appropriate pool
    pub async fn allocate(&self, pool_type: PoolType, data: Vec<u8>) -> Result<u64> {
        if !self.config.enabled {
            return Err(Error::parsing("Memory pooling is disabled".to_string()));
        }

        let start_time = Instant::now();
        
        // Try nursery first for short-lived objects
        if self.should_use_nursery(&pool_type) {
            let nursery = self.nursery.read();
            match nursery.allocate(pool_type, data) {
                Ok(entry_id) => {
                    self.update_manager_stats(start_time, true);
                    return Ok(entry_id);
                }
                Err(_) => {
                    // Fall back to regular pools
                }
            }
        }
        
        // Use regular pools
        let pools = self.pools.read();
        if let Some(pool) = pools.get(&pool_type) {
            let entry_id = pool.allocate(data)?;
            self.update_manager_stats(start_time, true);
            Ok(entry_id)
        } else {
            Err(Error::parsing(format!("Pool type {:?} not found", pool_type)))
        }
    }

    /// Deallocate memory back to pool
    pub fn deallocate(&self, pool_type: PoolType, entry_id: u64) -> Result<()> {
        // Try nursery first
        let nursery = self.nursery.read();
        if let Ok(_) = nursery.promote_object(entry_id, pool_type) {
            return Ok(());
        }
        
        // Use regular pools
        let pools = self.pools.read();
        if let Some(pool) = pools.get(&pool_type) {
            pool.deallocate(entry_id)?;
            
            let mut stats = self.stats.write();
            stats.total_deallocations += 1;
            
            Ok(())
        } else {
            Err(Error::parsing(format!("Pool type {:?} not found", pool_type)))
        }
    }

    /// Determine if object should use nursery
    fn should_use_nursery(&self, pool_type: &PoolType) -> bool {
        matches!(pool_type, PoolType::Small | PoolType::String | PoolType::Array | PoolType::Object)
    }

    /// Update manager statistics
    fn update_manager_stats(&self, start_time: Instant, success: bool) {
        if success {
            let mut stats = self.stats.write();
            stats.total_allocations += 1;
            
            let allocation_time = start_time.elapsed().as_micros() as f64;
            stats.avg_allocation_time_us = 
                (stats.avg_allocation_time_us * (stats.total_allocations - 1) as f64 + allocation_time) / stats.total_allocations as f64;
        }
    }

    /// Collect nursery
    pub async fn collect_nursery(&self) -> Result<NurseryStats> {
        let nursery = self.nursery.read();
        nursery.collect().await
    }

    /// Get memory pressure
    pub fn get_memory_pressure(&self) -> f64 {
        let nursery = self.nursery.read();
        let pools = self.pools.read();
        
        let nursery_stats = nursery.get_stats();
        let total_memory: usize = pools.values().map(|p| p.get_stats().total_memory).sum();
        
        let total_used = nursery_stats.current_size + total_memory;
        let total_available = nursery_stats.peak_size + total_memory;
        
        if total_available > 0 {
            total_used as f64 / total_available as f64
        } else {
            0.0
        }
    }

    /// Handle memory pressure
    pub async fn handle_memory_pressure(&self) -> Result<()> {
        let pressure = self.get_memory_pressure();
        
        if pressure > self.config.memory_pressure_threshold {
            // Trigger nursery collection
            self.collect_nursery().await?;
            
            // Shrink pools
            let mut pools = self.pools.write();
            for pool in pools.values_mut() {
                pool.shrink_pool()?;
            }
        }
        
        Ok(())
    }

    /// Get manager statistics
    pub fn get_stats(&self) -> ManagerStats {
        self.stats.read().clone()
    }

    /// Get pool statistics
    pub fn get_pool_stats(&self, pool_type: PoolType) -> Option<PoolStats> {
        let pools = self.pools.read();
        pools.get(&pool_type).map(|p| p.get_stats())
    }

    /// Get nursery statistics
    pub fn get_nursery_stats(&self) -> NurseryStats {
        let nursery = self.nursery.read();
        nursery.get_stats()
    }

    /// Clear all pools
    pub fn clear(&self) {
        let nursery = self.nursery.read();
        nursery.clear();
        
        let mut pools = self.pools.write();
        for pool in pools.values_mut() {
            pool.clear();
        }
        
        let mut stats = self.stats.write();
        *stats = ManagerStats {
            total_allocations: 0,
            total_deallocations: 0,
            current_memory: 0,
            peak_memory: 0,
            memory_efficiency: 0.0,
            avg_allocation_time_us: 0.0,
            pool_hit_rate: 0.0,
        };
    }
}
