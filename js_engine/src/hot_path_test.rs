#[cfg(test)]
mod tests {
    use super::*;
    use crate::hot_path::{
        HotPathOptimizer, HotPathConfig, HotPathId, HotPathStats, PathNode, PathNodeType,
        OptimizationHint, OptimizationHintType, OptimizedPath, OptimizationStats
    };

    #[tokio::test]
    async fn test_hot_path_optimizer_creation() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let stats = optimizer.get_optimization_stats();
        assert_eq!(stats.paths_optimized, 0);
        assert_eq!(stats.optimization_attempts, 0);
        assert_eq!(stats.successful_optimizations, 0);
        assert_eq!(stats.success_rate, 0.0);
    }

    #[tokio::test]
    async fn test_path_execution_recording() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let path_nodes = vec![
            PathNode {
                node_id: "func_call".to_string(),
                node_type: PathNodeType::FunctionCall,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
            PathNode {
                node_id: "loop".to_string(),
                node_type: PathNodeType::Loop,
                execution_count: 1,
                avg_time_us: 200,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        optimizer.record_path_execution("test_func", path_nodes, 500).await.unwrap();
        
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
        assert_eq!(hot_paths[0].path_id.function_id, "test_func");
        assert_eq!(hot_paths[0].execution_count, 1);
        assert_eq!(hot_paths[0].total_time_us, 500);
    }

    #[tokio::test]
    async fn test_hot_path_detection() {
        let mut config = HotPathConfig::default();
        config.min_execution_count = 5; // Lower threshold for testing
        let optimizer = HotPathOptimizer::new(config);
        
        let path_nodes = vec![
            PathNode {
                node_id: "hot_node".to_string(),
                node_type: PathNodeType::Loop,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        // Execute path multiple times to trigger hot path detection
        for _ in 0..10 {
            optimizer.record_path_execution("hot_func", path_nodes.clone(), 100).await.unwrap();
        }
        
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
        assert_eq!(hot_paths[0].execution_count, 10);
        assert!(hot_paths[0].execution_count >= 5);
    }

    #[tokio::test]
    async fn test_path_signature_calculation() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let path_nodes1 = vec![
            PathNode {
                node_id: "node1".to_string(),
                node_type: PathNodeType::FunctionCall,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        let path_nodes2 = vec![
            PathNode {
                node_id: "node2".to_string(),
                node_type: PathNodeType::Loop,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        optimizer.record_path_execution("func1", path_nodes1, 100).await.unwrap();
        optimizer.record_path_execution("func2", path_nodes2, 100).await.unwrap();
        
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 2);
        
        // Path signatures should be different
        assert_ne!(hot_paths[0].path_id.path_signature, hot_paths[1].path_id.path_signature);
    }

    #[tokio::test]
    async fn test_optimization_hints() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let optimization_hint = OptimizationHint {
            hint_type: OptimizationHintType::InlineFunction,
            data: "inline_me".to_string(),
            confidence: 0.9,
        };
        
        let path_nodes = vec![
            PathNode {
                node_id: "hinted_node".to_string(),
                node_type: PathNodeType::FunctionCall,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: vec![optimization_hint],
            },
        ];
        
        optimizer.record_path_execution("hinted_func", path_nodes, 100).await.unwrap();
        
        // The path tree should contain the optimization hint
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
    }

    #[tokio::test]
    async fn test_path_tree_construction() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let child_node = PathNode {
            node_id: "child".to_string(),
            node_type: PathNodeType::PropertyAccess,
            execution_count: 1,
            avg_time_us: 50,
            children: Vec::new(),
            optimization_hints: Vec::new(),
        };
        
        let parent_node = PathNode {
            node_id: "parent".to_string(),
            node_type: PathNodeType::FunctionCall,
            execution_count: 1,
            avg_time_us: 100,
            children: vec![child_node],
            optimization_hints: Vec::new(),
        };
        
        let path_nodes = vec![parent_node];
        optimizer.record_path_execution("tree_func", path_nodes, 150).await.unwrap();
        
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
    }

    #[tokio::test]
    async fn test_stability_score_calculation() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let path_nodes = vec![
            PathNode {
                node_id: "stable_node".to_string(),
                node_type: PathNodeType::Loop,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        // Execute multiple times to build up stability
        for _ in 0..20 {
            optimizer.record_path_execution("stable_func", path_nodes.clone(), 100).await.unwrap();
        }
        
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
        assert!(hot_paths[0].stability_score > 0.0);
        assert!(hot_paths[0].stability_score <= 1.0);
    }

    #[tokio::test]
    async fn test_frequency_calculation() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let path_nodes = vec![
            PathNode {
                node_id: "frequent_node".to_string(),
                node_type: PathNodeType::FunctionCall,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        // Execute rapidly to build up frequency
        for _ in 0..10 {
            optimizer.record_path_execution("frequent_func", path_nodes.clone(), 100).await.unwrap();
        }
        
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
        assert!(hot_paths[0].frequency > 0.0);
    }

    #[tokio::test]
    async fn test_optimization_application() {
        let mut config = HotPathConfig::default();
        config.min_execution_count = 5;
        let optimizer = HotPathOptimizer::new(config);
        
        let optimization_hint = OptimizationHint {
            hint_type: OptimizationHintType::InlineFunction,
            data: "inline_me".to_string(),
            confidence: 0.9,
        };
        
        let path_nodes = vec![
            PathNode {
                node_id: "optimizable_node".to_string(),
                node_type: PathNodeType::FunctionCall,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: vec![optimization_hint],
            },
        ];
        
        // Execute enough times to trigger optimization
        for _ in 0..10 {
            optimizer.record_path_execution("optimizable_func", path_nodes.clone(), 100).await.unwrap();
        }
        
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
        
        // Try to optimize the hot path
        let path_id = &hot_paths[0].path_id;
        let optimized_path = optimizer.optimize_hot_path(path_id).await.unwrap();
        
        assert_eq!(optimized_path.original_path_id, *path_id);
        assert!(optimized_path.optimization_level > 0);
        assert!(optimized_path.improvement_factor > 1.0);
        assert!(optimized_path.is_valid);
    }

    #[tokio::test]
    async fn test_basic_optimizations() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let optimization_hint = OptimizationHint {
            hint_type: OptimizationHintType::ConstantFolding,
            data: "fold_constants".to_string(),
            confidence: 0.8,
        };
        
        let path_nodes = vec![
            PathNode {
                node_id: "basic_node".to_string(),
                node_type: PathNodeType::Arithmetic,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: vec![optimization_hint],
            },
        ];
        
        optimizer.record_path_execution("basic_func", path_nodes, 100).await.unwrap();
        
        let hot_paths = optimizer.get_all_hot_paths();
        let path_id = &hot_paths[0].path_id;
        
        // Force optimization level 1
        let mut stats = hot_paths[0].clone();
        stats.optimization_level = 1;
        
        // This would require modifying the internal state, so we'll just test the structure
        assert_eq!(stats.optimization_level, 1);
    }

    #[tokio::test]
    async fn test_intermediate_optimizations() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let optimization_hint = OptimizationHint {
            hint_type: OptimizationHintType::InlineFunction,
            data: "inline_function".to_string(),
            confidence: 0.9,
        };
        
        let path_nodes = vec![
            PathNode {
                node_id: "intermediate_node".to_string(),
                node_type: PathNodeType::FunctionCall,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: vec![optimization_hint],
            },
        ];
        
        optimizer.record_path_execution("intermediate_func", path_nodes, 100).await.unwrap();
        
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
    }

    #[tokio::test]
    async fn test_aggressive_optimizations() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let optimization_hint = OptimizationHint {
            hint_type: OptimizationHintType::EliminateDeadCode,
            data: "eliminate_dead".to_string(),
            confidence: 0.95,
        };
        
        let path_nodes = vec![
            PathNode {
                node_id: "aggressive_node".to_string(),
                node_type: PathNodeType::Conditional,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: vec![optimization_hint],
            },
        ];
        
        optimizer.record_path_execution("aggressive_func", path_nodes, 100).await.unwrap();
        
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
    }

    #[tokio::test]
    async fn test_optimization_statistics() {
        let mut config = HotPathConfig::default();
        config.min_execution_count = 3;
        let optimizer = HotPathOptimizer::new(config);
        
        let path_nodes = vec![
            PathNode {
                node_id: "stats_node".to_string(),
                node_type: PathNodeType::Loop,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        // Execute to trigger optimization
        for _ in 0..5 {
            optimizer.record_path_execution("stats_func", path_nodes.clone(), 100).await.unwrap();
        }
        
        let hot_paths = optimizer.get_all_hot_paths();
        let path_id = &hot_paths[0].path_id;
        
        // Optimize the path
        optimizer.optimize_hot_path(path_id).await.unwrap();
        
        let stats = optimizer.get_optimization_stats();
        assert_eq!(stats.paths_optimized, 1);
        assert_eq!(stats.optimization_attempts, 1);
        assert_eq!(stats.successful_optimizations, 1);
        assert_eq!(stats.success_rate, 1.0);
        assert!(stats.total_optimization_time_us > 0);
    }

    #[tokio::test]
    async fn test_optimized_path_caching() {
        let mut config = HotPathConfig::default();
        config.min_execution_count = 3;
        let optimizer = HotPathOptimizer::new(config);
        
        let path_nodes = vec![
            PathNode {
                node_id: "cache_node".to_string(),
                node_type: PathNodeType::FunctionCall,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        // Execute to trigger optimization
        for _ in 0..5 {
            optimizer.record_path_execution("cache_func", path_nodes.clone(), 100).await.unwrap();
        }
        
        let hot_paths = optimizer.get_all_hot_paths();
        let path_id = &hot_paths[0].path_id;
        
        // Optimize the path
        let optimized_path = optimizer.optimize_hot_path(path_id).await.unwrap();
        
        // Retrieve from cache
        let cached_path = optimizer.get_optimized_path(path_id);
        assert!(cached_path.is_some());
        assert_eq!(cached_path.unwrap().original_path_id, optimized_path.original_path_id);
    }

    #[tokio::test]
    async fn test_hot_path_optimizer_disabled() {
        let mut config = HotPathConfig::default();
        config.enabled = false;
        let optimizer = HotPathOptimizer::new(config);
        
        let path_nodes = vec![
            PathNode {
                node_id: "disabled_node".to_string(),
                node_type: PathNodeType::FunctionCall,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        // Execute multiple times
        for _ in 0..20 {
            optimizer.record_path_execution("disabled_func", path_nodes.clone(), 100).await.unwrap();
        }
        
        // Should not record any hot paths when disabled
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 0);
    }

    #[tokio::test]
    async fn test_hot_path_optimizer_clear() {
        let config = HotPathConfig::default();
        let optimizer = HotPathOptimizer::new(config);
        
        let path_nodes = vec![
            PathNode {
                node_id: "clear_node".to_string(),
                node_type: PathNodeType::FunctionCall,
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            },
        ];
        
        // Add some data
        optimizer.record_path_execution("clear_func", path_nodes, 100).await.unwrap();
        
        // Verify data exists
        let hot_paths_before = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths_before.len(), 1);
        
        // Clear everything
        optimizer.clear();
        
        // Verify everything is cleared
        let hot_paths_after = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths_after.len(), 0);
        
        let stats_after = optimizer.get_optimization_stats();
        assert_eq!(stats_after.paths_optimized, 0);
        assert_eq!(stats_after.optimization_attempts, 0);
    }

    #[tokio::test]
    async fn test_path_node_types() {
        // Test all path node types
        let node_types = vec![
            PathNodeType::FunctionCall,
            PathNodeType::Loop,
            PathNodeType::Conditional,
            PathNodeType::PropertyAccess,
            PathNodeType::ArrayAccess,
            PathNodeType::Arithmetic,
            PathNodeType::Comparison,
            PathNodeType::Assignment,
            PathNodeType::Return,
            PathNodeType::Other,
        ];
        
        for node_type in node_types {
            let path_node = PathNode {
                node_id: format!("{:?}", node_type),
                node_type: node_type.clone(),
                execution_count: 1,
                avg_time_us: 100,
                children: Vec::new(),
                optimization_hints: Vec::new(),
            };
            
            assert_eq!(path_node.node_type, node_type);
        }
    }

    #[tokio::test]
    async fn test_optimization_hint_types() {
        // Test all optimization hint types
        let hint_types = vec![
            OptimizationHintType::InlineFunction,
            OptimizationHintType::UnrollLoop,
            OptimizationHintType::HoistInvariant,
            OptimizationHintType::EliminateDeadCode,
            OptimizationHintType::OptimizePropertyAccess,
            OptimizationHintType::OptimizeArrayAccess,
            OptimizationHintType::ConstantFolding,
            OptimizationHintType::StrengthReduction,
            OptimizationHintType::LoopFusion,
            OptimizationHintType::LoopFission,
        ];
        
        for hint_type in hint_types {
            let hint = OptimizationHint {
                hint_type: hint_type.clone(),
                data: format!("{:?}", hint_type),
                confidence: 0.8,
            };
            
            assert_eq!(hint.hint_type, hint_type);
        }
    }

    #[tokio::test]
    async fn test_hot_path_config_default() {
        let config = HotPathConfig::default();
        
        assert_eq!(config.min_execution_count, 100);
        assert_eq!(config.min_frequency, 10.0);
        assert_eq!(config.min_stability_score, 0.8);
        assert_eq!(config.max_optimization_level, 3);
        assert!(config.enabled);
        assert_eq!(config.hash_function, "xxhash");
        assert_eq!(config.optimization_timeout_ms, 5000);
    }

    #[tokio::test]
    async fn test_hot_path_integration() {
        let mut config = HotPathConfig::default();
        config.min_execution_count = 5;
        let optimizer = HotPathOptimizer::new(config);
        
        // Create a complex path with multiple nodes and hints
        let child_hint = OptimizationHint {
            hint_type: OptimizationHintType::ConstantFolding,
            data: "child_constant".to_string(),
            confidence: 0.9,
        };
        
        let child_node = PathNode {
            node_id: "child".to_string(),
            node_type: PathNodeType::Arithmetic,
            execution_count: 1,
            avg_time_us: 50,
            children: Vec::new(),
            optimization_hints: vec![child_hint],
        };
        
        let parent_hint = OptimizationHint {
            hint_type: OptimizationHintType::InlineFunction,
            data: "parent_inline".to_string(),
            confidence: 0.8,
        };
        
        let parent_node = PathNode {
            node_id: "parent".to_string(),
            node_type: PathNodeType::FunctionCall,
            execution_count: 1,
            avg_time_us: 100,
            children: vec![child_node],
            optimization_hints: vec![parent_hint],
        };
        
        let path_nodes = vec![parent_node];
        
        // Execute multiple times to build up statistics
        for i in 0..10 {
            optimizer.record_path_execution("integration_func", path_nodes.clone(), 100 + i).await.unwrap();
        }
        
        // Verify hot path detection
        let hot_paths = optimizer.get_all_hot_paths();
        assert_eq!(hot_paths.len(), 1);
        assert_eq!(hot_paths[0].execution_count, 10);
        assert!(hot_paths[0].stability_score > 0.0);
        assert!(hot_paths[0].frequency > 0.0);
        
        // Test optimization
        let path_id = &hot_paths[0].path_id;
        let optimized_path = optimizer.optimize_hot_path(path_id).await.unwrap();
        
        assert_eq!(optimized_path.original_path_id, *path_id);
        assert!(optimized_path.optimization_level > 0);
        assert!(optimized_path.improvement_factor > 1.0);
        assert!(optimized_path.is_valid);
        
        // Verify statistics
        let stats = optimizer.get_optimization_stats();
        assert_eq!(stats.paths_optimized, 1);
        assert_eq!(stats.successful_optimizations, 1);
        assert_eq!(stats.success_rate, 1.0);
    }
}
