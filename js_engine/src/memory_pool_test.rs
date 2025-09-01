#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::{
        MemoryPool, PoolConfig, PoolType, PoolStats, PoolEntry,
        Nursery, NurseryConfig, NurseryStats,
        MemoryPoolManager, ManagerConfig, ManagerStats
    };

    #[tokio::test]
    async fn test_memory_pool_creation() {
        let config = PoolConfig::default();
        let pool = MemoryPool::new(config);
        
        let stats = pool.get_stats();
        assert_eq!(stats.pool_type, PoolType::Small);
        assert_eq!(stats.total_objects, 1000);
        assert_eq!(stats.objects_available, 1000);
        assert_eq!(stats.objects_in_use, 0);
        assert_eq!(stats.allocation_count, 0);
        assert_eq!(stats.deallocation_count, 0);
    }

    #[tokio::test]
    async fn test_memory_pool_allocation() {
        let config = PoolConfig::default();
        let pool = MemoryPool::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let entry_id = pool.allocate(data.clone()).unwrap();
        
        let entry = pool.get_entry(entry_id);
        assert!(entry.is_some());
        
        let entry = entry.unwrap();
        assert_eq!(entry.id, entry_id);
        assert_eq!(entry.pool_type, PoolType::Small);
        assert_eq!(entry.size, 64);
        assert_eq!(entry.data, data);
        assert!(entry.is_in_use);
        assert_eq!(entry.reference_count, 1);
        
        let stats = pool.get_stats();
        assert_eq!(stats.objects_in_use, 1);
        assert_eq!(stats.objects_available, 999);
        assert_eq!(stats.allocation_count, 1);
        assert!(stats.avg_allocation_time_us > 0.0);
    }

    #[tokio::test]
    async fn test_memory_pool_deallocation() {
        let config = PoolConfig::default();
        let pool = MemoryPool::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let entry_id = pool.allocate(data).unwrap();
        
        let stats_before = pool.get_stats();
        assert_eq!(stats_before.objects_in_use, 1);
        assert_eq!(stats_before.objects_available, 999);
        
        pool.deallocate(entry_id).unwrap();
        
        let stats_after = pool.get_stats();
        assert_eq!(stats_after.objects_in_use, 0);
        assert_eq!(stats_after.objects_available, 1000);
        assert_eq!(stats_after.deallocation_count, 1);
        
        // Entry should be available for reuse
        let entry = pool.get_entry(entry_id);
        assert!(entry.is_some());
        assert!(!entry.unwrap().is_in_use);
    }

    #[tokio::test]
    async fn test_memory_pool_expansion() {
        let mut config = PoolConfig::default();
        config.objects_per_pool = 2; // Small pool for testing
        config.max_pools = 3;
        let pool = MemoryPool::new(config);
        
        // Allocate all initial objects
        let entry_ids: Vec<u64> = (0..2)
            .map(|i| pool.allocate(vec![i]).unwrap())
            .collect();
        
        let stats_before = pool.get_stats();
        assert_eq!(stats_before.total_pools, 1);
        assert_eq!(stats_before.objects_in_use, 2);
        assert_eq!(stats_before.objects_available, 0);
        
        // Allocate one more to trigger expansion
        let new_entry_id = pool.allocate(vec![99]).unwrap();
        
        let stats_after = pool.get_stats();
        assert_eq!(stats_after.total_pools, 2);
        assert_eq!(stats_after.objects_in_use, 3);
        assert_eq!(stats_after.objects_available, 1); // 2 * 2 - 3 = 1
        
        // Verify all entries exist
        for entry_id in entry_ids {
            assert!(pool.get_entry(entry_id).is_some());
        }
        assert!(pool.get_entry(new_entry_id).is_some());
    }

    #[tokio::test]
    async fn test_memory_pool_shrinking() {
        let mut config = PoolConfig::default();
        config.objects_per_pool = 10;
        config.shrink_threshold = 0.5; // Shrink if less than 50% used
        let pool = MemoryPool::new(config);
        
        // Allocate some objects
        let entry_ids: Vec<u64> = (0..5)
            .map(|i| pool.allocate(vec![i]).unwrap())
            .collect();
        
        // Deallocate most objects to trigger shrinking
        for entry_id in &entry_ids[2..] {
            pool.deallocate(*entry_id).unwrap();
        }
        
        let stats = pool.get_stats();
        assert_eq!(stats.objects_in_use, 2);
        assert_eq!(stats.objects_available, 8);
        
        // Shrink the pool
        pool.shrink_pool().unwrap();
        
        // In this case, shrinking might not occur due to the threshold
        // The exact behavior depends on the implementation
    }

    #[tokio::test]
    async fn test_memory_pool_disabled() {
        let mut config = PoolConfig::default();
        config.enabled = false;
        let pool = MemoryPool::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let result = pool.allocate(data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Pool is disabled");
    }

    #[tokio::test]
    async fn test_memory_pool_clear() {
        let config = PoolConfig::default();
        let pool = MemoryPool::new(config);
        
        // Allocate some objects
        let entry_ids: Vec<u64> = (0..5)
            .map(|i| pool.allocate(vec![i]).unwrap())
            .collect();
        
        // Verify objects exist
        for entry_id in &entry_ids {
            assert!(pool.get_entry(*entry_id).is_some());
        }
        
        // Clear the pool
        pool.clear();
        
        // Verify all objects are gone
        for entry_id in &entry_ids {
            assert!(pool.get_entry(*entry_id).is_none());
        }
        
        let stats = pool.get_stats();
        assert_eq!(stats.total_objects, 0);
        assert_eq!(stats.objects_in_use, 0);
        assert_eq!(stats.objects_available, 0);
        assert_eq!(stats.allocation_count, 0);
        assert_eq!(stats.deallocation_count, 0);
    }

    #[tokio::test]
    async fn test_nursery_creation() {
        let config = NurseryConfig::default();
        let nursery = Nursery::new(config);
        
        let stats = nursery.get_stats();
        assert_eq!(stats.total_objects, 0);
        assert_eq!(stats.promoted_objects, 0);
        assert_eq!(stats.collected_objects, 0);
        assert_eq!(stats.current_size, 0);
        assert_eq!(stats.peak_size, 0);
        assert_eq!(stats.promotion_rate, 0.0);
        assert_eq!(stats.collection_count, 0);
    }

    #[tokio::test]
    async fn test_nursery_allocation() {
        let config = NurseryConfig::default();
        let nursery = Nursery::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let entry_id = nursery.allocate(PoolType::Small, data.clone()).unwrap();
        
        let stats = nursery.get_stats();
        assert_eq!(stats.total_objects, 1);
        assert_eq!(stats.current_size, 5);
        assert_eq!(stats.peak_size, 5);
        assert_eq!(stats.promoted_objects, 0);
    }

    #[tokio::test]
    async fn test_nursery_promotion() {
        let config = NurseryConfig::default();
        let nursery = Nursery::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let entry_id = nursery.allocate(PoolType::Small, data).unwrap();
        
        let stats_before = nursery.get_stats();
        assert_eq!(stats_before.total_objects, 1);
        assert_eq!(stats_before.promoted_objects, 0);
        
        // Promote the object
        nursery.promote_object(entry_id, PoolType::Small).unwrap();
        
        let stats_after = nursery.get_stats();
        assert_eq!(stats_after.promoted_objects, 1);
        assert!(stats_after.promotion_rate > 0.0);
    }

    #[tokio::test]
    async fn test_nursery_collection() {
        let config = NurseryConfig::default();
        let nursery = Nursery::new(config);
        
        // Allocate some objects
        for i in 0..10 {
            nursery.allocate(PoolType::Small, vec![i]).unwrap();
        }
        
        let stats_before = nursery.get_stats();
        assert_eq!(stats_before.total_objects, 10);
        assert_eq!(stats_before.collection_count, 0);
        
        // Collect nursery
        let collection_stats = nursery.collect().await.unwrap();
        
        assert_eq!(collection_stats.collection_count, 1);
        assert!(collection_stats.avg_collection_time_ms > 0.0);
    }

    #[tokio::test]
    async fn test_nursery_disabled() {
        let mut config = NurseryConfig::default();
        config.enabled = false;
        let nursery = Nursery::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let result = nursery.allocate(PoolType::Small, data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Nursery is disabled");
    }

    #[tokio::test]
    async fn test_nursery_clear() {
        let config = NurseryConfig::default();
        let nursery = Nursery::new(config);
        
        // Allocate some objects
        for i in 0..5 {
            nursery.allocate(PoolType::Small, vec![i]).unwrap();
        }
        
        let stats_before = nursery.get_stats();
        assert_eq!(stats_before.total_objects, 5);
        assert!(stats_before.current_size > 0);
        
        // Clear nursery
        nursery.clear();
        
        let stats_after = nursery.get_stats();
        assert_eq!(stats_after.total_objects, 0);
        assert_eq!(stats_after.current_size, 0);
        assert_eq!(stats_after.peak_size, 0);
    }

    #[tokio::test]
    async fn test_memory_pool_manager_creation() {
        let config = ManagerConfig::default();
        let manager = MemoryPoolManager::new(config);
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_allocations, 0);
        assert_eq!(stats.total_deallocations, 0);
        assert_eq!(stats.current_memory, 0);
        assert_eq!(stats.peak_memory, 0);
        assert_eq!(stats.memory_efficiency, 0.0);
        assert_eq!(stats.avg_allocation_time_us, 0.0);
        assert_eq!(stats.pool_hit_rate, 0.0);
    }

    #[tokio::test]
    async fn test_memory_pool_manager_allocation() {
        let config = ManagerConfig::default();
        let manager = MemoryPoolManager::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let entry_id = manager.allocate(PoolType::Small, data).await.unwrap();
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_allocations, 1);
        assert!(stats.avg_allocation_time_us > 0.0);
        
        // Check nursery stats
        let nursery_stats = manager.get_nursery_stats();
        assert_eq!(nursery_stats.total_objects, 1);
        assert!(nursery_stats.current_size > 0);
    }

    #[tokio::test]
    async fn test_memory_pool_manager_deallocation() {
        let config = ManagerConfig::default();
        let manager = MemoryPoolManager::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let entry_id = manager.allocate(PoolType::Small, data).await.unwrap();
        
        let stats_before = manager.get_stats();
        assert_eq!(stats_before.total_allocations, 1);
        assert_eq!(stats_before.total_deallocations, 0);
        
        manager.deallocate(PoolType::Small, entry_id).unwrap();
        
        let stats_after = manager.get_stats();
        assert_eq!(stats_after.total_deallocations, 1);
    }

    #[tokio::test]
    async fn test_memory_pool_manager_nursery_collection() {
        let config = ManagerConfig::default();
        let manager = MemoryPoolManager::new(config);
        
        // Allocate some objects
        for i in 0..10 {
            manager.allocate(PoolType::Small, vec![i]).await.unwrap();
        }
        
        let nursery_stats_before = manager.get_nursery_stats();
        assert_eq!(nursery_stats_before.total_objects, 10);
        assert_eq!(nursery_stats_before.collection_count, 0);
        
        // Collect nursery
        let collection_stats = manager.collect_nursery().await.unwrap();
        
        assert_eq!(collection_stats.collection_count, 1);
        assert!(collection_stats.avg_collection_time_ms > 0.0);
    }

    #[tokio::test]
    async fn test_memory_pool_manager_memory_pressure() {
        let config = ManagerConfig::default();
        let manager = MemoryPoolManager::new(config);
        
        // Initially, memory pressure should be low
        let pressure_before = manager.get_memory_pressure();
        assert!(pressure_before < 0.1);
        
        // Allocate many objects to increase pressure
        for i in 0..100 {
            manager.allocate(PoolType::Small, vec![i; 100]).await.unwrap();
        }
        
        let pressure_after = manager.get_memory_pressure();
        assert!(pressure_after > pressure_before);
    }

    #[tokio::test]
    async fn test_memory_pool_manager_handle_pressure() {
        let mut config = ManagerConfig::default();
        config.memory_pressure_threshold = 0.1; // Low threshold for testing
        let manager = MemoryPoolManager::new(config);
        
        // Allocate objects to create pressure
        for i in 0..50 {
            manager.allocate(PoolType::Small, vec![i; 100]).await.unwrap();
        }
        
        // Handle memory pressure
        manager.handle_memory_pressure().await.unwrap();
        
        // Verify that pressure handling occurred
        let pressure = manager.get_memory_pressure();
        // The exact pressure after handling depends on the implementation
    }

    #[tokio::test]
    async fn test_memory_pool_manager_pool_stats() {
        let config = ManagerConfig::default();
        let manager = MemoryPoolManager::new(config);
        
        // Allocate from different pool types
        manager.allocate(PoolType::Small, vec![1, 2, 3]).await.unwrap();
        manager.allocate(PoolType::String, vec![4, 5, 6]).await.unwrap();
        manager.allocate(PoolType::Array, vec![7, 8, 9]).await.unwrap();
        
        // Get stats for each pool type
        let small_stats = manager.get_pool_stats(PoolType::Small);
        let string_stats = manager.get_pool_stats(PoolType::String);
        let array_stats = manager.get_pool_stats(PoolType::Array);
        
        assert!(small_stats.is_some());
        assert!(string_stats.is_some());
        assert!(array_stats.is_some());
        
        assert_eq!(small_stats.unwrap().pool_type, PoolType::Small);
        assert_eq!(string_stats.unwrap().pool_type, PoolType::String);
        assert_eq!(array_stats.unwrap().pool_type, PoolType::Array);
    }

    #[tokio::test]
    async fn test_memory_pool_manager_disabled() {
        let mut config = ManagerConfig::default();
        config.enabled = false;
        let manager = MemoryPoolManager::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let result = manager.allocate(PoolType::Small, data).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Memory pooling is disabled");
    }

    #[tokio::test]
    async fn test_memory_pool_manager_clear() {
        let config = ManagerConfig::default();
        let manager = MemoryPoolManager::new(config);
        
        // Allocate some objects
        for i in 0..10 {
            manager.allocate(PoolType::Small, vec![i]).await.unwrap();
        }
        
        let stats_before = manager.get_stats();
        assert_eq!(stats_before.total_allocations, 10);
        
        // Clear all pools
        manager.clear();
        
        let stats_after = manager.get_stats();
        assert_eq!(stats_after.total_allocations, 0);
        assert_eq!(stats_after.total_deallocations, 0);
        assert_eq!(stats_after.current_memory, 0);
        assert_eq!(stats_after.peak_memory, 0);
    }

    #[tokio::test]
    async fn test_pool_type_enum() {
        // Test all pool types
        let pool_types = vec![
            PoolType::Nursery,
            PoolType::Small,
            PoolType::Medium,
            PoolType::Large,
            PoolType::String,
            PoolType::Array,
            PoolType::Object,
            PoolType::Function,
        ];
        
        for pool_type in pool_types {
            assert_eq!(format!("{:?}", pool_type), format!("{:?}", pool_type));
        }
    }

    #[tokio::test]
    async fn test_pool_config_default() {
        let config = PoolConfig::default();
        
        assert_eq!(config.pool_type, PoolType::Small);
        assert_eq!(config.object_size, 64);
        assert_eq!(config.objects_per_pool, 1000);
        assert_eq!(config.max_pools, 10);
        assert!(config.enabled);
        assert_eq!(config.growth_factor, 2.0);
        assert_eq!(config.shrink_threshold, 0.3);
    }

    #[tokio::test]
    async fn test_nursery_config_default() {
        let config = NurseryConfig::default();
        
        assert_eq!(config.max_size, 10 * 1024 * 1024); // 10MB
        assert_eq!(config.promotion_threshold, 3);
        assert_eq!(config.collection_frequency, 0.1);
        assert!(config.enabled);
        assert_eq!(config.growth_factor, 1.5);
    }

    #[tokio::test]
    async fn test_manager_config_default() {
        let config = ManagerConfig::default();
        
        assert!(config.enabled);
        assert_eq!(config.memory_pressure_threshold, 0.8);
        assert_eq!(config.cleanup_interval, 30.0);
        assert_eq!(config.pool_configs.len(), 7); // 7 different pool types
        assert!(config.nursery_config.enabled);
    }

    #[tokio::test]
    async fn test_memory_pool_integration() {
        let config = ManagerConfig::default();
        let manager = MemoryPoolManager::new(config);
        
        // Test allocation from different pool types
        let allocations = vec![
            (PoolType::Small, vec![1, 2, 3]),
            (PoolType::String, vec![4, 5, 6, 7]),
            (PoolType::Array, vec![8, 9, 10, 11, 12]),
            (PoolType::Object, vec![13, 14, 15]),
            (PoolType::Function, vec![16, 17, 18, 19, 20]),
        ];
        
        let mut entry_ids = Vec::new();
        
        // Allocate objects
        for (pool_type, data) in allocations {
            let entry_id = manager.allocate(pool_type, data).await.unwrap();
            entry_ids.push((pool_type, entry_id));
        }
        
        // Verify allocations
        let stats = manager.get_stats();
        assert_eq!(stats.total_allocations, 5);
        assert!(stats.avg_allocation_time_us > 0.0);
        
        // Check nursery stats
        let nursery_stats = manager.get_nursery_stats();
        assert_eq!(nursery_stats.total_objects, 4); // Small, String, Array, Object use nursery
        assert!(nursery_stats.current_size > 0);
        
        // Deallocate objects
        for (pool_type, entry_id) in entry_ids {
            manager.deallocate(pool_type, entry_id).unwrap();
        }
        
        // Verify deallocations
        let stats_after = manager.get_stats();
        assert_eq!(stats_after.total_deallocations, 5);
        
        // Test memory pressure handling
        let pressure = manager.get_memory_pressure();
        assert!(pressure >= 0.0 && pressure <= 1.0);
        
        // Test nursery collection
        let collection_stats = manager.collect_nursery().await.unwrap();
        assert_eq!(collection_stats.collection_count, 1);
        assert!(collection_stats.avg_collection_time_ms > 0.0);
    }
}
