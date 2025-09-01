//! Storage module for Matte Browser
//! 
//! This module provides Web Storage (localStorage, sessionStorage) and IndexedDB
//! implementations for the browser.

pub mod error;
pub mod web_storage;
pub mod indexed_db;

pub use error::{Error, Result};
pub use web_storage::{
    WebStorageManager, LocalStorage, SessionStorage, StorageItem,
    StorageQuotaManager, StoragePartitioningManager, StoragePartition,
    PartitionPolicy, PartitionPolicyType, PartitionRule,
    StorageEvent, StorageEventType, StorageStats,
};
pub use indexed_db::{
    IndexedDBManager, IndexedDatabase, ObjectStore, Index,
    KeyPath, StoreRecord, DatabaseMetadata, ObjectStoreMetadata,
    DatabaseState, DatabaseVersionManager, TransactionManager,
    Transaction, TransactionMode, TransactionState,
    IndexedDBRequest, RequestType, RequestData, RequestState, RequestResult,
    IndexedDBCursor, CursorSource, CursorDirection,
    DatabaseStats,
};

/// Storage manager that combines Web Storage and IndexedDB
pub struct StorageManager {
    /// Web Storage manager
    web_storage: Arc<RwLock<WebStorageManager>>,
    /// IndexedDB manager
    indexed_db: Arc<RwLock<IndexedDBManager>>,
    /// Storage directory
    storage_directory: PathBuf,
}

use std::sync::Arc;
use parking_lot::RwLock;
use std::path::PathBuf;

impl StorageManager {
    /// Create new storage manager
    pub async fn new(storage_directory: PathBuf) -> Result<Self> {
        let web_storage = Arc::new(RwLock::new(WebStorageManager::new(storage_directory.clone())?));
        let indexed_db = Arc::new(RwLock::new(IndexedDBManager::new(storage_directory.join("indexeddb"))?));
        
        Ok(Self {
            web_storage,
            indexed_db,
            storage_directory,
        })
    }

    /// Get Web Storage manager
    pub fn web_storage(&self) -> Arc<RwLock<WebStorageManager>> {
        self.web_storage.clone()
    }

    /// Get IndexedDB manager
    pub fn indexed_db(&self) -> Arc<RwLock<IndexedDBManager>> {
        self.indexed_db.clone()
    }

    /// Get storage directory
    pub fn storage_directory(&self) -> &PathBuf {
        &self.storage_directory
    }

    /// Get combined storage statistics
    pub async fn get_storage_stats(&self) -> Result<CombinedStorageStats> {
        let web_storage_stats = {
            let web_storage = self.web_storage.read();
            web_storage.get_storage_stats().await?
        };
        
        let indexed_db_stats = {
            let indexed_db = self.indexed_db.read();
            let databases = indexed_db.get_database_list().await?;
            let mut total_size = 0;
            let mut total_databases = 0;
            
            for db_name in databases {
                if let Ok(stats) = indexed_db.get_database_stats(&db_name).await {
                    total_size += stats.size;
                    total_databases += 1;
                }
            }
            
            IndexedDBStats {
                database_count: total_databases,
                total_size,
            }
        };
        
        Ok(CombinedStorageStats {
            web_storage: web_storage_stats,
            indexed_db: indexed_db_stats,
            total_size: web_storage_stats.total_size + indexed_db_stats.total_size,
        })
    }

    /// Clean up expired data
    pub async fn cleanup_expired_data(&self) -> Result<()> {
        // Clean up expired session storage
        {
            let web_storage = self.web_storage.read();
            web_storage.cleanup_expired_sessions().await?;
        }
        
        // Clean up expired transactions
        {
            let indexed_db = self.indexed_db.read();
            // This would clean up expired transactions in a real implementation
        }
        
        Ok(())
    }

    /// Shutdown storage manager
    pub async fn shutdown(&self) -> Result<()> {
        // Save any pending data
        // Close any open connections
        // Clean up resources
        
        Ok(())
    }
}

/// Combined storage statistics
#[derive(Debug, Clone)]
pub struct CombinedStorageStats {
    /// Web Storage statistics
    pub web_storage: StorageStats,
    /// IndexedDB statistics
    pub indexed_db: IndexedDBStats,
    /// Total storage size
    pub total_size: usize,
}

/// IndexedDB statistics
#[derive(Debug, Clone)]
pub struct IndexedDBStats {
    /// Number of databases
    pub database_count: usize,
    /// Total size in bytes
    pub total_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_storage_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let storage_manager = StorageManager::new(temp_dir.path().to_path_buf()).await;
        assert!(storage_manager.is_ok());
    }

    #[tokio::test]
    async fn test_web_storage_operations() {
        let temp_dir = TempDir::new().unwrap();
        let storage_manager = StorageManager::new(temp_dir.path().to_path_buf()).await.unwrap();
        let web_storage = storage_manager.web_storage();
        
        // Test localStorage operations
        let origin = "https://example.com";
        let key = "test_key";
        let value = "test_value";
        
        // Set item
        let result = web_storage.read().set_local_storage_item(origin, key, value).await;
        assert!(result.is_ok());
        
        // Get item
        let result = web_storage.read().get_local_storage_item(origin, key).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(value.to_string()));
        
        // Remove item
        let result = web_storage.read().remove_local_storage_item(origin, key).await;
        assert!(result.is_ok());
        
        // Verify item is removed
        let result = web_storage.read().get_local_storage_item(origin, key).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test]
    async fn test_indexed_db_operations() {
        let temp_dir = TempDir::new().unwrap();
        let storage_manager = StorageManager::new(temp_dir.path().to_path_buf()).await.unwrap();
        let indexed_db = storage_manager.indexed_db();
        
        let db_name = "test_db";
        let store_name = "test_store";
        
        // Open database
        let db = indexed_db.read().open_database(db_name, Some(1)).await;
        assert!(db.is_ok());
        
        // Create object store
        let result = indexed_db.read().create_object_store(
            db_name,
            store_name,
            KeyPath::String("id".to_string()),
            false,
        ).await;
        assert!(result.is_ok());
        
        // Add record
        let key = "test_key";
        let value = serde_json::json!({"id": "test_key", "name": "Test Record"});
        let result = indexed_db.read().add_record(db_name, store_name, key, value.clone()).await;
        assert!(result.is_ok());
        
        // Get record
        let result = indexed_db.read().get_record(db_name, store_name, key).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(value));
        
        // Delete record
        let result = indexed_db.read().delete_record(db_name, store_name, key).await;
        assert!(result.is_ok());
        
        // Verify record is deleted
        let result = indexed_db.read().get_record(db_name, store_name, key).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test]
    async fn test_storage_stats() {
        let temp_dir = TempDir::new().unwrap();
        let storage_manager = StorageManager::new(temp_dir.path().to_path_buf()).await.unwrap();
        
        let stats = storage_manager.get_storage_stats().await;
        assert!(stats.is_ok());
        
        let stats = stats.unwrap();
        assert_eq!(stats.web_storage.origin_count, 0);
        assert_eq!(stats.web_storage.item_count, 0);
        assert_eq!(stats.indexed_db.database_count, 0);
    }
}
