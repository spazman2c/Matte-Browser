#[cfg(test)]
mod tests {
    use super::*;
    use crate::inline_cache::{
        InlineCacheManager, PropertyCache, MethodCache, GlobalCache, ShapeRegistry,
        PropertyCacheEntry, MethodCacheEntry, GlobalCacheEntry, Value, ObjectValue, FunctionValue, ClassValue,
        CacheStats, InlineCacheStats, ShapeDefinition
    };
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_property_cache_creation() {
        let mut cache = PropertyCache::new(100);
        assert_eq!(cache.get_stats().size, 0);
        assert_eq!(cache.get_stats().max_size, 100);
        assert_eq!(cache.get_stats().hits, 0);
        assert_eq!(cache.get_stats().misses, 0);
    }

    #[tokio::test]
    async fn test_property_cache_store_and_lookup() {
        let mut cache = PropertyCache::new(10);
        
        // Store a property
        let value = Value::Number(42.0);
        cache.store(1, "x".to_string(), 100, 0, value.clone());
        
        // Look up the property
        let entry = cache.lookup(1, "x");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().value, value);
        assert_eq!(entry.unwrap().shape_id, 100);
        assert_eq!(entry.unwrap().offset, 0);
    }

    #[tokio::test]
    async fn test_property_cache_miss() {
        let mut cache = PropertyCache::new(10);
        
        // Try to look up a non-existent property
        let entry = cache.lookup(1, "x");
        assert!(entry.is_none());
        assert_eq!(cache.get_stats().misses, 1);
    }

    #[tokio::test]
    async fn test_property_cache_update() {
        let mut cache = PropertyCache::new(10);
        
        // Store initial value
        cache.store(1, "x".to_string(), 100, 0, Value::Number(42.0));
        
        // Update the value
        cache.update(1, "x", Value::String("updated".to_string()));
        
        // Verify the update
        let entry = cache.lookup(1, "x");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().value, Value::String("updated".to_string()));
        assert_eq!(entry.unwrap().hit_count, 2); // 1 for store, 1 for update
    }

    #[tokio::test]
    async fn test_property_cache_invalidation() {
        let mut cache = PropertyCache::new(10);
        
        // Store properties
        cache.store(1, "x".to_string(), 100, 0, Value::Number(42.0));
        cache.store(1, "y".to_string(), 100, 1, Value::String("hello".to_string()));
        cache.store(2, "x".to_string(), 200, 0, Value::Boolean(true));
        
        // Invalidate object 1
        cache.invalidate_object(1);
        
        // Verify object 1 properties are gone
        assert!(cache.lookup(1, "x").is_none());
        assert!(cache.lookup(1, "y").is_none());
        
        // Verify object 2 properties remain
        assert!(cache.lookup(2, "x").is_some());
    }

    #[tokio::test]
    async fn test_property_cache_eviction() {
        let mut cache = PropertyCache::new(3);
        
        // Fill the cache
        cache.store(1, "x".to_string(), 100, 0, Value::Number(1.0));
        cache.store(1, "y".to_string(), 100, 1, Value::Number(2.0));
        cache.store(1, "z".to_string(), 100, 2, Value::Number(3.0));
        
        // This should trigger eviction
        cache.store(2, "w".to_string(), 200, 0, Value::Number(4.0));
        
        // Cache should be at half capacity
        assert_eq!(cache.get_stats().size, 2);
    }

    #[tokio::test]
    async fn test_method_cache_creation() {
        let mut cache = MethodCache::new(100);
        assert_eq!(cache.get_stats().size, 0);
        assert_eq!(cache.get_stats().max_size, 100);
    }

    #[tokio::test]
    async fn test_method_cache_store_and_lookup() {
        let mut cache = MethodCache::new(10);
        
        let method = FunctionValue {
            name: "test".to_string(),
            param_count: 2,
            local_count: 3,
            closure: HashMap::new(),
        };
        
        // Store a method
        cache.store(1, "test".to_string(), 100, 0, method.clone());
        
        // Look up the method
        let entry = cache.lookup(1, "test");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().method.name, "test");
        assert_eq!(entry.unwrap().shape_id, 100);
        assert_eq!(entry.unwrap().offset, 0);
    }

    #[tokio::test]
    async fn test_method_cache_update() {
        let mut cache = MethodCache::new(10);
        
        let method1 = FunctionValue {
            name: "test".to_string(),
            param_count: 2,
            local_count: 3,
            closure: HashMap::new(),
        };
        
        let method2 = FunctionValue {
            name: "test".to_string(),
            param_count: 3,
            local_count: 4,
            closure: HashMap::new(),
        };
        
        // Store initial method
        cache.store(1, "test".to_string(), 100, 0, method1);
        
        // Update the method
        cache.update(1, "test", method2);
        
        // Verify the update
        let entry = cache.lookup(1, "test");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().method.param_count, 3);
        assert_eq!(entry.unwrap().method.local_count, 4);
    }

    #[tokio::test]
    async fn test_global_cache_creation() {
        let mut cache = GlobalCache::new(100);
        assert_eq!(cache.get_stats().size, 0);
        assert_eq!(cache.get_stats().max_size, 100);
    }

    #[tokio::test]
    async fn test_global_cache_store_and_lookup() {
        let mut cache = GlobalCache::new(10);
        
        // Store a global variable
        cache.store("globalVar".to_string(), Value::Number(42.0));
        
        // Look up the variable
        let entry = cache.lookup("globalVar");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().value, Value::Number(42.0));
        assert_eq!(entry.unwrap().name, "globalVar");
    }

    #[tokio::test]
    async fn test_global_cache_update() {
        let mut cache = GlobalCache::new(10);
        
        // Store initial value
        cache.store("globalVar".to_string(), Value::Number(42.0));
        
        // Update the value
        cache.update("globalVar", Value::String("updated".to_string()));
        
        // Verify the update
        let entry = cache.lookup("globalVar");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().value, Value::String("updated".to_string()));
    }

    #[tokio::test]
    async fn test_global_cache_invalidation() {
        let mut cache = GlobalCache::new(10);
        
        // Store variables
        cache.store("x".to_string(), Value::Number(1.0));
        cache.store("y".to_string(), Value::Number(2.0));
        
        // Invalidate one variable
        cache.invalidate("x");
        
        // Verify x is gone, y remains
        assert!(cache.lookup("x").is_none());
        assert!(cache.lookup("y").is_some());
    }

    #[tokio::test]
    async fn test_shape_registry_creation() {
        let registry = ShapeRegistry::new();
        assert_eq!(registry.get_all_shapes().len(), 0);
    }

    #[tokio::test]
    async fn test_shape_registry_create_shape() {
        let mut registry = ShapeRegistry::new();
        
        let properties = vec!["x".to_string(), "y".to_string()];
        let shape_id = registry.create_shape(properties.clone(), None);
        
        let shape = registry.get_shape(shape_id);
        assert!(shape.is_some());
        assert_eq!(shape.unwrap().id, shape_id);
        assert_eq!(shape.unwrap().properties, properties);
        assert_eq!(shape.unwrap().offsets.get("x"), Some(&0));
        assert_eq!(shape.unwrap().offsets.get("y"), Some(&1));
    }

    #[tokio::test]
    async fn test_shape_registry_transition() {
        let mut registry = ShapeRegistry::new();
        
        // Create base shape
        let base_properties = vec!["x".to_string()];
        let base_shape_id = registry.create_shape(base_properties, None);
        
        // Transition to new shape
        let new_shape_id = registry.transition_shape(base_shape_id, "y".to_string());
        
        let new_shape = registry.get_shape(new_shape_id);
        assert!(new_shape.is_some());
        assert_eq!(new_shape.unwrap().properties, vec!["x".to_string(), "y".to_string()]);
        assert_eq!(new_shape.unwrap().parent, Some(base_shape_id));
    }

    #[tokio::test]
    async fn test_inline_cache_manager_creation() {
        let manager = InlineCacheManager::new(100, 50, 25);
        
        let stats = manager.get_stats();
        assert_eq!(stats.property_cache.size, 0);
        assert_eq!(stats.method_cache.size, 0);
        assert_eq!(stats.global_cache.size, 0);
        assert_eq!(stats.shape_count, 0);
    }

    #[tokio::test]
    async fn test_inline_cache_manager_property_operations() {
        let manager = InlineCacheManager::new(100, 50, 25);
        
        // Store a property
        manager.store_property(1, "x".to_string(), 100, 0, Value::Number(42.0));
        
        // Look up the property
        let value = manager.lookup_property(1, "x");
        assert!(value.is_some());
        assert_eq!(value.unwrap(), Value::Number(42.0));
    }

    #[tokio::test]
    async fn test_inline_cache_manager_method_operations() {
        let manager = InlineCacheManager::new(100, 50, 25);
        
        let method = FunctionValue {
            name: "test".to_string(),
            param_count: 2,
            local_count: 3,
            closure: HashMap::new(),
        };
        
        // Store a method
        manager.store_method(1, "test".to_string(), 100, 0, method.clone());
        
        // Look up the method
        let found_method = manager.lookup_method(1, "test");
        assert!(found_method.is_some());
        assert_eq!(found_method.unwrap().name, "test");
    }

    #[tokio::test]
    async fn test_inline_cache_manager_global_operations() {
        let manager = InlineCacheManager::new(100, 50, 25);
        
        // Store a global variable
        manager.store_global("globalVar".to_string(), Value::Number(42.0));
        
        // Look up the variable
        let value = manager.lookup_global("globalVar");
        assert!(value.is_some());
        assert_eq!(value.unwrap(), Value::Number(42.0));
    }

    #[tokio::test]
    async fn test_inline_cache_manager_invalidation() {
        let manager = InlineCacheManager::new(100, 50, 25);
        
        // Store properties and methods for object 1
        manager.store_property(1, "x".to_string(), 100, 0, Value::Number(42.0));
        manager.store_method(1, "test".to_string(), 100, 0, FunctionValue {
            name: "test".to_string(),
            param_count: 0,
            local_count: 0,
            closure: HashMap::new(),
        });
        
        // Store properties and methods for object 2
        manager.store_property(2, "x".to_string(), 200, 0, Value::Number(100.0));
        
        // Invalidate object 1
        manager.invalidate_object(1);
        
        // Verify object 1 is gone, object 2 remains
        assert!(manager.lookup_property(1, "x").is_none());
        assert!(manager.lookup_method(1, "test").is_none());
        assert!(manager.lookup_property(2, "x").is_some());
    }

    #[tokio::test]
    async fn test_inline_cache_manager_clear_all() {
        let manager = InlineCacheManager::new(100, 50, 25);
        
        // Add some data
        manager.store_property(1, "x".to_string(), 100, 0, Value::Number(42.0));
        manager.store_method(1, "test".to_string(), 100, 0, FunctionValue {
            name: "test".to_string(),
            param_count: 0,
            local_count: 0,
            closure: HashMap::new(),
        });
        manager.store_global("globalVar".to_string(), Value::Number(42.0));
        
        // Clear all
        manager.clear_all();
        
        // Verify everything is cleared
        let stats = manager.get_stats();
        assert_eq!(stats.property_cache.size, 0);
        assert_eq!(stats.method_cache.size, 0);
        assert_eq!(stats.global_cache.size, 0);
        assert_eq!(stats.shape_count, 0);
    }

    #[tokio::test]
    async fn test_cache_stats_calculation() {
        let mut cache = PropertyCache::new(10);
        
        // Add some hits and misses
        cache.store(1, "x".to_string(), 100, 0, Value::Number(42.0));
        cache.lookup(1, "x"); // Hit
        cache.lookup(1, "y"); // Miss
        cache.lookup(1, "x"); // Hit
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate, 2.0 / 3.0);
    }

    #[tokio::test]
    async fn test_value_operations() {
        // Test different value types
        let values = vec![
            Value::Undefined,
            Value::Null,
            Value::Boolean(true),
            Value::Number(42.0),
            Value::String("hello".to_string()),
            Value::Object(ObjectValue {
                shape_id: 1,
                properties: HashMap::new(),
                prototype: None,
            }),
            Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
        ];
        
        let mut cache = PropertyCache::new(10);
        for (i, value) in values.iter().enumerate() {
            cache.store(i as u64, format!("prop{}", i), 100, i, value.clone());
            let entry = cache.lookup(i as u64, &format!("prop{}", i));
            assert!(entry.is_some());
            assert_eq!(entry.unwrap().value, *value);
        }
    }

    #[tokio::test]
    async fn test_function_value_operations() {
        let function = FunctionValue {
            name: "test_function".to_string(),
            param_count: 3,
            local_count: 5,
            closure: {
                let mut map = HashMap::new();
                map.insert("x".to_string(), Value::Number(42.0));
                map
            },
        };
        
        assert_eq!(function.name, "test_function");
        assert_eq!(function.param_count, 3);
        assert_eq!(function.local_count, 5);
        assert_eq!(function.closure.len(), 1);
    }

    #[tokio::test]
    async fn test_class_value_operations() {
        let class = ClassValue {
            name: "TestClass".to_string(),
            constructor: None,
            methods: HashMap::new(),
            static_methods: HashMap::new(),
            properties: {
                let mut map = HashMap::new();
                map.insert("x".to_string(), Value::Number(42.0));
                map
            },
        };
        
        assert_eq!(class.name, "TestClass");
        assert!(class.constructor.is_none());
        assert_eq!(class.methods.len(), 0);
        assert_eq!(class.static_methods.len(), 0);
        assert_eq!(class.properties.len(), 1);
    }

    #[tokio::test]
    async fn test_inline_cache_integration() {
        let manager = InlineCacheManager::new(100, 50, 25);
        
        // Test property caching
        manager.store_property(1, "x".to_string(), 100, 0, Value::Number(42.0));
        let prop_value = manager.lookup_property(1, "x");
        assert!(prop_value.is_some());
        
        // Test method caching
        let method = FunctionValue {
            name: "test".to_string(),
            param_count: 2,
            local_count: 3,
            closure: HashMap::new(),
        };
        manager.store_method(1, "test".to_string(), 100, 0, method.clone());
        let method_value = manager.lookup_method(1, "test");
        assert!(method_value.is_some());
        
        // Test global caching
        manager.store_global("globalVar".to_string(), Value::String("hello".to_string()));
        let global_value = manager.lookup_global("globalVar");
        assert!(global_value.is_some());
        
        // Test shape registry
        let shape_registry = manager.shape_registry();
        let mut registry = shape_registry.write();
        let shape_id = registry.create_shape(vec!["x".to_string(), "y".to_string()], None);
        assert!(registry.get_shape(shape_id).is_some());
        
        // Verify statistics
        let stats = manager.get_stats();
        assert_eq!(stats.property_cache.size, 1);
        assert_eq!(stats.method_cache.size, 1);
        assert_eq!(stats.global_cache.size, 1);
        assert_eq!(stats.shape_count, 1);
    }
}
