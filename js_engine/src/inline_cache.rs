use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// Cache entry for property access
#[derive(Debug, Clone)]
pub struct PropertyCacheEntry {
    /// Object shape/hidden class identifier
    pub shape_id: u64,
    /// Property offset in the object
    pub offset: usize,
    /// Property value
    pub value: Value,
    /// Cache hit count
    pub hit_count: u32,
    /// Last access timestamp
    pub last_access: u64,
}

/// Cache entry for method calls
#[derive(Debug, Clone)]
pub struct MethodCacheEntry {
    /// Object shape/hidden class identifier
    pub shape_id: u64,
    /// Method function reference
    pub method: FunctionValue,
    /// Method offset in the object
    pub offset: usize,
    /// Cache hit count
    pub hit_count: u32,
    /// Last access timestamp
    pub last_access: u64,
}

/// Cache entry for global variable access
#[derive(Debug, Clone)]
pub struct GlobalCacheEntry {
    /// Variable name
    pub name: String,
    /// Variable value
    pub value: Value,
    /// Cache hit count
    pub hit_count: u32,
    /// Last access timestamp
    pub last_access: u64,
}

/// JavaScript value for cache operations
#[derive(Debug, Clone)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(ObjectValue),
    Array(Vec<Value>),
    Function(FunctionValue),
    Class(ClassValue),
}

/// Object value with shape tracking
#[derive(Debug, Clone)]
pub struct ObjectValue {
    /// Object shape identifier
    pub shape_id: u64,
    /// Object properties
    pub properties: HashMap<String, Value>,
    /// Object prototype
    pub prototype: Option<Arc<ObjectValue>>,
}

/// Function value
#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub name: String,
    pub param_count: u32,
    pub local_count: u32,
    pub closure: HashMap<String, Value>,
}

/// Class value
#[derive(Debug, Clone)]
pub struct ClassValue {
    pub name: String,
    pub constructor: Option<FunctionValue>,
    pub methods: HashMap<String, FunctionValue>,
    pub static_methods: HashMap<String, FunctionValue>,
    pub properties: HashMap<String, Value>,
}

/// Inline cache for property access
#[derive(Debug)]
pub struct PropertyCache {
    /// Cache entries indexed by object and property name
    entries: HashMap<(u64, String), PropertyCacheEntry>,
    /// Maximum cache size
    max_size: usize,
    /// Cache hit statistics
    hits: u64,
    /// Cache miss statistics
    misses: u64,
}

impl PropertyCache {
    /// Create a new property cache
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }

    /// Look up a property in the cache
    pub fn lookup(&mut self, object_id: u64, property_name: &str) -> Option<&PropertyCacheEntry> {
        let key = (object_id, property_name.to_string());
        if let Some(entry) = self.entries.get(&key) {
            self.hits += 1;
            Some(entry)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Store a property in the cache
    pub fn store(&mut self, object_id: u64, property_name: String, shape_id: u64, offset: usize, value: Value) {
        let key = (object_id, property_name);
        
        // Evict if cache is full
        if self.entries.len() >= self.max_size {
            self.evict_least_used();
        }

        let entry = PropertyCacheEntry {
            shape_id,
            offset,
            value,
            hit_count: 1,
            last_access: self.get_timestamp(),
        };

        self.entries.insert(key, entry);
    }

    /// Update an existing cache entry
    pub fn update(&mut self, object_id: u64, property_name: &str, value: Value) {
        let key = (object_id, property_name.to_string());
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.value = value;
            entry.hit_count += 1;
            entry.last_access = self.get_timestamp();
        }
    }

    /// Invalidate cache entries for an object
    pub fn invalidate_object(&mut self, object_id: u64) {
        self.entries.retain(|(id, _), _| *id != object_id);
    }

    /// Invalidate cache entries for a specific property
    pub fn invalidate_property(&mut self, object_id: u64, property_name: &str) {
        let key = (object_id, property_name.to_string());
        self.entries.remove(&key);
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            size: self.entries.len(),
            max_size: self.max_size,
            hits: self.hits,
            misses: self.misses,
            hit_rate: if self.hits + self.misses > 0 {
                self.hits as f64 / (self.hits + self.misses) as f64
            } else {
                0.0
            },
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Evict least used entries
    fn evict_least_used(&mut self) {
        let target_size = self.max_size / 2; // Evict half the cache
        
        if self.entries.len() <= target_size {
            return;
        }

        // Find entries with lowest hit count and oldest access time
        let mut entries: Vec<_> = self.entries.drain().collect();
        entries.sort_by(|(_, a), (_, b)| {
            a.hit_count.cmp(&b.hit_count)
                .then(a.last_access.cmp(&b.last_access))
        });

        // Keep the best entries
        for (key, entry) in entries.into_iter().take(target_size) {
            self.entries.insert(key, entry);
        }
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

/// Inline cache for method calls
#[derive(Debug)]
pub struct MethodCache {
    /// Cache entries indexed by object and method name
    entries: HashMap<(u64, String), MethodCacheEntry>,
    /// Maximum cache size
    max_size: usize,
    /// Cache hit statistics
    hits: u64,
    /// Cache miss statistics
    misses: u64,
}

impl MethodCache {
    /// Create a new method cache
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }

    /// Look up a method in the cache
    pub fn lookup(&mut self, object_id: u64, method_name: &str) -> Option<&MethodCacheEntry> {
        let key = (object_id, method_name.to_string());
        if let Some(entry) = self.entries.get(&key) {
            self.hits += 1;
            Some(entry)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Store a method in the cache
    pub fn store(&mut self, object_id: u64, method_name: String, shape_id: u64, offset: usize, method: FunctionValue) {
        let key = (object_id, method_name);
        
        // Evict if cache is full
        if self.entries.len() >= self.max_size {
            self.evict_least_used();
        }

        let entry = MethodCacheEntry {
            shape_id,
            method,
            offset,
            hit_count: 1,
            last_access: self.get_timestamp(),
        };

        self.entries.insert(key, entry);
    }

    /// Update an existing cache entry
    pub fn update(&mut self, object_id: u64, method_name: &str, method: FunctionValue) {
        let key = (object_id, method_name.to_string());
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.method = method;
            entry.hit_count += 1;
            entry.last_access = self.get_timestamp();
        }
    }

    /// Invalidate cache entries for an object
    pub fn invalidate_object(&mut self, object_id: u64) {
        self.entries.retain(|(id, _), _| *id != object_id);
    }

    /// Invalidate cache entries for a specific method
    pub fn invalidate_method(&mut self, object_id: u64, method_name: &str) {
        let key = (object_id, method_name.to_string());
        self.entries.remove(&key);
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            size: self.entries.len(),
            max_size: self.max_size,
            hits: self.hits,
            misses: self.misses,
            hit_rate: if self.hits + self.misses > 0 {
                self.hits as f64 / (self.hits + self.misses) as f64
            } else {
                0.0
            },
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Evict least used entries
    fn evict_least_used(&mut self) {
        let target_size = self.max_size / 2;
        
        if self.entries.len() <= target_size {
            return;
        }

        let mut entries: Vec<_> = self.entries.drain().collect();
        entries.sort_by(|(_, a), (_, b)| {
            a.hit_count.cmp(&b.hit_count)
                .then(a.last_access.cmp(&b.last_access))
        });

        for (key, entry) in entries.into_iter().take(target_size) {
            self.entries.insert(key, entry);
        }
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

/// Inline cache for global variable access
#[derive(Debug)]
pub struct GlobalCache {
    /// Cache entries indexed by variable name
    entries: HashMap<String, GlobalCacheEntry>,
    /// Maximum cache size
    max_size: usize,
    /// Cache hit statistics
    hits: u64,
    /// Cache miss statistics
    misses: u64,
}

impl GlobalCache {
    /// Create a new global cache
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }

    /// Look up a global variable in the cache
    pub fn lookup(&mut self, name: &str) -> Option<&GlobalCacheEntry> {
        if let Some(entry) = self.entries.get(name) {
            self.hits += 1;
            Some(entry)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Store a global variable in the cache
    pub fn store(&mut self, name: String, value: Value) {
        // Evict if cache is full
        if self.entries.len() >= self.max_size {
            self.evict_least_used();
        }

        let entry = GlobalCacheEntry {
            name: name.clone(),
            value,
            hit_count: 1,
            last_access: self.get_timestamp(),
        };

        self.entries.insert(name, entry);
    }

    /// Update an existing cache entry
    pub fn update(&mut self, name: &str, value: Value) {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.value = value;
            entry.hit_count += 1;
            entry.last_access = self.get_timestamp();
        }
    }

    /// Invalidate cache entry for a specific variable
    pub fn invalidate(&mut self, name: &str) {
        self.entries.remove(name);
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            size: self.entries.len(),
            max_size: self.max_size,
            hits: self.hits,
            misses: self.misses,
            hit_rate: if self.hits + self.misses > 0 {
                self.hits as f64 / (self.hits + self.misses) as f64
            } else {
                0.0
            },
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Evict least used entries
    fn evict_least_used(&mut self) {
        let target_size = self.max_size / 2;
        
        if self.entries.len() <= target_size {
            return;
        }

        let mut entries: Vec<_> = self.entries.drain().collect();
        entries.sort_by(|(_, a), (_, b)| {
            a.hit_count.cmp(&b.hit_count)
                .then(a.last_access.cmp(&b.last_access))
        });

        for (name, entry) in entries.into_iter().take(target_size) {
            self.entries.insert(name, entry);
        }
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
}

/// Main inline cache manager
pub struct InlineCacheManager {
    /// Property access cache
    property_cache: Arc<RwLock<PropertyCache>>,
    /// Method call cache
    method_cache: Arc<RwLock<MethodCache>>,
    /// Global variable cache
    global_cache: Arc<RwLock<GlobalCache>>,
    /// Object shape registry
    shape_registry: Arc<RwLock<ShapeRegistry>>,
}

/// Object shape registry for tracking object layouts
#[derive(Debug)]
pub struct ShapeRegistry {
    /// Next available shape ID
    next_shape_id: u64,
    /// Shape definitions
    shapes: HashMap<u64, ShapeDefinition>,
}

/// Shape definition for object layout
#[derive(Debug, Clone)]
pub struct ShapeDefinition {
    /// Shape ID
    pub id: u64,
    /// Property names in order
    pub properties: Vec<String>,
    /// Property offsets
    pub offsets: HashMap<String, usize>,
    /// Parent shape (for inheritance)
    pub parent: Option<u64>,
}

impl ShapeRegistry {
    /// Create a new shape registry
    pub fn new() -> Self {
        Self {
            next_shape_id: 1,
            shapes: HashMap::new(),
        }
    }

    /// Create a new shape
    pub fn create_shape(&mut self, properties: Vec<String>, parent: Option<u64>) -> u64 {
        let shape_id = self.next_shape_id;
        self.next_shape_id += 1;

        let mut offsets = HashMap::new();
        for (i, prop) in properties.iter().enumerate() {
            offsets.insert(prop.clone(), i);
        }

        let shape = ShapeDefinition {
            id: shape_id,
            properties,
            offsets,
            parent,
        };

        self.shapes.insert(shape_id, shape);
        shape_id
    }

    /// Get a shape definition
    pub fn get_shape(&self, shape_id: u64) -> Option<&ShapeDefinition> {
        self.shapes.get(&shape_id)
    }

    /// Transition a shape by adding a property
    pub fn transition_shape(&mut self, base_shape_id: u64, new_property: String) -> u64 {
        let base_shape = self.shapes.get(&base_shape_id).cloned();
        
        let mut properties = if let Some(ref base) = base_shape {
            base.properties.clone()
        } else {
            Vec::new()
        };

        if !properties.contains(&new_property) {
            properties.push(new_property);
        }

        self.create_shape(properties, Some(base_shape_id))
    }

    /// Get all shapes
    pub fn get_all_shapes(&self) -> &HashMap<u64, ShapeDefinition> {
        &self.shapes
    }

    /// Clear all shapes
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.next_shape_id = 1;
    }
}

impl InlineCacheManager {
    /// Create a new inline cache manager
    pub fn new(property_cache_size: usize, method_cache_size: usize, global_cache_size: usize) -> Self {
        Self {
            property_cache: Arc::new(RwLock::new(PropertyCache::new(property_cache_size))),
            method_cache: Arc::new(RwLock::new(MethodCache::new(method_cache_size))),
            global_cache: Arc::new(RwLock::new(GlobalCache::new(global_cache_size))),
            shape_registry: Arc::new(RwLock::new(ShapeRegistry::new())),
        }
    }

    /// Get property cache
    pub fn property_cache(&self) -> Arc<RwLock<PropertyCache>> {
        Arc::clone(&self.property_cache)
    }

    /// Get method cache
    pub fn method_cache(&self) -> Arc<RwLock<MethodCache>> {
        Arc::clone(&self.method_cache)
    }

    /// Get global cache
    pub fn global_cache(&self) -> Arc<RwLock<GlobalCache>> {
        Arc::clone(&self.global_cache)
    }

    /// Get shape registry
    pub fn shape_registry(&self) -> Arc<RwLock<ShapeRegistry>> {
        Arc::clone(&self.shape_registry)
    }

    /// Look up a property with caching
    pub fn lookup_property(&self, object_id: u64, property_name: &str) -> Option<Value> {
        let mut cache = self.property_cache.write();
        if let Some(entry) = cache.lookup(object_id, property_name) {
            Some(entry.value.clone())
        } else {
            None
        }
    }

    /// Store a property in cache
    pub fn store_property(&self, object_id: u64, property_name: String, shape_id: u64, offset: usize, value: Value) {
        let mut cache = self.property_cache.write();
        cache.store(object_id, property_name, shape_id, offset, value);
    }

    /// Look up a method with caching
    pub fn lookup_method(&self, object_id: u64, method_name: &str) -> Option<FunctionValue> {
        let mut cache = self.method_cache.write();
        if let Some(entry) = cache.lookup(object_id, method_name) {
            Some(entry.method.clone())
        } else {
            None
        }
    }

    /// Store a method in cache
    pub fn store_method(&self, object_id: u64, method_name: String, shape_id: u64, offset: usize, method: FunctionValue) {
        let mut cache = self.method_cache.write();
        cache.store(object_id, method_name, shape_id, offset, method);
    }

    /// Look up a global variable with caching
    pub fn lookup_global(&self, name: &str) -> Option<Value> {
        let mut cache = self.global_cache.write();
        if let Some(entry) = cache.lookup(name) {
            Some(entry.value.clone())
        } else {
            None
        }
    }

    /// Store a global variable in cache
    pub fn store_global(&self, name: String, value: Value) {
        let mut cache = self.global_cache.write();
        cache.store(name, value);
    }

    /// Invalidate all caches for an object
    pub fn invalidate_object(&self, object_id: u64) {
        {
            let mut property_cache = self.property_cache.write();
            property_cache.invalidate_object(object_id);
        }
        {
            let mut method_cache = self.method_cache.write();
            method_cache.invalidate_object(object_id);
        }
    }

    /// Get comprehensive cache statistics
    pub fn get_stats(&self) -> InlineCacheStats {
        InlineCacheStats {
            property_cache: self.property_cache.read().get_stats(),
            method_cache: self.method_cache.read().get_stats(),
            global_cache: self.global_cache.read().get_stats(),
            shape_count: self.shape_registry.read().shapes.len(),
        }
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        {
            let mut property_cache = self.property_cache.write();
            property_cache.clear();
        }
        {
            let mut method_cache = self.method_cache.write();
            method_cache.clear();
        }
        {
            let mut global_cache = self.global_cache.write();
            global_cache.clear();
        }
        {
            let mut shape_registry = self.shape_registry.write();
            shape_registry.clear();
        }
    }
}

/// Comprehensive cache statistics
#[derive(Debug, Clone)]
pub struct InlineCacheStats {
    pub property_cache: CacheStats,
    pub method_cache: CacheStats,
    pub global_cache: CacheStats,
    pub shape_count: usize,
}
