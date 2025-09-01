#[cfg(test)]
mod tests {
    use super::*;
    use crate::garbage_collector::{
        GarbageCollector, GCConfig, GCStrategy, MemoryObject, RootReference, RootType,
        ReferenceState, GCStats, GenerationalConfig, IncrementalConfig
    };

    #[tokio::test]
    async fn test_garbage_collector_creation() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        let stats = gc.get_stats();
        assert_eq!(stats.total_collections, 0);
        assert_eq!(stats.total_objects_collected, 0);
        assert_eq!(stats.total_memory_freed, 0);
        assert_eq!(stats.current_heap_size, 0);
        assert_eq!(stats.live_objects, 0);
    }

    #[tokio::test]
    async fn test_object_allocation() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        let data = vec![1, 2, 3, 4, 5];
        let object_id = gc.allocate("test_object", 100, data.clone()).unwrap();
        
        let object = gc.get_object(object_id);
        assert!(object.is_some());
        
        let obj = object.unwrap();
        assert_eq!(obj.id, object_id);
        assert_eq!(obj.object_type, "test_object");
        assert_eq!(obj.size, 100);
        assert_eq!(obj.data, data);
        assert_eq!(obj.reference_count, 1);
        assert_eq!(obj.state, ReferenceState::Reachable);
        assert_eq!(obj.generation, 0);
    }

    #[tokio::test]
    async fn test_multiple_object_allocation() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        let object_ids = vec![
            gc.allocate("obj1", 50, vec![1, 2, 3]).unwrap(),
            gc.allocate("obj2", 75, vec![4, 5, 6]).unwrap(),
            gc.allocate("obj3", 25, vec![7, 8, 9]).unwrap(),
        ];
        
        let objects = gc.get_all_objects();
        assert_eq!(objects.len(), 3);
        
        for (i, object_id) in object_ids.iter().enumerate() {
            let object = gc.get_object(*object_id);
            assert!(object.is_some());
            assert_eq!(object.unwrap().object_type, format!("obj{}", i + 1));
        }
    }

    #[tokio::test]
    async fn test_reference_management() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        let obj1_id = gc.allocate("obj1", 50, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 50, vec![4, 5, 6]).unwrap();
        
        // Add reference from obj1 to obj2
        gc.add_reference(obj1_id, obj2_id).unwrap();
        
        let obj1 = gc.get_object(obj1_id).unwrap();
        assert!(obj1.references.contains(&obj2_id));
        
        // Remove reference
        gc.remove_reference(obj1_id, obj2_id).unwrap();
        
        let obj1_updated = gc.get_object(obj1_id).unwrap();
        assert!(!obj1_updated.references.contains(&obj2_id));
    }

    #[tokio::test]
    async fn test_root_reference_management() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        let obj1_id = gc.allocate("obj1", 50, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 50, vec![4, 5, 6]).unwrap();
        
        // Add root reference
        gc.add_root("global_vars", vec![obj1_id, obj2_id], RootType::Global).unwrap();
        
        let roots = gc.get_roots();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].id, "global_vars");
        assert_eq!(roots[0].root_type, RootType::Global);
        assert_eq!(roots[0].object_ids, vec![obj1_id, obj2_id]);
        
        // Remove root reference
        gc.remove_root("global_vars").unwrap();
        
        let roots_after = gc.get_roots();
        assert_eq!(roots_after.len(), 0);
    }

    #[tokio::test]
    async fn test_mark_and_sweep_gc() {
        let mut config = GCConfig::default();
        config.strategy = GCStrategy::MarkAndSweep;
        config.memory_threshold = 1000; // Low threshold for testing
        let gc = GarbageCollector::new(config);
        
        // Create objects with references
        let obj1_id = gc.allocate("obj1", 100, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 100, vec![4, 5, 6]).unwrap();
        let obj3_id = gc.allocate("obj3", 100, vec![7, 8, 9]).unwrap();
        
        // Create reference chain: obj1 -> obj2 -> obj3
        gc.add_reference(obj1_id, obj2_id).unwrap();
        gc.add_reference(obj2_id, obj3_id).unwrap();
        
        // Add root reference to obj1
        gc.add_root("root", vec![obj1_id], RootType::Global).unwrap();
        
        // Create unreferenced object
        let unreferenced_id = gc.allocate("unreferenced", 100, vec![10, 11, 12]).unwrap();
        
        let stats_before = gc.get_stats();
        assert_eq!(stats_before.live_objects, 4);
        
        // Perform garbage collection
        let stats_after = gc.collect_garbage().await.unwrap();
        
        // Unreferenced object should be collected
        assert!(gc.get_object(unreferenced_id).is_none());
        assert_eq!(stats_after.total_objects_collected, 1);
        assert_eq!(stats_after.total_memory_freed, 100);
        assert_eq!(stats_after.live_objects, 3);
    }

    #[tokio::test]
    async fn test_generational_gc() {
        let mut config = GCConfig::default();
        config.strategy = GCStrategy::Generational;
        config.generational_config = GenerationalConfig {
            generations: 3,
            promotion_thresholds: vec![1, 5, 10],
            collection_frequencies: vec![0.1, 0.5, 1.0],
        };
        let gc = GarbageCollector::new(config);
        
        // Create objects in different generations
        let obj1_id = gc.allocate("obj1", 50, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 50, vec![4, 5, 6]).unwrap();
        let obj3_id = gc.allocate("obj3", 50, vec![7, 8, 9]).unwrap();
        
        // Simulate object aging by updating reference counts
        {
            let mut objects = gc.get_all_objects();
            for obj in &mut objects {
                match obj.object_type.as_str() {
                    "obj1" => obj.reference_count = 2, // Will be promoted to generation 1
                    "obj2" => obj.reference_count = 6, // Will be promoted to generation 2
                    "obj3" => obj.reference_count = 12, // Will be promoted to generation 2
                    _ => {}
                }
            }
        }
        
        // Perform generational collection
        let stats = gc.collect_garbage().await.unwrap();
        assert_eq!(stats.total_collections, 1);
    }

    #[tokio::test]
    async fn test_incremental_gc() {
        let mut config = GCConfig::default();
        config.strategy = GCStrategy::Incremental;
        config.incremental_config = IncrementalConfig {
            max_step_time_ms: 5,
            objects_per_step: 2,
            use_write_barriers: true,
        };
        let gc = GarbageCollector::new(config);
        
        // Create multiple objects
        let object_ids: Vec<u64> = (0..10)
            .map(|i| gc.allocate(&format!("obj{}", i), 50, vec![i]).unwrap())
            .collect();
        
        // Add some references
        for i in 0..5 {
            gc.add_reference(object_ids[i], object_ids[i + 1]).unwrap();
        }
        
        // Add root reference
        gc.add_root("root", vec![object_ids[0]], RootType::Global).unwrap();
        
        // Perform incremental collection
        let stats = gc.collect_garbage().await.unwrap();
        assert_eq!(stats.total_collections, 1);
    }

    #[tokio::test]
    async fn test_concurrent_gc() {
        let mut config = GCConfig::default();
        config.strategy = GCStrategy::Concurrent;
        let gc = GarbageCollector::new(config);
        
        // Create objects
        let obj1_id = gc.allocate("obj1", 50, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 50, vec![4, 5, 6]).unwrap();
        
        // Add root reference
        gc.add_root("root", vec![obj1_id], RootType::Global).unwrap();
        
        // Perform concurrent collection
        let stats = gc.collect_garbage().await.unwrap();
        assert_eq!(stats.total_collections, 1);
    }

    #[tokio::test]
    async fn test_gc_statistics() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        // Allocate objects
        let obj1_id = gc.allocate("obj1", 100, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 150, vec![4, 5, 6]).unwrap();
        
        let stats_before = gc.get_stats();
        assert_eq!(stats_before.live_objects, 2);
        assert_eq!(stats_before.current_heap_size, 250);
        assert_eq!(stats_before.peak_heap_size, 250);
        
        // Add root and perform collection
        gc.add_root("root", vec![obj1_id], RootType::Global).unwrap();
        let stats_after = gc.collect_garbage().await.unwrap();
        
        assert_eq!(stats_after.total_collections, 1);
        assert_eq!(stats_after.total_objects_collected, 1); // obj2 should be collected
        assert_eq!(stats_after.total_memory_freed, 150);
        assert!(stats_after.avg_collection_time_ms > 0.0);
        assert!(stats_after.last_collection_time_ms > 0.0);
    }

    #[tokio::test]
    async fn test_gc_disabled() {
        let mut config = GCConfig::default();
        config.enabled = false;
        let gc = GarbageCollector::new(config);
        
        // Allocate objects
        let obj1_id = gc.allocate("obj1", 100, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 100, vec![4, 5, 6]).unwrap();
        
        // Add root reference
        gc.add_root("root", vec![obj1_id], RootType::Global).unwrap();
        
        // Perform collection (should be no-op)
        let stats = gc.collect_garbage().await.unwrap();
        assert_eq!(stats.total_collections, 0);
        assert_eq!(stats.total_objects_collected, 0);
        
        // All objects should still exist
        assert!(gc.get_object(obj1_id).is_some());
        assert!(gc.get_object(obj2_id).is_some());
    }

    #[tokio::test]
    async fn test_memory_threshold_triggering() {
        let mut config = GCConfig::default();
        config.memory_threshold = 200; // Low threshold
        let gc = GarbageCollector::new(config);
        
        // Allocate objects to exceed threshold
        let obj1_id = gc.allocate("obj1", 100, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 150, vec![4, 5, 6]).unwrap();
        
        // Add root reference to prevent collection
        gc.add_root("root", vec![obj1_id], RootType::Global).unwrap();
        
        let stats = gc.get_stats();
        assert_eq!(stats.current_heap_size, 250);
        assert!(stats.current_heap_size > config.memory_threshold);
    }

    #[tokio::test]
    async fn test_object_lifecycle() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        // Create object
        let object_id = gc.allocate("lifecycle_obj", 100, vec![1, 2, 3]).unwrap();
        let object = gc.get_object(object_id).unwrap();
        
        // Verify initial state
        assert_eq!(object.state, ReferenceState::Reachable);
        assert_eq!(object.reference_count, 1);
        assert_eq!(object.generation, 0);
        
        // Add references
        let obj2_id = gc.allocate("obj2", 50, vec![4, 5]).unwrap();
        gc.add_reference(object_id, obj2_id).unwrap();
        
        let updated_object = gc.get_object(object_id).unwrap();
        assert!(updated_object.references.contains(&obj2_id));
        
        // Remove references
        gc.remove_reference(object_id, obj2_id).unwrap();
        
        let final_object = gc.get_object(object_id).unwrap();
        assert!(!final_object.references.contains(&obj2_id));
    }

    #[tokio::test]
    async fn test_root_types() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        let obj1_id = gc.allocate("obj1", 50, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 50, vec![4, 5, 6]).unwrap();
        
        // Add different types of roots
        gc.add_root("globals", vec![obj1_id], RootType::Global).unwrap();
        gc.add_root("stack", vec![obj2_id], RootType::Stack).unwrap();
        
        let roots = gc.get_roots();
        assert_eq!(roots.len(), 2);
        
        let global_root = roots.iter().find(|r| r.root_type == RootType::Global).unwrap();
        let stack_root = roots.iter().find(|r| r.root_type == RootType::Stack).unwrap();
        
        assert_eq!(global_root.id, "globals");
        assert_eq!(stack_root.id, "stack");
    }

    #[tokio::test]
    async fn test_reference_state_transitions() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        let obj1_id = gc.allocate("obj1", 50, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 50, vec![4, 5, 6]).unwrap();
        
        // Initially both objects are reachable
        assert_eq!(gc.get_object(obj1_id).unwrap().state, ReferenceState::Reachable);
        assert_eq!(gc.get_object(obj2_id).unwrap().state, ReferenceState::Reachable);
        
        // Add root reference to obj1 only
        gc.add_root("root", vec![obj1_id], RootType::Global).unwrap();
        
        // Perform collection
        gc.collect_garbage().await.unwrap();
        
        // obj1 should still be reachable, obj2 should be collected
        assert!(gc.get_object(obj1_id).is_some());
        assert!(gc.get_object(obj2_id).is_none());
    }

    #[tokio::test]
    async fn test_gc_clear() {
        let config = GCConfig::default();
        let gc = GarbageCollector::new(config);
        
        // Add some objects and roots
        let obj1_id = gc.allocate("obj1", 50, vec![1, 2, 3]).unwrap();
        let obj2_id = gc.allocate("obj2", 50, vec![4, 5, 6]).unwrap();
        gc.add_root("root", vec![obj1_id], RootType::Global).unwrap();
        
        // Verify data exists
        assert_eq!(gc.get_all_objects().len(), 2);
        assert_eq!(gc.get_roots().len(), 1);
        
        // Clear everything
        gc.clear();
        
        // Verify everything is cleared
        assert_eq!(gc.get_all_objects().len(), 0);
        assert_eq!(gc.get_roots().len(), 0);
        
        let stats = gc.get_stats();
        assert_eq!(stats.total_collections, 0);
        assert_eq!(stats.total_objects_collected, 0);
        assert_eq!(stats.current_heap_size, 0);
        assert_eq!(stats.live_objects, 0);
    }

    #[tokio::test]
    async fn test_gc_config_default() {
        let config = GCConfig::default();
        
        assert_eq!(config.strategy, GCStrategy::MarkAndSweep);
        assert_eq!(config.memory_threshold, 1024 * 1024); // 1MB
        assert_eq!(config.time_threshold, 30.0);
        assert_eq!(config.max_heap_size, 100 * 1024 * 1024); // 100MB
        assert!(config.enabled);
        assert_eq!(config.collection_timeout_ms, 5000);
        assert_eq!(config.generational_config.generations, 3);
        assert_eq!(config.incremental_config.max_step_time_ms, 10);
        assert!(config.incremental_config.use_write_barriers);
    }

    #[tokio::test]
    async fn test_gc_integration() {
        let mut config = GCConfig::default();
        config.memory_threshold = 500; // Low threshold for testing
        let gc = GarbageCollector::new(config);
        
        // Create a complex object graph
        let root_obj_id = gc.allocate("root_obj", 100, vec![1, 2, 3]).unwrap();
        let child1_id = gc.allocate("child1", 75, vec![4, 5, 6]).unwrap();
        let child2_id = gc.allocate("child2", 75, vec![7, 8, 9]).unwrap();
        let grandchild_id = gc.allocate("grandchild", 50, vec![10, 11, 12]).unwrap();
        
        // Create reference chain: root -> child1 -> grandchild, root -> child2
        gc.add_reference(root_obj_id, child1_id).unwrap();
        gc.add_reference(root_obj_id, child2_id).unwrap();
        gc.add_reference(child1_id, grandchild_id).unwrap();
        
        // Create unreferenced objects
        let unreferenced1_id = gc.allocate("unreferenced1", 100, vec![13, 14, 15]).unwrap();
        let unreferenced2_id = gc.allocate("unreferenced2", 100, vec![16, 17, 18]).unwrap();
        
        // Add root reference
        gc.add_root("main_root", vec![root_obj_id], RootType::Global).unwrap();
        
        // Verify initial state
        let stats_before = gc.get_stats();
        assert_eq!(stats_before.live_objects, 6);
        assert_eq!(stats_before.current_heap_size, 500);
        
        // Perform garbage collection
        let stats_after = gc.collect_garbage().await.unwrap();
        
        // Verify collection results
        assert_eq!(stats_after.total_collections, 1);
        assert_eq!(stats_after.total_objects_collected, 2); // unreferenced1 and unreferenced2
        assert_eq!(stats_after.total_memory_freed, 200);
        assert_eq!(stats_after.live_objects, 4);
        
        // Verify reachable objects still exist
        assert!(gc.get_object(root_obj_id).is_some());
        assert!(gc.get_object(child1_id).is_some());
        assert!(gc.get_object(child2_id).is_some());
        assert!(gc.get_object(grandchild_id).is_some());
        
        // Verify unreferenced objects are collected
        assert!(gc.get_object(unreferenced1_id).is_none());
        assert!(gc.get_object(unreferenced2_id).is_none());
    }
}
