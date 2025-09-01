use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// IndexedDB manager
pub struct IndexedDBManager {
    /// Database instances
    databases: Arc<RwLock<HashMap<String, Arc<RwLock<IndexedDatabase>>>>>,
    /// Database directory
    database_directory: PathBuf,
    /// Version manager
    version_manager: Arc<RwLock<DatabaseVersionManager>>,
    /// Transaction manager
    transaction_manager: Arc<RwLock<TransactionManager>>,
}

/// IndexedDB database
pub struct IndexedDatabase {
    /// Database name
    name: String,
    /// Database version
    version: u32,
    /// Object stores
    object_stores: HashMap<String, ObjectStore>,
    /// Database file path
    file_path: PathBuf,
    /// Database metadata
    metadata: DatabaseMetadata,
    /// Database state
    state: DatabaseState,
}

/// Object store
pub struct ObjectStore {
    /// Store name
    name: String,
    /// Store key path
    key_path: KeyPath,
    /// Store auto increment
    auto_increment: bool,
    /// Store indexes
    indexes: HashMap<String, Index>,
    /// Store data
    data: HashMap<String, StoreRecord>,
    /// Store metadata
    metadata: ObjectStoreMetadata,
}

/// Index
pub struct Index {
    /// Index name
    name: String,
    /// Index key path
    key_path: KeyPath,
    /// Index unique
    unique: bool,
    /// Index multi entry
    multi_entry: bool,
    /// Index data
    data: HashMap<String, Vec<String>>, // key -> record keys
}

/// Key path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyPath {
    /// No key path
    None,
    /// String key path
    String(String),
    /// Array key path
    Array(Vec<String>),
}

/// Store record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreRecord {
    /// Record key
    pub key: String,
    /// Record value
    pub value: serde_json::Value,
    /// Record created time
    pub created: u64,
    /// Record modified time
    pub modified: u64,
    /// Record size
    pub size: usize,
}

/// Database metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetadata {
    /// Database created time
    pub created: u64,
    /// Database last modified time
    pub last_modified: u64,
    /// Database description
    pub description: String,
    /// Database size
    pub size: usize,
}

/// Object store metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectStoreMetadata {
    /// Store created time
    pub created: u64,
    /// Store last modified time
    pub last_modified: u64,
    /// Store description
    pub description: String,
    /// Store record count
    pub record_count: usize,
    /// Store size
    pub size: usize,
}

/// Database state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DatabaseState {
    /// Database closed
    Closed,
    /// Database opening
    Opening,
    /// Database open
    Open,
    /// Database closing
    Closing,
    /// Database error
    Error,
}

/// Database version manager
pub struct DatabaseVersionManager {
    /// Database versions
    versions: HashMap<String, u32>,
    /// Version change callbacks
    version_change_callbacks: HashMap<String, Vec<Box<dyn Fn(u32, u32) + Send + Sync>>>,
}

/// Transaction manager
pub struct TransactionManager {
    /// Active transactions
    transactions: HashMap<String, Transaction>,
    /// Transaction counter
    transaction_counter: u64,
}

/// Transaction
pub struct Transaction {
    /// Transaction ID
    id: String,
    /// Transaction mode
    mode: TransactionMode,
    /// Transaction object stores
    object_stores: Vec<String>,
    /// Transaction state
    state: TransactionState,
    /// Transaction created time
    created: u64,
    /// Transaction timeout
    timeout: u64,
}

/// Transaction mode
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TransactionMode {
    /// Read only transaction
    ReadOnly,
    /// Read write transaction
    ReadWrite,
    /// Version change transaction
    VersionChange,
}

/// Transaction state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TransactionState {
    /// Transaction active
    Active,
    /// Transaction committed
    Committed,
    /// Transaction aborted
    Aborted,
    /// Transaction error
    Error,
}

/// IndexedDB request
pub struct IndexedDBRequest {
    /// Request ID
    id: String,
    /// Request type
    request_type: RequestType,
    /// Request data
    data: RequestData,
    /// Request state
    state: RequestState,
    /// Request result
    result: Option<RequestResult>,
    /// Request error
    error: Option<Error>,
}

/// Request type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RequestType {
    /// Open database request
    OpenDatabase,
    /// Create object store request
    CreateObjectStore,
    /// Delete object store request
    DeleteObjectStore,
    /// Add record request
    AddRecord,
    /// Put record request
    PutRecord,
    /// Get record request
    GetRecord,
    /// Delete record request
    DeleteRecord,
    /// Clear store request
    ClearStore,
    /// Count records request
    CountRecords,
    /// Create index request
    CreateIndex,
    /// Delete index request
    DeleteIndex,
}

/// Request data
#[derive(Debug, Clone)]
pub enum RequestData {
    /// No data
    None,
    /// String data
    String(String),
    /// JSON data
    Json(serde_json::Value),
    /// Key value data
    KeyValue { key: String, value: serde_json::Value },
    /// Object store data
    ObjectStore { name: String, key_path: KeyPath, auto_increment: bool },
    /// Index data
    Index { name: String, key_path: KeyPath, unique: bool, multi_entry: bool },
}

/// Request state
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RequestState {
    /// Request pending
    Pending,
    /// Request processing
    Processing,
    /// Request completed
    Completed,
    /// Request error
    Error,
}

/// Request result
#[derive(Debug, Clone)]
pub enum RequestResult {
    /// No result
    None,
    /// String result
    String(String),
    /// JSON result
    Json(serde_json::Value),
    /// Number result
    Number(f64),
    /// Boolean result
    Boolean(bool),
    /// Array result
    Array(Vec<serde_json::Value>),
}

/// IndexedDB cursor
pub struct IndexedDBCursor {
    /// Cursor ID
    id: String,
    /// Cursor source
    source: CursorSource,
    /// Cursor direction
    direction: CursorDirection,
    /// Cursor key
    key: Option<String>,
    /// Cursor value
    value: Option<serde_json::Value>,
    /// Cursor primary key
    primary_key: Option<String>,
    /// Cursor position
    position: usize,
    /// Cursor data
    data: Vec<StoreRecord>,
}

/// Cursor source
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CursorSource {
    /// Object store cursor
    ObjectStore,
    /// Index cursor
    Index,
}

/// Cursor direction
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CursorDirection {
    /// Next cursor
    Next,
    /// Previous cursor
    Prev,
    /// Next unique cursor
    NextUnique,
    /// Previous unique cursor
    PrevUnique,
}

impl IndexedDBManager {
    /// Create new IndexedDB manager
    pub fn new(database_directory: PathBuf) -> Result<Self> {
        // Create database directory if it doesn't exist
        fs::create_dir_all(&database_directory)
            .map_err(|e| Error::storage(format!("Failed to create database directory: {}", e)))?;
        
        let version_manager = Arc::new(RwLock::new(DatabaseVersionManager::new()));
        let transaction_manager = Arc::new(RwLock::new(TransactionManager::new()));
        
        Ok(Self {
            databases: Arc::new(RwLock::new(HashMap::new())),
            database_directory,
            version_manager,
            transaction_manager,
        })
    }

    /// Open database
    pub async fn open_database(&self, name: &str, version: Option<u32>) -> Result<Arc<RwLock<IndexedDatabase>>> {
        let mut databases = self.databases.write();
        
        if let Some(database) = databases.get(name) {
            let mut db_guard = database.write();
            
            // Check if version upgrade is needed
            if let Some(new_version) = version {
                if new_version > db_guard.version {
                    // Trigger version change
                    self.trigger_version_change(name, db_guard.version, new_version).await?;
                    db_guard.version = new_version;
                    db_guard.save_metadata()?;
                }
            }
            
            return Ok(database.clone());
        }
        
        // Create new database
        let database = Arc::new(RwLock::new(IndexedDatabase::new(
            name,
            version.unwrap_or(1),
            &self.database_directory,
        )?));
        
        databases.insert(name.to_string(), database.clone());
        
        Ok(database)
    }

    /// Delete database
    pub async fn delete_database(&self, name: &str) -> Result<()> {
        let mut databases = self.databases.write();
        
        if let Some(database) = databases.remove(name) {
            let db_guard = database.read();
            db_guard.delete()?;
        }
        
        Ok(())
    }

    /// Create object store
    pub async fn create_object_store(
        &self,
        database_name: &str,
        store_name: &str,
        key_path: KeyPath,
        auto_increment: bool,
    ) -> Result<()> {
        let database = self.get_database(database_name).await?;
        let mut db_guard = database.write();
        
        db_guard.create_object_store(store_name, key_path, auto_increment)?;
        
        Ok(())
    }

    /// Delete object store
    pub async fn delete_object_store(&self, database_name: &str, store_name: &str) -> Result<()> {
        let database = self.get_database(database_name).await?;
        let mut db_guard = database.write();
        
        db_guard.delete_object_store(store_name)?;
        
        Ok(())
    }

    /// Add record
    pub async fn add_record(
        &self,
        database_name: &str,
        store_name: &str,
        key: &str,
        value: serde_json::Value,
    ) -> Result<()> {
        let database = self.get_database(database_name).await?;
        let mut db_guard = database.write();
        
        db_guard.add_record(store_name, key, value)?;
        
        Ok(())
    }

    /// Put record
    pub async fn put_record(
        &self,
        database_name: &str,
        store_name: &str,
        key: &str,
        value: serde_json::Value,
    ) -> Result<()> {
        let database = self.get_database(database_name).await?;
        let mut db_guard = database.write();
        
        db_guard.put_record(store_name, key, value)?;
        
        Ok(())
    }

    /// Get record
    pub async fn get_record(
        &self,
        database_name: &str,
        store_name: &str,
        key: &str,
    ) -> Result<Option<serde_json::Value>> {
        let database = self.get_database(database_name).await?;
        let db_guard = database.read();
        
        Ok(db_guard.get_record(store_name, key))
    }

    /// Delete record
    pub async fn delete_record(&self, database_name: &str, store_name: &str, key: &str) -> Result<()> {
        let database = self.get_database(database_name).await?;
        let mut db_guard = database.write();
        
        db_guard.delete_record(store_name, key)?;
        
        Ok(())
    }

    /// Clear store
    pub async fn clear_store(&self, database_name: &str, store_name: &str) -> Result<()> {
        let database = self.get_database(database_name).await?;
        let mut db_guard = database.write();
        
        db_guard.clear_store(store_name)?;
        
        Ok(())
    }

    /// Count records
    pub async fn count_records(&self, database_name: &str, store_name: &str) -> Result<usize> {
        let database = self.get_database(database_name).await?;
        let db_guard = database.read();
        
        Ok(db_guard.count_records(store_name))
    }

    /// Create index
    pub async fn create_index(
        &self,
        database_name: &str,
        store_name: &str,
        index_name: &str,
        key_path: KeyPath,
        unique: bool,
        multi_entry: bool,
    ) -> Result<()> {
        let database = self.get_database(database_name).await?;
        let mut db_guard = database.write();
        
        db_guard.create_index(store_name, index_name, key_path, unique, multi_entry)?;
        
        Ok(())
    }

    /// Delete index
    pub async fn delete_index(&self, database_name: &str, store_name: &str, index_name: &str) -> Result<()> {
        let database = self.get_database(database_name).await?;
        let mut db_guard = database.write();
        
        db_guard.delete_index(store_name, index_name)?;
        
        Ok(())
    }

    /// Get database
    async fn get_database(&self, name: &str) -> Result<Arc<RwLock<IndexedDatabase>>> {
        let databases = self.databases.read();
        
        databases
            .get(name)
            .cloned()
            .ok_or_else(|| Error::storage(format!("Database '{}' not found", name)))
    }

    /// Trigger version change
    async fn trigger_version_change(&self, database_name: &str, old_version: u32, new_version: u32) -> Result<()> {
        let version_manager = self.version_manager.read();
        
        if let Some(callbacks) = version_manager.version_change_callbacks.get(database_name) {
            for callback in callbacks {
                callback(old_version, new_version);
            }
        }
        
        Ok(())
    }

    /// Add version change callback
    pub async fn add_version_change_callback<F>(&self, database_name: &str, callback: F) -> Result<()>
    where
        F: Fn(u32, u32) + Send + Sync + 'static,
    {
        let mut version_manager = self.version_manager.write();
        
        version_manager
            .version_change_callbacks
            .entry(database_name.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
        
        Ok(())
    }

    /// Create transaction
    pub async fn create_transaction(
        &self,
        database_name: &str,
        object_stores: Vec<String>,
        mode: TransactionMode,
    ) -> Result<String> {
        let mut transaction_manager = self.transaction_manager.write();
        
        let transaction_id = Uuid::new_v4().to_string();
        let transaction = Transaction::new(transaction_id.clone(), mode, object_stores);
        
        transaction_manager.transactions.insert(transaction_id.clone(), transaction);
        
        Ok(transaction_id)
    }

    /// Commit transaction
    pub async fn commit_transaction(&self, transaction_id: &str) -> Result<()> {
        let mut transaction_manager = self.transaction_manager.write();
        
        if let Some(transaction) = transaction_manager.transactions.get_mut(transaction_id) {
            transaction.commit()?;
        }
        
        Ok(())
    }

    /// Abort transaction
    pub async fn abort_transaction(&self, transaction_id: &str) -> Result<()> {
        let mut transaction_manager = self.transaction_manager.write();
        
        if let Some(transaction) = transaction_manager.transactions.get_mut(transaction_id) {
            transaction.abort()?;
        }
        
        Ok(())
    }

    /// Get database list
    pub async fn get_database_list(&self) -> Result<Vec<String>> {
        let databases = self.databases.read();
        
        Ok(databases.keys().cloned().collect())
    }

    /// Get database statistics
    pub async fn get_database_stats(&self, database_name: &str) -> Result<DatabaseStats> {
        let database = self.get_database(database_name).await?;
        let db_guard = database.read();
        
        Ok(db_guard.get_stats())
    }
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    /// Database name
    pub name: String,
    /// Database version
    pub version: u32,
    /// Object store count
    pub object_store_count: usize,
    /// Total record count
    pub total_record_count: usize,
    /// Database size
    pub size: usize,
    /// Database state
    pub state: DatabaseState,
}

impl IndexedDatabase {
    /// Create new database
    pub fn new(name: &str, version: u32, database_directory: &Path) -> Result<Self> {
        let file_path = database_directory.join(format!("{}.json", name));
        
        let metadata = if file_path.exists() {
            Self::load_metadata(&file_path)?
        } else {
            DatabaseMetadata::new()
        };
        
        let object_stores = if file_path.exists() {
            Self::load_object_stores(&file_path)?
        } else {
            HashMap::new()
        };
        
        Ok(Self {
            name: name.to_string(),
            version,
            object_stores,
            file_path,
            metadata,
            state: DatabaseState::Open,
        })
    }

    /// Create object store
    pub fn create_object_store(&mut self, name: &str, key_path: KeyPath, auto_increment: bool) -> Result<()> {
        if self.object_stores.contains_key(name) {
            return Err(Error::storage(format!("Object store '{}' already exists", name)));
        }
        
        let object_store = ObjectStore::new(name, key_path, auto_increment);
        self.object_stores.insert(name.to_string(), object_store);
        
        self.save_metadata()?;
        
        Ok(())
    }

    /// Delete object store
    pub fn delete_object_store(&mut self, name: &str) -> Result<()> {
        if !self.object_stores.contains_key(name) {
            return Err(Error::storage(format!("Object store '{}' not found", name)));
        }
        
        self.object_stores.remove(name);
        
        self.save_metadata()?;
        
        Ok(())
    }

    /// Add record
    pub fn add_record(&mut self, store_name: &str, key: &str, value: serde_json::Value) -> Result<()> {
        let store = self.get_object_store_mut(store_name)?;
        
        if store.data.contains_key(key) {
            return Err(Error::storage("Key already exists".to_string()));
        }
        
        store.add_record(key, value)?;
        
        Ok(())
    }

    /// Put record
    pub fn put_record(&mut self, store_name: &str, key: &str, value: serde_json::Value) -> Result<()> {
        let store = self.get_object_store_mut(store_name)?;
        
        store.put_record(key, value)?;
        
        Ok(())
    }

    /// Get record
    pub fn get_record(&self, store_name: &str, key: &str) -> Option<serde_json::Value> {
        let store = self.get_object_store(store_name)?;
        
        store.get_record(key)
    }

    /// Delete record
    pub fn delete_record(&mut self, store_name: &str, key: &str) -> Result<()> {
        let store = self.get_object_store_mut(store_name)?;
        
        store.delete_record(key)?;
        
        Ok(())
    }

    /// Clear store
    pub fn clear_store(&mut self, store_name: &str) -> Result<()> {
        let store = self.get_object_store_mut(store_name)?;
        
        store.clear()?;
        
        Ok(())
    }

    /// Count records
    pub fn count_records(&self, store_name: &str) -> usize {
        let store = self.get_object_store(store_name)?;
        
        store.data.len()
    }

    /// Create index
    pub fn create_index(
        &mut self,
        store_name: &str,
        index_name: &str,
        key_path: KeyPath,
        unique: bool,
        multi_entry: bool,
    ) -> Result<()> {
        let store = self.get_object_store_mut(store_name)?;
        
        store.create_index(index_name, key_path, unique, multi_entry)?;
        
        Ok(())
    }

    /// Delete index
    pub fn delete_index(&mut self, store_name: &str, index_name: &str) -> Result<()> {
        let store = self.get_object_store_mut(store_name)?;
        
        store.delete_index(index_name)?;
        
        Ok(())
    }

    /// Get object store
    fn get_object_store(&self, name: &str) -> Result<&ObjectStore> {
        self.object_stores
            .get(name)
            .ok_or_else(|| Error::storage(format!("Object store '{}' not found", name)))
    }

    /// Get object store mutable
    fn get_object_store_mut(&mut self, name: &str) -> Result<&mut ObjectStore> {
        self.object_stores
            .get_mut(name)
            .ok_or_else(|| Error::storage(format!("Object store '{}' not found", name)))
    }

    /// Save metadata
    fn save_metadata(&self) -> Result<()> {
        let metadata = DatabaseMetadata {
            created: self.metadata.created,
            last_modified: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            description: self.metadata.description.clone(),
            size: self.calculate_size(),
        };
        
        let content = serde_json::to_string_pretty(&metadata)
            .map_err(|e| Error::storage(format!("Failed to serialize metadata: {}", e)))?;
        
        fs::write(&self.file_path, content)
            .map_err(|e| Error::storage(format!("Failed to write metadata file: {}", e)))
    }

    /// Load metadata
    fn load_metadata(file_path: &Path) -> Result<DatabaseMetadata> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| Error::storage(format!("Failed to read metadata file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| Error::storage(format!("Failed to parse metadata file: {}", e)))
    }

    /// Load object stores
    fn load_object_stores(file_path: &Path) -> Result<HashMap<String, ObjectStore>> {
        // This is a simplified implementation
        // In a real implementation, you would load object stores from the database file
        Ok(HashMap::new())
    }

    /// Calculate size
    fn calculate_size(&self) -> usize {
        self.object_stores.values().map(|store| store.metadata.size).sum()
    }

    /// Delete database
    fn delete(&self) -> Result<()> {
        if self.file_path.exists() {
            fs::remove_file(&self.file_path)
                .map_err(|e| Error::storage(format!("Failed to delete database file: {}", e)))?;
        }
        
        Ok(())
    }

    /// Get statistics
    fn get_stats(&self) -> DatabaseStats {
        let object_store_count = self.object_stores.len();
        let total_record_count = self.object_stores.values().map(|store| store.data.len()).sum();
        let size = self.calculate_size();
        
        DatabaseStats {
            name: self.name.clone(),
            version: self.version,
            object_store_count,
            total_record_count,
            size,
            state: self.state,
        }
    }
}

impl ObjectStore {
    /// Create new object store
    pub fn new(name: &str, key_path: KeyPath, auto_increment: bool) -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            name: name.to_string(),
            key_path,
            auto_increment,
            indexes: HashMap::new(),
            data: HashMap::new(),
            metadata: ObjectStoreMetadata {
                created: current_time,
                last_modified: current_time,
                description: format!("Object store '{}'", name),
                record_count: 0,
                size: 0,
            },
        }
    }

    /// Add record
    pub fn add_record(&mut self, key: &str, value: serde_json::Value) -> Result<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let record = StoreRecord {
            key: key.to_string(),
            value: value.clone(),
            created: current_time,
            modified: current_time,
            size: key.len() + serde_json::to_string(&value).unwrap().len(),
        };
        
        self.data.insert(key.to_string(), record);
        self.update_metadata();
        
        // Update indexes
        self.update_indexes(key, &value)?;
        
        Ok(())
    }

    /// Put record
    pub fn put_record(&mut self, key: &str, value: serde_json::Value) -> Result<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let record = StoreRecord {
            key: key.to_string(),
            value: value.clone(),
            created: current_time,
            modified: current_time,
            size: key.len() + serde_json::to_string(&value).unwrap().len(),
        };
        
        self.data.insert(key.to_string(), record);
        self.update_metadata();
        
        // Update indexes
        self.update_indexes(key, &value)?;
        
        Ok(())
    }

    /// Get record
    pub fn get_record(&self, key: &str) -> Option<serde_json::Value> {
        self.data.get(key).map(|record| record.value.clone())
    }

    /// Delete record
    pub fn delete_record(&mut self, key: &str) -> Result<()> {
        if let Some(record) = self.data.remove(key) {
            self.update_metadata();
            
            // Remove from indexes
            self.remove_from_indexes(key)?;
        }
        
        Ok(())
    }

    /// Clear store
    pub fn clear(&mut self) -> Result<()> {
        self.data.clear();
        self.indexes.clear();
        self.update_metadata();
        
        Ok(())
    }

    /// Create index
    pub fn create_index(
        &mut self,
        name: &str,
        key_path: KeyPath,
        unique: bool,
        multi_entry: bool,
    ) -> Result<()> {
        if self.indexes.contains_key(name) {
            return Err(Error::storage(format!("Index '{}' already exists", name)));
        }
        
        let index = Index::new(name, key_path, unique, multi_entry);
        self.indexes.insert(name.to_string(), index);
        
        // Build index from existing data
        self.build_index(name)?;
        
        Ok(())
    }

    /// Delete index
    pub fn delete_index(&mut self, name: &str) -> Result<()> {
        if !self.indexes.contains_key(name) {
            return Err(Error::storage(format!("Index '{}' not found", name)));
        }
        
        self.indexes.remove(name);
        
        Ok(())
    }

    /// Update metadata
    fn update_metadata(&mut self) {
        self.metadata.record_count = self.data.len();
        self.metadata.size = self.data.values().map(|record| record.size).sum();
        self.metadata.last_modified = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Update indexes
    fn update_indexes(&mut self, key: &str, value: &serde_json::Value) -> Result<()> {
        for index in self.indexes.values_mut() {
            index.add_record(key, value)?;
        }
        
        Ok(())
    }

    /// Remove from indexes
    fn remove_from_indexes(&mut self, key: &str) -> Result<()> {
        for index in self.indexes.values_mut() {
            index.remove_record(key);
        }
        
        Ok(())
    }

    /// Build index
    fn build_index(&mut self, index_name: &str) -> Result<()> {
        if let Some(index) = self.indexes.get_mut(index_name) {
            for (key, record) in &self.data {
                index.add_record(key, &record.value)?;
            }
        }
        
        Ok(())
    }
}

impl Index {
    /// Create new index
    pub fn new(name: &str, key_path: KeyPath, unique: bool, multi_entry: bool) -> Self {
        Self {
            name: name.to_string(),
            key_path,
            unique,
            multi_entry,
            data: HashMap::new(),
        }
    }

    /// Add record to index
    pub fn add_record(&mut self, key: &str, value: &serde_json::Value) -> Result<()> {
        let index_key = self.extract_key(value)?;
        
        if self.unique {
            if self.data.contains_key(&index_key) {
                return Err(Error::storage("Unique constraint violation".to_string()));
            }
            self.data.insert(index_key, vec![key.to_string()]);
        } else {
            self.data
                .entry(index_key)
                .or_insert_with(Vec::new)
                .push(key.to_string());
        }
        
        Ok(())
    }

    /// Remove record from index
    pub fn remove_record(&mut self, key: &str) {
        // This is a simplified implementation
        // In a real implementation, you would find and remove the key from the index
        for values in self.data.values_mut() {
            values.retain(|k| k != key);
        }
        
        // Remove empty entries
        self.data.retain(|_, values| !values.is_empty());
    }

    /// Extract key from value
    fn extract_key(&self, value: &serde_json::Value) -> Result<String> {
        match &self.key_path {
            KeyPath::None => Ok(value.to_string()),
            KeyPath::String(path) => {
                if let Some(key_value) = value.get(path) {
                    Ok(key_value.to_string())
                } else {
                    Err(Error::storage(format!("Key path '{}' not found", path)))
                }
            }
            KeyPath::Array(paths) => {
                let mut key_parts = Vec::new();
                for path in paths {
                    if let Some(key_value) = value.get(path) {
                        key_parts.push(key_value.to_string());
                    } else {
                        return Err(Error::storage(format!("Key path '{}' not found", path)));
                    }
                }
                Ok(key_parts.join(":"))
            }
        }
    }
}

impl DatabaseMetadata {
    /// Create new database metadata
    pub fn new() -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            created: current_time,
            last_modified: current_time,
            description: "IndexedDB database".to_string(),
            size: 0,
        }
    }
}

impl DatabaseVersionManager {
    /// Create new database version manager
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
            version_change_callbacks: HashMap::new(),
        }
    }
}

impl TransactionManager {
    /// Create new transaction manager
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            transaction_counter: 0,
        }
    }
}

impl Transaction {
    /// Create new transaction
    pub fn new(id: String, mode: TransactionMode, object_stores: Vec<String>) -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            id,
            mode,
            object_stores,
            state: TransactionState::Active,
            created: current_time,
            timeout: 5000, // 5 seconds
        }
    }

    /// Commit transaction
    pub fn commit(&mut self) -> Result<()> {
        if self.state != TransactionState::Active {
            return Err(Error::storage("Transaction is not active".to_string()));
        }
        
        self.state = TransactionState::Committed;
        
        Ok(())
    }

    /// Abort transaction
    pub fn abort(&mut self) -> Result<()> {
        if self.state != TransactionState::Active {
            return Err(Error::storage("Transaction is not active".to_string()));
        }
        
        self.state = TransactionState::Aborted;
        
        Ok(())
    }
}
