#[cfg(test)]
mod tests {
    use super::*;
    use crate::tiering::{
        TieringManager, TieringConfig, ExecutionTier, FunctionStats, CodeCacheEntry,
        ExecutionResult, TieringStats, EngineStats
    };

    #[tokio::test]
    async fn test_tiering_manager_creation() {
        let config = TieringConfig::default();
        let manager = TieringManager::new(config);
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_functions, 0);
        assert_eq!(stats.cached_functions, 0);
        assert_eq!(stats.baseline_queue_size, 0);
        assert_eq!(stats.optimizing_queue_size, 0);
    }

    #[tokio::test]
    async fn test_interpreter_execution() {
        let config = TieringConfig::default();
        let manager = TieringManager::new(config);
        
        let result = manager.execute_function("test_func", "console.log('hello')").await.unwrap();
        
        assert_eq!(result.function_id, "test_func");
        assert_eq!(result.tier, ExecutionTier::Interpreter);
        assert!(result.success);
        assert!(result.result.contains("Interpreted"));
        
        let stats = manager.get_stats();
        assert_eq!(stats.interpreter_stats.functions_executed, 1);
    }

    #[tokio::test]
    async fn test_baseline_promotion() {
        let mut config = TieringConfig::default();
        config.hot_threshold = 5; // Lower threshold for testing
        let manager = TieringManager::new(config);
        
        // Execute function multiple times to trigger baseline promotion
        for i in 0..10 {
            let result = manager.execute_function("hot_func", "console.log('hot')").await.unwrap();
            
            if i < 5 {
                assert_eq!(result.tier, ExecutionTier::Interpreter);
            } else {
                assert_eq!(result.tier, ExecutionTier::Baseline);
            }
        }
        
        let stats = manager.get_stats();
        assert_eq!(stats.baseline_stats.functions_executed, 5);
        assert_eq!(stats.interpreter_stats.functions_executed, 5);
    }

    #[tokio::test]
    async fn test_optimizing_promotion() {
        let mut config = TieringConfig::default();
        config.hot_threshold = 5;
        config.optimization_threshold = 10; // Lower threshold for testing
        let manager = TieringManager::new(config);
        
        // Execute function many times to trigger optimizing promotion
        for i in 0..15 {
            let result = manager.execute_function("very_hot_func", "console.log('very hot')").await.unwrap();
            
            if i < 5 {
                assert_eq!(result.tier, ExecutionTier::Interpreter);
            } else if i < 10 {
                assert_eq!(result.tier, ExecutionTier::Baseline);
            } else {
                assert_eq!(result.tier, ExecutionTier::Optimizing);
            }
        }
        
        let stats = manager.get_stats();
        assert_eq!(stats.optimizing_stats.functions_executed, 5);
        assert_eq!(stats.baseline_stats.functions_executed, 5);
        assert_eq!(stats.interpreter_stats.functions_executed, 5);
    }

    #[tokio::test]
    async fn test_function_stats_tracking() {
        let config = TieringConfig::default();
        let manager = TieringManager::new(config);
        
        // Execute function multiple times
        for _ in 0..5 {
            manager.execute_function("stats_func", "console.log('stats')").await.unwrap();
        }
        
        // Get function statistics
        let function_stats = manager.get_function_stats("stats_func");
        assert!(function_stats.is_some());
        
        let stats = function_stats.unwrap();
        assert_eq!(stats.function_id, "stats_func");
        assert_eq!(stats.execution_count, 5);
        assert!(stats.total_time_us > 0);
        assert!(stats.avg_time_us > 0);
        assert_eq!(stats.current_tier, ExecutionTier::Interpreter);
        assert!(!stats.is_hot);
        assert!(!stats.is_optimized);
    }

    #[tokio::test]
    async fn test_hot_path_detection() {
        let mut config = TieringConfig::default();
        config.hot_threshold = 3;
        let manager = TieringManager::new(config);
        
        // Execute function to trigger hot path detection
        for _ in 0..5 {
            manager.execute_function("hot_path_func", "console.log('hot path')").await.unwrap();
        }
        
        let function_stats = manager.get_function_stats("hot_path_func");
        assert!(function_stats.is_some());
        
        let stats = function_stats.unwrap();
        assert!(stats.is_hot);
        assert_eq!(stats.execution_count, 5);
    }

    #[tokio::test]
    async fn test_optimization_detection() {
        let mut config = TieringConfig::default();
        config.hot_threshold = 3;
        config.optimization_threshold = 5;
        let manager = TieringManager::new(config);
        
        // Execute function to trigger optimization
        for _ in 0..7 {
            manager.execute_function("optimize_func", "console.log('optimize')").await.unwrap();
        }
        
        let function_stats = manager.get_function_stats("optimize_func");
        assert!(function_stats.is_some());
        
        let stats = function_stats.unwrap();
        assert!(stats.is_hot);
        assert!(stats.is_optimized);
        assert_eq!(stats.execution_count, 7);
    }

    #[tokio::test]
    async fn test_code_caching() {
        let config = TieringConfig::default();
        let manager = TieringManager::new(config);
        
        // Execute function in baseline tier to trigger compilation
        let mut config = TieringConfig::default();
        config.hot_threshold = 1;
        let manager = TieringManager::new(config);
        
        // First execution should be interpreter
        let result1 = manager.execute_function("cache_func", "console.log('cache')").await.unwrap();
        assert_eq!(result1.tier, ExecutionTier::Interpreter);
        
        // Second execution should be baseline (and cached)
        let result2 = manager.execute_function("cache_func", "console.log('cache')").await.unwrap();
        assert_eq!(result2.tier, ExecutionTier::Baseline);
        
        let stats = manager.get_stats();
        assert_eq!(stats.cached_functions, 1);
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let mut config = TieringConfig::default();
        config.max_cache_size = 3;
        let manager = TieringManager::new(config);
        
        // Fill the cache
        for i in 0..5 {
            let function_id = format!("func_{}", i);
            manager.execute_function(&function_id, "console.log('test')").await.unwrap();
        }
        
        let stats = manager.get_stats();
        // Cache should be at half capacity due to eviction
        assert!(stats.cached_functions <= 2);
    }

    #[tokio::test]
    async fn test_tiering_disabled() {
        let mut config = TieringConfig::default();
        config.enabled = false;
        let manager = TieringManager::new(config);
        
        // All executions should be in interpreter tier when disabled
        for _ in 0..10 {
            let result = manager.execute_function("disabled_func", "console.log('disabled')").await.unwrap();
            assert_eq!(result.tier, ExecutionTier::Interpreter);
        }
        
        let stats = manager.get_stats();
        assert_eq!(stats.interpreter_stats.functions_executed, 10);
        assert_eq!(stats.baseline_stats.functions_executed, 0);
        assert_eq!(stats.optimizing_stats.functions_executed, 0);
    }

    #[tokio::test]
    async fn test_compilation_queue_management() {
        let mut config = TieringConfig::default();
        config.hot_threshold = 2;
        config.optimization_threshold = 4;
        let manager = TieringManager::new(config);
        
        // Execute multiple functions to fill compilation queues
        for i in 0..3 {
            let function_id = format!("queue_func_{}", i);
            for _ in 0..5 {
                manager.execute_function(&function_id, "console.log('queue')").await.unwrap();
            }
        }
        
        let stats = manager.get_stats();
        assert!(stats.baseline_queue_size > 0);
        assert!(stats.optimizing_queue_size > 0);
    }

    #[tokio::test]
    async fn test_execution_time_tracking() {
        let config = TieringConfig::default();
        let manager = TieringManager::new(config);
        
        // Execute function and check timing
        let result = manager.execute_function("timing_func", "console.log('timing')").await.unwrap();
        
        assert!(result.execution_time_us > 0);
        assert!(result.execution_time_us < 10000); // Should be reasonable
        
        let function_stats = manager.get_function_stats("timing_func");
        assert!(function_stats.is_some());
        
        let stats = function_stats.unwrap();
        assert_eq!(stats.total_time_us, result.execution_time_us);
        assert_eq!(stats.avg_time_us, result.execution_time_us);
    }

    #[tokio::test]
    async fn test_engine_statistics() {
        let mut config = TieringConfig::default();
        config.hot_threshold = 2;
        config.optimization_threshold = 4;
        let manager = TieringManager::new(config);
        
        // Execute function multiple times to use different tiers
        for _ in 0..6 {
            manager.execute_function("stats_func", "console.log('stats')").await.unwrap();
        }
        
        let stats = manager.get_stats();
        
        // Check interpreter stats
        assert_eq!(stats.interpreter_stats.functions_executed, 2);
        assert!(stats.interpreter_stats.total_time_us > 0);
        assert!(stats.interpreter_stats.avg_time_per_function > 0);
        
        // Check baseline stats
        assert_eq!(stats.baseline_stats.functions_executed, 2);
        assert!(stats.baseline_stats.total_time_us > 0);
        assert!(stats.baseline_stats.avg_time_per_function > 0);
        assert!(stats.baseline_stats.compilation_count > 0);
        
        // Check optimizing stats
        assert_eq!(stats.optimizing_stats.functions_executed, 2);
        assert!(stats.optimizing_stats.total_time_us > 0);
        assert!(stats.optimizing_stats.avg_time_per_function > 0);
        assert!(stats.optimizing_stats.compilation_count > 0);
    }

    #[tokio::test]
    async fn test_tiering_clear() {
        let config = TieringConfig::default();
        let manager = TieringManager::new(config);
        
        // Add some data
        for i in 0..3 {
            let function_id = format!("clear_func_{}", i);
            manager.execute_function(&function_id, "console.log('clear')").await.unwrap();
        }
        
        // Verify data exists
        let stats_before = manager.get_stats();
        assert_eq!(stats_before.total_functions, 3);
        assert!(stats_before.interpreter_stats.functions_executed > 0);
        
        // Clear everything
        manager.clear();
        
        // Verify everything is cleared
        let stats_after = manager.get_stats();
        assert_eq!(stats_after.total_functions, 0);
        assert_eq!(stats_after.cached_functions, 0);
        assert_eq!(stats_after.interpreter_stats.functions_executed, 0);
        assert_eq!(stats_after.baseline_stats.functions_executed, 0);
        assert_eq!(stats_after.optimizing_stats.functions_executed, 0);
        assert_eq!(stats_after.baseline_queue_size, 0);
        assert_eq!(stats_after.optimizing_queue_size, 0);
    }

    #[tokio::test]
    async fn test_multiple_functions() {
        let mut config = TieringConfig::default();
        config.hot_threshold = 2;
        config.optimization_threshold = 4;
        let manager = TieringManager::new(config);
        
        // Execute multiple different functions
        let functions = vec!["func_a", "func_b", "func_c", "func_d"];
        
        for (i, func_id) in functions.iter().enumerate() {
            for _ in 0..(i + 1) * 2 {
                manager.execute_function(func_id, "console.log('multi')").await.unwrap();
            }
        }
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_functions, 4);
        
        // Check that each function has appropriate stats
        for func_id in functions {
            let function_stats = manager.get_function_stats(func_id);
            assert!(function_stats.is_some());
        }
    }

    #[tokio::test]
    async fn test_execution_tier_enum() {
        // Test enum functionality
        let interpreter = ExecutionTier::Interpreter;
        let baseline = ExecutionTier::Baseline;
        let optimizing = ExecutionTier::Optimizing;
        
        assert_ne!(interpreter, baseline);
        assert_ne!(baseline, optimizing);
        assert_ne!(interpreter, optimizing);
        
        // Test cloning
        let cloned = interpreter;
        assert_eq!(interpreter, cloned);
        
        // Test debug formatting
        assert_eq!(format!("{:?}", interpreter), "Interpreter");
        assert_eq!(format!("{:?}", baseline), "Baseline");
        assert_eq!(format!("{:?}", optimizing), "Optimizing");
    }

    #[tokio::test]
    async fn test_execution_result() {
        let result = ExecutionResult {
            function_id: "test".to_string(),
            tier: ExecutionTier::Interpreter,
            execution_time_us: 1000,
            success: true,
            result: "test result".to_string(),
        };
        
        assert_eq!(result.function_id, "test");
        assert_eq!(result.tier, ExecutionTier::Interpreter);
        assert_eq!(result.execution_time_us, 1000);
        assert!(result.success);
        assert_eq!(result.result, "test result");
    }

    #[tokio::test]
    async fn test_tiering_config_default() {
        let config = TieringConfig::default();
        
        assert_eq!(config.hot_threshold, 50);
        assert_eq!(config.optimization_threshold, 500);
        assert_eq!(config.max_cache_size, 1000);
        assert!(config.enabled);
        assert_eq!(config.promotion_delays.len(), 3);
        assert_eq!(config.promotion_delays.get(&ExecutionTier::Interpreter), Some(&10));
        assert_eq!(config.promotion_delays.get(&ExecutionTier::Baseline), Some(&100));
        assert_eq!(config.promotion_delays.get(&ExecutionTier::Optimizing), Some(&1000));
    }

    #[tokio::test]
    async fn test_tiering_integration() {
        let mut config = TieringConfig::default();
        config.hot_threshold = 3;
        config.optimization_threshold = 6;
        let manager = TieringManager::new(config);
        
        // Execute a function through all tiers
        let function_id = "integration_func";
        
        // First 3 executions: Interpreter
        for i in 0..3 {
            let result = manager.execute_function(function_id, "console.log('integration')").await.unwrap();
            assert_eq!(result.tier, ExecutionTier::Interpreter, "Execution {} should be interpreter", i);
        }
        
        // Next 3 executions: Baseline
        for i in 0..3 {
            let result = manager.execute_function(function_id, "console.log('integration')").await.unwrap();
            assert_eq!(result.tier, ExecutionTier::Baseline, "Execution {} should be baseline", i);
        }
        
        // Remaining executions: Optimizing
        for i in 0..3 {
            let result = manager.execute_function(function_id, "console.log('integration')").await.unwrap();
            assert_eq!(result.tier, ExecutionTier::Optimizing, "Execution {} should be optimizing", i);
        }
        
        // Verify final statistics
        let function_stats = manager.get_function_stats(function_id);
        assert!(function_stats.is_some());
        
        let stats = function_stats.unwrap();
        assert_eq!(stats.execution_count, 9);
        assert!(stats.is_hot);
        assert!(stats.is_optimized);
        assert_eq!(stats.current_tier, ExecutionTier::Optimizing);
        
        let overall_stats = manager.get_stats();
        assert_eq!(overall_stats.total_functions, 1);
        assert!(overall_stats.cached_functions > 0);
    }
}
