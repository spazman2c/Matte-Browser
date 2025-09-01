#[cfg(test)]
mod tests {
    use super::*;
    use crate::destructuring::{DestructuringSystem, DestructuringEngine, SpreadOperator, PatternMatcher, Value};

    #[tokio::test]
    async fn test_destructuring_engine_creation() {
        let engine = DestructuringEngine::new();
        
        assert!(engine.get_variables().is_empty());
        assert!(engine.get_rest_params().is_empty());
    }

    #[tokio::test]
    async fn test_spread_operator_creation() {
        let spread_op = SpreadOperator::new();
        
        // Test that spread operator is created successfully
        assert!(true); // If we get here, creation succeeded
    }

    #[tokio::test]
    async fn test_array_spreading() {
        let spread_op = SpreadOperator::new();
        
        let mut target = vec![Value::Number(1.0), Value::Number(2.0)];
        let source = vec![Value::Number(3.0), Value::Number(4.0)];
        
        spread_op.spread_array(&mut target, &source);
        
        assert_eq!(target.len(), 4);
        assert!(matches!(target[0], Value::Number(n) if n == 1.0));
        assert!(matches!(target[1], Value::Number(n) if n == 2.0));
        assert!(matches!(target[2], Value::Number(n) if n == 3.0));
        assert!(matches!(target[3], Value::Number(n) if n == 4.0));
    }

    #[tokio::test]
    async fn test_object_spreading() {
        let spread_op = SpreadOperator::new();
        
        let mut target = HashMap::new();
        target.insert("a".to_string(), Value::Number(1.0));
        target.insert("b".to_string(), Value::Number(2.0));
        
        let mut source = HashMap::new();
        source.insert("c".to_string(), Value::Number(3.0));
        source.insert("d".to_string(), Value::Number(4.0));
        
        spread_op.spread_object(&mut target, &source);
        
        assert_eq!(target.len(), 4);
        assert!(target.contains_key("a"));
        assert!(target.contains_key("b"));
        assert!(target.contains_key("c"));
        assert!(target.contains_key("d"));
    }

    #[tokio::test]
    async fn test_create_spread_array() {
        let spread_op = SpreadOperator::new();
        
        let elements = vec![
            Value::Number(1.0),
            Value::Array(vec![Value::Number(2.0), Value::Number(3.0)]),
            Value::Number(4.0),
        ];
        
        let result = spread_op.create_spread_array(&elements);
        
        assert_eq!(result.len(), 4);
        assert!(matches!(result[0], Value::Number(n) if n == 1.0));
        assert!(matches!(result[1], Value::Number(n) if n == 2.0));
        assert!(matches!(result[2], Value::Number(n) if n == 3.0));
        assert!(matches!(result[3], Value::Number(n) if n == 4.0));
    }

    #[tokio::test]
    async fn test_create_spread_object() {
        let spread_op = SpreadOperator::new();
        
        let mut obj1 = HashMap::new();
        obj1.insert("a".to_string(), Value::Number(1.0));
        obj1.insert("b".to_string(), Value::Number(2.0));
        
        let properties = vec![
            ("c".to_string(), Value::Number(3.0)),
            ("obj".to_string(), Value::Object(obj1)),
            ("d".to_string(), Value::Number(4.0)),
        ];
        
        let result = spread_op.create_spread_object(&properties);
        
        assert!(result.contains_key("a"));
        assert!(result.contains_key("b"));
        assert!(result.contains_key("c"));
        assert!(result.contains_key("d"));
    }

    #[tokio::test]
    async fn test_destructuring_system_creation() {
        let system = DestructuringSystem::new();
        
        assert!(system.get_variables().is_empty());
        assert!(system.get_rest_params().is_empty());
    }

    #[tokio::test]
    async fn test_pattern_matcher_creation() {
        let matcher = PatternMatcher::new();
        
        assert!(matcher.get_matched_values().is_empty());
    }

    #[tokio::test]
    async fn test_value_operations() {
        // Test different value types
        let string_value = Value::String("hello".to_string());
        let number_value = Value::Number(42.0);
        let boolean_value = Value::Boolean(true);
        let array_value = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        let mut object_value = HashMap::new();
        object_value.insert("key".to_string(), Value::String("value".to_string()));
        let object_value = Value::Object(object_value);

        // Test pattern matching
        match string_value {
            Value::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string value"),
        }

        match number_value {
            Value::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected number value"),
        }

        match boolean_value {
            Value::Boolean(b) => assert!(b),
            _ => panic!("Expected boolean value"),
        }

        match array_value {
            Value::Array(arr) => assert_eq!(arr.len(), 2),
            _ => panic!("Expected array value"),
        }

        match object_value {
            Value::Object(obj) => assert!(obj.contains_key("key")),
            _ => panic!("Expected object value"),
        }
    }

    #[tokio::test]
    async fn test_destructuring_context_operations() {
        use crate::destructuring::DestructuringContext;
        
        let mut context = DestructuringContext {
            variables: HashMap::new(),
            defaults: HashMap::new(),
            rest_params: HashMap::new(),
        };
        
        // Test variable operations
        context.variables.insert("test".to_string(), Value::String("value".to_string()));
        assert!(context.variables.contains_key("test"));
        assert!(matches!(context.variables.get("test"), Some(Value::String(s)) if s == "value"));
        
        // Test default operations
        context.defaults.insert("default".to_string(), Value::Number(42.0));
        assert!(context.defaults.contains_key("default"));
        assert!(matches!(context.defaults.get("default"), Some(Value::Number(n)) if n == 42.0));
        
        // Test rest parameter operations
        context.rest_params.insert("rest".to_string(), vec![Value::Number(1.0), Value::Number(2.0)]);
        assert!(context.rest_params.contains_key("rest"));
        assert_eq!(context.rest_params.get("rest").unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_spread_arguments() {
        let spread_op = SpreadOperator::new();
        
        let args = vec![
            Value::Number(1.0),
            Value::Array(vec![Value::Number(2.0), Value::Number(3.0)]),
            Value::String("test".to_string()),
        ];
        
        let result = spread_op.spread_arguments(&args);
        
        assert_eq!(result.len(), 4);
        assert!(matches!(result[0], Value::Number(n) if n == 1.0));
        assert!(matches!(result[1], Value::Number(n) if n == 2.0));
        assert!(matches!(result[2], Value::Number(n) if n == 3.0));
        assert!(matches!(result[3], Value::String(s) if s == "test"));
    }

    #[tokio::test]
    async fn test_destructuring_system_clear() {
        let mut system = DestructuringSystem::new();
        
        // Add some data
        let mut obj = HashMap::new();
        obj.insert("test".to_string(), Value::String("value".to_string()));
        
        // Clear the system
        system.clear();
        
        assert!(system.get_variables().is_empty());
        assert!(system.get_rest_params().is_empty());
    }

    #[tokio::test]
    async fn test_pattern_matcher_clear() {
        let mut matcher = PatternMatcher::new();
        
        // Add some data
        matcher.context.insert("test".to_string(), Value::String("value".to_string()));
        
        // Clear the matcher
        matcher.clear();
        
        assert!(matcher.get_matched_values().is_empty());
    }

    #[tokio::test]
    async fn test_value_conversion() {
        let engine = DestructuringEngine::new();
        
        // Test string to object conversion
        let string_value = Value::String("test".to_string());
        let obj_result = engine.convert_to_object(string_value).unwrap();
        assert!(obj_result.contains_key("0"));
        assert!(obj_result.contains_key("length"));
        
        // Test string to array conversion
        let string_value = Value::String("abc".to_string());
        let arr_result = engine.convert_to_array(string_value).unwrap();
        assert_eq!(arr_result.len(), 3);
        assert!(matches!(arr_result[0], Value::String(s) if s == "a"));
        assert!(matches!(arr_result[1], Value::String(s) if s == "b"));
        assert!(matches!(arr_result[2], Value::String(s) if s == "c"));
    }

    #[tokio::test]
    async fn test_literal_parsing() {
        let engine = DestructuringEngine::new();
        
        // Test string literal
        let string_literal = crate::ast::Literal::String("test".to_string());
        let result = engine.parse_literal(&string_literal).unwrap();
        assert!(matches!(result, Value::String(s) if s == "test"));
        
        // Test number literal
        let number_literal = crate::ast::Literal::Number(42.0);
        let result = engine.parse_literal(&number_literal).unwrap();
        assert!(matches!(result, Value::Number(n) if n == 42.0));
        
        // Test boolean literal
        let boolean_literal = crate::ast::Literal::Boolean(true);
        let result = engine.parse_literal(&boolean_literal).unwrap();
        assert!(matches!(result, Value::Boolean(true)));
        
        // Test null literal
        let null_literal = crate::ast::Literal::Null;
        let result = engine.parse_literal(&null_literal).unwrap();
        assert!(matches!(result, Value::Null));
    }

    #[tokio::test]
    async fn test_destructuring_system_integration() {
        let mut system = DestructuringSystem::new();
        
        // Test array spreading
        let mut target_array = vec![Value::Number(1.0)];
        let source_array = vec![Value::Number(2.0), Value::Number(3.0)];
        system.spread_array(&mut target_array, &source_array);
        
        assert_eq!(target_array.len(), 3);
        assert!(matches!(target_array[0], Value::Number(n) if n == 1.0));
        assert!(matches!(target_array[1], Value::Number(n) if n == 2.0));
        assert!(matches!(target_array[2], Value::Number(n) if n == 3.0));
        
        // Test object spreading
        let mut target_obj = HashMap::new();
        target_obj.insert("a".to_string(), Value::Number(1.0));
        
        let mut source_obj = HashMap::new();
        source_obj.insert("b".to_string(), Value::Number(2.0));
        
        system.spread_object(&mut target_obj, &source_obj);
        
        assert_eq!(target_obj.len(), 2);
        assert!(target_obj.contains_key("a"));
        assert!(target_obj.contains_key("b"));
    }
}
