#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{
        CacheStatus, CacheEntry, CacheControl, CachePartition, EvictionPolicy,
        CacheStats, CacheWarmingEntry, CacheAnalytics, MemoryCache, DiskCache,
        CacheConfig, CacheManager, CacheWarmingManager
    };
    use std::collections::HashMap;
    use std::time::Duration;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_cache_entry_creation() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        headers.insert("Cache-Control".to_string(), "max-age=3600".to_string());
        
        let entry = CacheEntry::new(
            "https://example.com/page".to_string(),
            b"<html>Hello World</html>".to_vec(),
            headers,
            200,
        );
        
        assert_eq!(entry.key, "https://example.com/page");
        assert_eq!(entry.data, b"<html>Hello World</html>");
        assert_eq!(entry.status_code, 200);
        assert_eq!(entry.content_length, 25);
        assert_eq!(entry.size, 25);
        assert_eq!(entry.access_count, 0);
        assert!(entry.is_fresh());
        assert!(!entry.is_stale());
        assert!(!entry.is_expired());
        assert_eq!(entry.status(), CacheStatus::Fresh);
    }

    #[test]
    fn test_cache_entry_expiration() {
        let mut entry = CacheEntry::new(
            "https://example.com/page".to_string(),
            b"test".to_vec(),
            HashMap::new(),
            200,
        );
        
        // Set expiration in the past
        entry.expires = Some(std::time::SystemTime::now() - Duration::from_secs(3600));
        
        assert!(!entry.is_fresh());
        assert!(entry.is_stale());
        assert!(entry.is_expired());
        assert_eq!(entry.status(), CacheStatus::Expired);
        
        // Set expiration in the future
        entry.expires = Some(std::time::SystemTime::now() + Duration::from_secs(3600));
        
        assert!(entry.is_fresh());
        assert!(!entry.is_stale());
        assert!(!entry.is_expired());
        assert_eq!(entry.status(), CacheStatus::Fresh);
    }

    #[test]
    fn test_cache_entry_access_tracking() {
        let mut entry = CacheEntry::new(
            "https://example.com/page".to_string(),
            b"test".to_vec(),
            HashMap::new(),
            200,
        );
        
        let initial_access_count = entry.access_count;
        let initial_last_accessed = entry.last_accessed;
        
        // Wait a bit to ensure time difference
        std::thread::sleep(Duration::from_millis(10));
        
        entry.update_access();
        
        assert_eq!(entry.access_count, initial_access_count + 1);
        assert!(entry.last_accessed > initial_last_accessed);
    }

    #[test]
    fn test_cache_entry_age_and_ttl() {
        let mut entry = CacheEntry::new(
            "https://example.com/page".to_string(),
            b"test".to_vec(),
            HashMap::new(),
            200,
        );
        
        // Wait a bit
        std::thread::sleep(Duration::from_millis(10));
        
        let age = entry.age();
        assert!(age > Duration::ZERO);
        
        // Set TTL
        entry.expires = Some(std::time::SystemTime::now() + Duration::from_secs(3600));
        
        let ttl = entry.ttl();
        assert!(ttl.is_some());
        assert!(ttl.unwrap() > Duration::ZERO);
        assert!(ttl.unwrap() <= Duration::from_secs(3600));
    }

    #[test]
    fn test_cache_control_default() {
        let cache_control = CacheControl::default();
        
        assert!(cache_control.max_age.is_none());
        assert!(cache_control.s_maxage.is_none());
        assert!(!cache_control.no_cache);
        assert!(!cache_control.no_store);
        assert!(!cache_control.must_revalidate);
        assert!(!cache_control.proxy_revalidate);
        assert!(!cache_control.public);
        assert!(!cache_control.private);
        assert!(!cache_control.immutable);
        assert!(cache_control.stale_while_revalidate.is_none());
        assert!(cache_control.stale_if_error.is_none());
        assert!(cache_control.is_cacheable());
        assert!(!cache_control.requires_revalidation());
    }

    #[test]
    fn test_cache_control_parsing() {
        let cache_control = CacheControl::from_header("max-age=3600, public, no-cache");
        
        assert_eq!(cache_control.max_age, Some(Duration::from_secs(3600)));
        assert!(cache_control.public);
        assert!(cache_control.no_cache);
        assert!(cache_control.requires_revalidation());
        
        let cache_control = CacheControl::from_header("no-store, private");
        
        assert!(cache_control.no_store);
        assert!(cache_control.private);
        assert!(!cache_control.is_cacheable());
        
        let cache_control = CacheControl::from_header("must-revalidate, s-maxage=1800");
        
        assert!(cache_control.must_revalidate);
        assert_eq!(cache_control.s_maxage, Some(Duration::from_secs(1800)));
        assert!(cache_control.requires_revalidation());
    }

    #[test]
    fn test_cache_partition_creation() {
        let partition = CachePartition::new(
            "example.com".to_string(),
            1024 * 1024, // 1 MB
            100,
            EvictionPolicy::LRU,
        );
        
        assert_eq!(partition.name, "example.com");
        assert_eq!(partition.max_size, 1024 * 1024);
        assert_eq!(partition.current_size, 0);
        assert_eq!(partition.max_entries, 100);
        assert_eq!(partition.current_entries, 0);
        assert_eq!(partition.eviction_policy, EvictionPolicy::LRU);
        assert!(partition.entries.is_empty());
        assert!(partition.access_order.is_empty());
    }

    #[test]
    fn test_cache_partition_entry_management() {
        let mut partition = CachePartition::new(
            "example.com".to_string(),
            1024, // 1 KB
            10,
            EvictionPolicy::LRU,
        );
        
        let entry = CacheEntry::new(
            "https://example.com/page1".to_string(),
            b"data1".to_vec(),
            HashMap::new(),
            200,
        );
        
        // Add entry
        let old_entry = partition.add_entry("https://example.com/page1".to_string(), entry);
        assert!(old_entry.is_none());
        assert_eq!(partition.current_entries, 1);
        assert_eq!(partition.current_size, 5);
        assert_eq!(partition.access_order.len(), 1);
        
        // Get entry
        let entry = partition.get_entry("https://example.com/page1");
        assert!(entry.is_some());
        let entry = entry.unwrap();
        assert_eq!(entry.access_count, 1);
        
        // Remove entry
        let removed_entry = partition.remove_entry("https://example.com/page1");
        assert!(removed_entry.is_some());
        assert_eq!(partition.current_entries, 0);
        assert_eq!(partition.current_size, 0);
        assert!(partition.access_order.is_empty());
    }

    #[test]
    fn test_cache_partition_eviction() {
        let mut partition = CachePartition::new(
            "example.com".to_string(),
            20, // 20 bytes
            3,
            EvictionPolicy::LRU,
        );
        
        // Add entries that exceed the size limit
        for i in 1..=5 {
            let entry = CacheEntry::new(
                format!("https://example.com/page{}", i),
                vec![0u8; 10], // 10 bytes each
                HashMap::new(),
                200,
            );
            partition.add_entry(format!("https://example.com/page{}", i), entry);
        }
        
        // Should have evicted some entries
        assert!(partition.current_entries <= 2); // 20 bytes / 10 bytes per entry = 2 entries max
        assert!(partition.current_size <= 20);
    }

    #[test]
    fn test_cache_partition_expired_cleaning() {
        let mut partition = CachePartition::new(
            "example.com".to_string(),
            1024,
            10,
            EvictionPolicy::LRU,
        );
        
        // Add a fresh entry
        let mut fresh_entry = CacheEntry::new(
            "https://example.com/fresh".to_string(),
            b"fresh".to_vec(),
            HashMap::new(),
            200,
        );
        fresh_entry.expires = Some(std::time::SystemTime::now() + Duration::from_secs(3600));
        partition.add_entry("https://example.com/fresh".to_string(), fresh_entry);
        
        // Add an expired entry
        let mut expired_entry = CacheEntry::new(
            "https://example.com/expired".to_string(),
            b"expired".to_vec(),
            HashMap::new(),
            200,
        );
        expired_entry.expires = Some(std::time::SystemTime::now() - Duration::from_secs(3600));
        partition.add_entry("https://example.com/expired".to_string(), expired_entry);
        
        assert_eq!(partition.current_entries, 2);
        
        // Clean expired entries
        let expired_count = partition.clean_expired();
        assert_eq!(expired_count, 1);
        assert_eq!(partition.current_entries, 1);
        assert!(partition.entries.contains_key("https://example.com/fresh"));
        assert!(!partition.entries.contains_key("https://example.com/expired"));
    }

    #[test]
    fn test_cache_stats() {
        let mut stats = CacheStats::new();
        
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.hit_ratio, 0.0);
        assert_eq!(stats.total_size, 0);
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.evictions, 0);
        assert_eq!(stats.expirations, 0);
        
        // Record hits and misses
        stats.record_hit();
        stats.record_hit();
        stats.record_miss();
        
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_ratio, 2.0 / 3.0);
        
        // Record evictions and expirations
        stats.record_eviction();
        stats.record_expiration();
        
        assert_eq!(stats.evictions, 1);
        assert_eq!(stats.expirations, 1);
        
        // Reset statistics
        stats.reset();
        
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.hit_ratio, 0.0);
        assert_eq!(stats.evictions, 0);
        assert_eq!(stats.expirations, 0);
    }

    #[test]
    fn test_memory_cache_creation() {
        let config = CacheConfig::default();
        let cache = MemoryCache::new(config);
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }

    #[test]
    fn test_memory_cache_operations() {
        let config = CacheConfig::default();
        let cache = MemoryCache::new(config);
        
        let entry = CacheEntry::new(
            "https://example.com/page".to_string(),
            b"test data".to_vec(),
            HashMap::new(),
            200,
        );
        
        // Put entry
        cache.put("https://example.com/page", entry);
        
        // Get entry
        let retrieved_entry = cache.get("https://example.com/page");
        assert!(retrieved_entry.is_some());
        let retrieved_entry = retrieved_entry.unwrap();
        assert_eq!(retrieved_entry.data, b"test data");
        
        // Get non-existent entry
        let non_existent = cache.get("https://example.com/nonexistent");
        assert!(non_existent.is_none());
        
        // Remove entry
        let removed_entry = cache.remove("https://example.com/page");
        assert!(removed_entry.is_some());
        
        // Try to get removed entry
        let retrieved_entry = cache.get("https://example.com/page");
        assert!(retrieved_entry.is_none());
    }

    #[test]
    fn test_memory_cache_domain_partitioning() {
        let config = CacheConfig::default();
        let cache = MemoryCache::new(config);
        
        // Add entries from different domains
        let entry1 = CacheEntry::new(
            "https://example.com/page1".to_string(),
            b"data1".to_vec(),
            HashMap::new(),
            200,
        );
        let entry2 = CacheEntry::new(
            "https://api.example.com/page2".to_string(),
            b"data2".to_vec(),
            HashMap::new(),
            200,
        );
        
        cache.put("https://example.com/page1", entry1);
        cache.put("https://api.example.com/page2", entry2);
        
        // Both should be retrievable
        assert!(cache.get("https://example.com/page1").is_some());
        assert!(cache.get("https://api.example.com/page2").is_some());
        
        // But they should be in different partitions
        let stats = cache.get_stats();
        assert_eq!(stats.total_entries, 2);
    }

    #[test]
    fn test_memory_cache_expired_cleaning() {
        let config = CacheConfig::default();
        let cache = MemoryCache::new(config);
        
        // Add a fresh entry
        let mut fresh_entry = CacheEntry::new(
            "https://example.com/fresh".to_string(),
            b"fresh".to_vec(),
            HashMap::new(),
            200,
        );
        fresh_entry.expires = Some(std::time::SystemTime::now() + Duration::from_secs(3600));
        cache.put("https://example.com/fresh", fresh_entry);
        
        // Add an expired entry
        let mut expired_entry = CacheEntry::new(
            "https://example.com/expired".to_string(),
            b"expired".to_vec(),
            HashMap::new(),
            200,
        );
        expired_entry.expires = Some(std::time::SystemTime::now() - Duration::from_secs(3600));
        cache.put("https://example.com/expired", expired_entry);
        
        // Clean expired entries
        let expired_count = cache.clean_expired();
        assert_eq!(expired_count, 1);
        
        // Fresh entry should still be available
        assert!(cache.get("https://example.com/fresh").is_some());
        // Expired entry should be gone
        assert!(cache.get("https://example.com/expired").is_none());
    }

    #[test]
    fn test_disk_cache_creation() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = CacheConfig::default();
        config.cache_directory = Some(temp_dir.path().to_path_buf());
        
        let cache = DiskCache::new(config).unwrap();
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }

    #[test]
    fn test_disk_cache_operations() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = CacheConfig::default();
        config.cache_directory = Some(temp_dir.path().to_path_buf());
        
        let cache = DiskCache::new(config).unwrap();
        
        let mut entry = CacheEntry::new(
            "https://example.com/page".to_string(),
            b"test data for disk".to_vec(),
            HashMap::new(),
            200,
        );
        
        // Put entry
        cache.put("https://example.com/page", &mut entry).unwrap();
        
        // Get entry
        let retrieved_entry = cache.get("https://example.com/page");
        assert!(retrieved_entry.is_some());
        let retrieved_entry = retrieved_entry.unwrap();
        assert_eq!(retrieved_entry.data, b"test data for disk");
        
        // Get non-existent entry
        let non_existent = cache.get("https://example.com/nonexistent");
        assert!(non_existent.is_none());
        
        // Remove entry
        cache.remove("https://example.com/page").unwrap();
        
        // Try to get removed entry
        let retrieved_entry = cache.get("https://example.com/page");
        assert!(retrieved_entry.is_none());
    }

    #[test]
    fn test_disk_cache_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = CacheConfig::default();
        config.cache_directory = Some(temp_dir.path().to_path_buf());
        
        // Create cache and add entry
        {
            let cache = DiskCache::new(config.clone()).unwrap();
            let mut entry = CacheEntry::new(
                "https://example.com/page".to_string(),
                b"persistent data".to_vec(),
                HashMap::new(),
                200,
            );
            cache.put("https://example.com/page", &mut entry).unwrap();
        }
        
        // Create new cache instance (should load from disk)
        let cache = DiskCache::new(config).unwrap();
        
        // Entry should still be available
        let retrieved_entry = cache.get("https://example.com/page");
        assert!(retrieved_entry.is_some());
        let retrieved_entry = retrieved_entry.unwrap();
        assert_eq!(retrieved_entry.data, b"persistent data");
    }

    #[test]
    fn test_cache_config_default() {
        let config = CacheConfig::default();
        
        assert_eq!(config.max_total_size, 100 * 1024 * 1024); // 100 MB
        assert_eq!(config.max_memory_size, 50 * 1024 * 1024);  // 50 MB
        assert_eq!(config.max_disk_size, 500 * 1024 * 1024);   // 500 MB
        assert_eq!(config.default_ttl, Duration::from_secs(3600)); // 1 hour
        assert!(config.enable_memory_cache);
        assert!(config.enable_disk_cache);
        assert!(!config.enable_cache_warming);
        assert!(config.enable_analytics);
        assert_eq!(config.compression_threshold, 1024); // 1 KB
        assert_eq!(config.default_eviction_policy, EvictionPolicy::LRU);
        assert!(config.partition_by_domain);
        assert!(config.cache_directory.is_none());
    }

    #[test]
    fn test_cache_manager_creation() {
        let config = CacheConfig::default();
        let manager = CacheManager::new(config).unwrap();
        
        let stats = manager.get_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }

    #[test]
    fn test_cache_manager_operations() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = CacheConfig::default();
        config.cache_directory = Some(temp_dir.path().to_path_buf());
        
        let manager = CacheManager::new(config).unwrap();
        
        let entry = CacheEntry::new(
            "https://example.com/page".to_string(),
            b"manager test data".to_vec(),
            HashMap::new(),
            200,
        );
        
        // Put entry
        manager.put("https://example.com/page", entry).unwrap();
        
        // Get entry
        let retrieved_entry = manager.get("https://example.com/page");
        assert!(retrieved_entry.is_some());
        let retrieved_entry = retrieved_entry.unwrap();
        assert_eq!(retrieved_entry.data, b"manager test data");
        
        // Get non-existent entry
        let non_existent = manager.get("https://example.com/nonexistent");
        assert!(non_existent.is_none());
        
        // Remove entry
        manager.remove("https://example.com/page").unwrap();
        
        // Try to get removed entry
        let retrieved_entry = manager.get("https://example.com/page");
        assert!(retrieved_entry.is_none());
    }

    #[test]
    fn test_cache_manager_memory_disk_integration() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = CacheConfig::default();
        config.cache_directory = Some(temp_dir.path().to_path_buf());
        
        let manager = CacheManager::new(config).unwrap();
        
        let entry = CacheEntry::new(
            "https://example.com/page".to_string(),
            b"integration test data".to_vec(),
            HashMap::new(),
            200,
        );
        
        // Put entry (should go to both memory and disk)
        manager.put("https://example.com/page", entry).unwrap();
        
        // Get entry (should come from memory first)
        let retrieved_entry = manager.get("https://example.com/page");
        assert!(retrieved_entry.is_some());
        
        // Create new manager instance
        let manager2 = CacheManager::new(config).unwrap();
        
        // Entry should still be available (from disk)
        let retrieved_entry = manager2.get("https://example.com/page");
        assert!(retrieved_entry.is_some());
        let retrieved_entry = retrieved_entry.unwrap();
        assert_eq!(retrieved_entry.data, b"integration test data");
    }

    #[test]
    fn test_cache_warming_entry() {
        let entry = CacheWarmingEntry {
            url: "https://example.com/warm".to_string(),
            priority: 5,
            expected_content_type: Some("text/html".to_string()),
            ttl: Some(Duration::from_secs(1800)),
            created: std::time::SystemTime::now(),
        };
        
        assert_eq!(entry.url, "https://example.com/warm");
        assert_eq!(entry.priority, 5);
        assert_eq!(entry.expected_content_type, Some("text/html".to_string()));
        assert_eq!(entry.ttl, Some(Duration::from_secs(1800)));
    }

    #[test]
    fn test_cache_warming_manager() {
        let config = CacheConfig::default();
        let manager = CacheWarmingManager::new(config);
        
        let entry = CacheWarmingEntry {
            url: "https://example.com/warm".to_string(),
            priority: 5,
            expected_content_type: None,
            ttl: None,
            created: std::time::SystemTime::now(),
        };
        
        // Add warming entry
        manager.add_entry(entry);
        
        // Check queue
        let queue = manager.queue.read();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].url, "https://example.com/warm");
    }

    #[test]
    fn test_cache_analytics() {
        let mut analytics = CacheAnalytics::new();
        
        assert!(analytics.hit_rate_by_domain.is_empty());
        assert!(analytics.hit_rate_by_content_type.is_empty());
        assert_eq!(analytics.avg_response_time, Duration::ZERO);
        assert_eq!(analytics.cache_efficiency, 0.0);
        assert_eq!(analytics.storage_utilization, 0.0);
        assert_eq!(analytics.eviction_rate, 0.0);
        assert_eq!(analytics.warming_success_rate, 0.0);
        
        // Update analytics
        let mut stats = CacheStats::new();
        stats.record_hit();
        stats.record_hit();
        stats.record_miss();
        stats.record_eviction();
        stats.total_entries = 10;
        
        analytics.update(&stats);
        
        assert_eq!(analytics.cache_efficiency, 2.0 / 3.0);
        assert_eq!(analytics.eviction_rate, 1.0 / 10.0);
    }

    #[test]
    fn test_cache_eviction_policies() {
        // Test LRU eviction
        let mut partition = CachePartition::new(
            "test.com".to_string(),
            30, // 30 bytes
            3,
            EvictionPolicy::LRU,
        );
        
        // Add entries
        for i in 1..=4 {
            let entry = CacheEntry::new(
                format!("page{}", i),
                vec![0u8; 10], // 10 bytes each
                HashMap::new(),
                200,
            );
            partition.add_entry(format!("page{}", i), entry);
        }
        
        // Should have evicted the least recently used entry
        assert!(partition.current_entries <= 3);
        
        // Test LFU eviction
        let mut partition = CachePartition::new(
            "test.com".to_string(),
            30,
            3,
            EvictionPolicy::LFU,
        );
        
        // Add entries and access them differently
        for i in 1..=4 {
            let entry = CacheEntry::new(
                format!("page{}", i),
                vec![0u8; 10],
                HashMap::new(),
                200,
            );
            partition.add_entry(format!("page{}", i), entry);
        }
        
        // Access some entries more than others
        partition.get_entry("page1");
        partition.get_entry("page1");
        partition.get_entry("page2");
        
        // Should have evicted the least frequently used entry
        assert!(partition.current_entries <= 3);
    }

    #[test]
    fn test_cache_size_limits() {
        let config = CacheConfig {
            max_memory_size: 100, // 100 bytes
            max_disk_size: 200,   // 200 bytes
            ..CacheConfig::default()
        };
        
        let temp_dir = TempDir::new().unwrap();
        let mut config = config;
        config.cache_directory = Some(temp_dir.path().to_path_buf());
        
        let manager = CacheManager::new(config).unwrap();
        
        // Add entries that exceed memory limit
        for i in 1..=5 {
            let entry = CacheEntry::new(
                format!("https://example.com/page{}", i),
                vec![0u8; 30], // 30 bytes each
                HashMap::new(),
                200,
            );
            manager.put(&format!("https://example.com/page{}", i), entry).unwrap();
        }
        
        // Check that size limits are respected
        let stats = manager.get_stats();
        assert!(stats.total_size <= 300); // memory + disk limit
    }

    #[test]
    fn test_cache_validation_and_revalidation() {
        let mut entry = CacheEntry::new(
            "https://example.com/page".to_string(),
            b"test data".to_vec(),
            HashMap::new(),
            200,
        );
        
        // Set cache control to require revalidation
        entry.cache_control = CacheControl::from_header("must-revalidate");
        
        assert!(entry.cache_control.requires_revalidation());
        
        // Set ETag for validation
        entry.etag = Some("\"abc123\"".to_string());
        
        // Set last modified for validation
        entry.last_modified = Some(std::time::SystemTime::now() - Duration::from_secs(3600));
        
        assert!(entry.etag.is_some());
        assert!(entry.last_modified.is_some());
    }

    #[test]
    fn test_cache_partitioning_by_site() {
        let config = CacheConfig::default();
        let cache = MemoryCache::new(config);
        
        // Add entries from different sites
        let sites = vec!["example.com", "api.example.com", "cdn.example.com", "blog.example.com"];
        
        for site in &sites {
            let entry = CacheEntry::new(
                format!("https://{}/page", site),
                b"site specific data".to_vec(),
                HashMap::new(),
                200,
            );
            cache.put(&format!("https://{}/page", site), entry);
        }
        
        // All entries should be retrievable
        for site in &sites {
            assert!(cache.get(&format!("https://{}/page", site)).is_some());
        }
        
        // Check that they're in separate partitions
        let stats = cache.get_stats();
        assert_eq!(stats.total_entries, sites.len());
    }

    #[test]
    fn test_comprehensive_cache_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = CacheConfig::default();
        config.cache_directory = Some(temp_dir.path().to_path_buf());
        config.enable_cache_warming = true;
        config.enable_analytics = true;
        
        let manager = CacheManager::new(config).unwrap();
        
        // Add warming entry
        let warming_entry = CacheWarmingEntry {
            url: "https://example.com/warm".to_string(),
            priority: 5,
            expected_content_type: Some("text/html".to_string()),
            ttl: Some(Duration::from_secs(1800)),
            created: std::time::SystemTime::now(),
        };
        manager.add_warming_entry(warming_entry);
        
        // Add cache entries
        let entry1 = CacheEntry::new(
            "https://example.com/page1".to_string(),
            b"page 1 data".to_vec(),
            HashMap::new(),
            200,
        );
        let entry2 = CacheEntry::new(
            "https://example.com/page2".to_string(),
            b"page 2 data".to_vec(),
            HashMap::new(),
            200,
        );
        
        manager.put("https://example.com/page1", entry1).unwrap();
        manager.put("https://example.com/page2", entry2).unwrap();
        
        // Retrieve entries
        assert!(manager.get("https://example.com/page1").is_some());
        assert!(manager.get("https://example.com/page2").is_some());
        
        // Get statistics
        let stats = manager.get_stats();
        assert!(stats.hits > 0);
        assert!(stats.total_entries > 0);
        
        // Get analytics
        let analytics = manager.get_analytics();
        assert!(analytics.cache_efficiency >= 0.0);
        assert!(analytics.cache_efficiency <= 1.0);
        
        // Clean expired entries
        let expired_count = manager.clean_expired().unwrap();
        assert!(expired_count >= 0);
        
        // Verify entries are still available (they shouldn't be expired)
        assert!(manager.get("https://example.com/page1").is_some());
        assert!(manager.get("https://example.com/page2").is_some());
    }
}
