#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::{
        StackManager, StackAllocator, StackGuard, OperandStack, CallStack, StackFrame,
        FunctionValue, ClassValue, Value, ExceptionInfo, StackStats, PoolStats
    };

    #[tokio::test]
    async fn test_operand_stack_creation() {
        let mut stack = OperandStack::new(100);
        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
        assert_eq!(stack.max_size(), 100);
    }

    #[tokio::test]
    async fn test_operand_stack_push_pop() {
        let mut stack = OperandStack::new(10);
        
        // Push values
        stack.push(Value::Number(42.0)).unwrap();
        stack.push(Value::String("hello".to_string())).unwrap();
        stack.push(Value::Boolean(true)).unwrap();
        
        assert_eq!(stack.size(), 3);
        assert!(!stack.is_empty());
        
        // Pop values
        assert_eq!(stack.pop().unwrap(), Value::Boolean(true));
        assert_eq!(stack.pop().unwrap(), Value::String("hello".to_string()));
        assert_eq!(stack.pop().unwrap(), Value::Number(42.0));
        
        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
    }

    #[tokio::test]
    async fn test_operand_stack_peek() {
        let mut stack = OperandStack::new(10);
        stack.push(Value::Number(42.0)).unwrap();
        stack.push(Value::String("hello".to_string())).unwrap();
        
        assert_eq!(*stack.peek().unwrap(), Value::String("hello".to_string()));
        assert_eq!(*stack.peek_n(1).unwrap(), Value::Number(42.0));
        
        // Peek should not remove values
        assert_eq!(stack.size(), 2);
    }

    #[tokio::test]
    async fn test_operand_stack_overflow() {
        let mut stack = OperandStack::new(2);
        stack.push(Value::Number(1.0)).unwrap();
        stack.push(Value::Number(2.0)).unwrap();
        
        // This should fail
        let result = stack.push(Value::Number(3.0));
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_operand_stack_underflow() {
        let mut stack = OperandStack::new(10);
        let result = stack.pop();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_operand_stack_operations() {
        let mut stack = OperandStack::new(10);
        stack.push(Value::Number(1.0)).unwrap();
        stack.push(Value::Number(2.0)).unwrap();
        
        // Test dup
        stack.dup().unwrap();
        assert_eq!(stack.size(), 3);
        assert_eq!(*stack.peek().unwrap(), Value::Number(2.0));
        
        // Test swap
        stack.swap().unwrap();
        assert_eq!(*stack.peek().unwrap(), Value::Number(1.0));
        assert_eq!(*stack.peek_n(1).unwrap(), Value::Number(2.0));
        
        // Test rot (need 3 values)
        stack.push(Value::Number(3.0)).unwrap();
        stack.rot().unwrap();
        assert_eq!(*stack.peek().unwrap(), Value::Number(2.0));
        assert_eq!(*stack.peek_n(1).unwrap(), Value::Number(3.0));
        assert_eq!(*stack.peek_n(2).unwrap(), Value::Number(1.0));
    }

    #[tokio::test]
    async fn test_stack_frame_creation() {
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 2,
            local_count: 5,
            closure: std::collections::HashMap::new(),
        };
        
        let frame = StackFrame::new(function, Some(100));
        assert_eq!(frame.pc, 0);
        assert_eq!(frame.locals.len(), 5);
        assert_eq!(frame.return_address, Some(100));
        assert!(frame.this_value.is_none());
        assert!(frame.arguments.is_empty());
    }

    #[tokio::test]
    async fn test_stack_frame_locals() {
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 0,
            local_count: 3,
            closure: std::collections::HashMap::new(),
        };
        
        let mut frame = StackFrame::new(function, None);
        
        // Set and get locals
        frame.set_local(0, Value::Number(42.0)).unwrap();
        frame.set_local(1, Value::String("hello".to_string())).unwrap();
        
        assert_eq!(*frame.get_local(0).unwrap(), Value::Number(42.0));
        assert_eq!(*frame.get_local(1).unwrap(), Value::String("hello".to_string()));
        assert_eq!(*frame.get_local(2).unwrap(), Value::Undefined);
    }

    #[tokio::test]
    async fn test_stack_frame_arguments() {
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 2,
            local_count: 0,
            closure: std::collections::HashMap::new(),
        };
        
        let mut frame = StackFrame::new(function, None);
        let args = vec![Value::Number(1.0), Value::String("arg".to_string())];
        frame.set_arguments(args);
        
        assert_eq!(*frame.get_argument(0).unwrap(), Value::Number(1.0));
        assert_eq!(*frame.get_argument(1).unwrap(), Value::String("arg".to_string()));
    }

    #[tokio::test]
    async fn test_call_stack_creation() {
        let call_stack = CallStack::new(100);
        assert_eq!(call_stack.depth(), 0);
        assert!(call_stack.is_empty());
        assert_eq!(call_stack.max_depth(), 100);
    }

    #[tokio::test]
    async fn test_call_stack_push_pop() {
        let mut call_stack = CallStack::new(10);
        
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 0,
            local_count: 2,
            closure: std::collections::HashMap::new(),
        };
        
        let frame = StackFrame::new(function, Some(100));
        call_stack.push(frame).unwrap();
        
        assert_eq!(call_stack.depth(), 1);
        assert!(!call_stack.is_empty());
        
        let popped_frame = call_stack.pop().unwrap();
        assert_eq!(popped_frame.return_address, Some(100));
        assert_eq!(call_stack.depth(), 0);
        assert!(call_stack.is_empty());
    }

    #[tokio::test]
    async fn test_call_stack_peek() {
        let mut call_stack = CallStack::new(10);
        
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 0,
            local_count: 2,
            closure: std::collections::HashMap::new(),
        };
        
        let frame = StackFrame::new(function, Some(100));
        call_stack.push(frame).unwrap();
        
        let peeked_frame = call_stack.peek().unwrap();
        assert_eq!(peeked_frame.return_address, Some(100));
        assert_eq!(call_stack.depth(), 1); // Should not be removed
    }

    #[tokio::test]
    async fn test_call_stack_overflow() {
        let mut call_stack = CallStack::new(1);
        
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 0,
            local_count: 2,
            closure: std::collections::HashMap::new(),
        };
        
        let frame1 = StackFrame::new(function.clone(), None);
        call_stack.push(frame1).unwrap();
        
        let frame2 = StackFrame::new(function, None);
        let result = call_stack.push(frame2);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_stack_manager_creation() {
        let manager = StackManager::new(100, 1000);
        assert_eq!(manager.call_stack_depth(), 0);
        assert_eq!(manager.operand_stack_size(), 0);
        assert!(manager.is_empty());
    }

    #[tokio::test]
    async fn test_stack_manager_operations() {
        let mut manager = StackManager::new(10, 100);
        
        // Test operand stack operations
        manager.operand_stack_mut().push(Value::Number(42.0)).unwrap();
        assert_eq!(manager.operand_stack_size(), 1);
        
        // Test call stack operations
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 0,
            local_count: 2,
            closure: std::collections::HashMap::new(),
        };
        
        let frame = StackFrame::new(function, None);
        manager.call_stack_mut().push(frame).unwrap();
        assert_eq!(manager.call_stack_depth(), 1);
    }

    #[tokio::test]
    async fn test_stack_manager_exception_handling() {
        let mut manager = StackManager::new(10, 100);
        
        let exception_info = ExceptionInfo {
            exception: Value::String("error".to_string()),
            catch_address: 100,
            finally_address: Some(200),
            frame_depth: 1,
        };
        
        manager.push_exception_handler(exception_info.clone());
        assert!(manager.current_exception_handler().is_some());
        
        let popped = manager.pop_exception_handler();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().catch_address, 100);
    }

    #[tokio::test]
    async fn test_stack_manager_stats() {
        let mut manager = StackManager::new(100, 1000);
        
        // Add some data
        manager.operand_stack_mut().push(Value::Number(42.0)).unwrap();
        
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 0,
            local_count: 2,
            closure: std::collections::HashMap::new(),
        };
        
        let frame = StackFrame::new(function, None);
        manager.call_stack_mut().push(frame).unwrap();
        
        let stats = manager.get_stats();
        assert_eq!(stats.call_stack_depth, 1);
        assert_eq!(stats.operand_stack_size, 1);
        assert_eq!(stats.exception_handlers, 0);
        assert_eq!(stats.max_call_depth, 100);
        assert_eq!(stats.max_operand_stack_size, 1000);
    }

    #[tokio::test]
    async fn test_stack_allocator_creation() {
        let allocator = StackAllocator::new(10);
        let stats = allocator.get_pool_stats();
        assert_eq!(stats.frame_pool_size, 0);
        assert_eq!(stats.operand_stack_pool_size, 0);
        assert_eq!(stats.max_pool_size, 10);
    }

    #[tokio::test]
    async fn test_stack_allocator_frame_allocation() {
        let mut allocator = StackAllocator::new(5);
        
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 0,
            local_count: 2,
            closure: std::collections::HashMap::new(),
        };
        
        // Allocate frame
        let frame = allocator.allocate_frame(function.clone(), Some(100));
        assert_eq!(frame.function.name, "test");
        assert_eq!(frame.return_address, Some(100));
        
        // Deallocate frame
        allocator.deallocate_frame(frame);
        let stats = allocator.get_pool_stats();
        assert_eq!(stats.frame_pool_size, 1);
    }

    #[tokio::test]
    async fn test_stack_allocator_operand_stack_allocation() {
        let mut allocator = StackAllocator::new(5);
        
        // Allocate operand stack
        let mut stack = allocator.allocate_operand_stack(100);
        stack.push(Value::Number(42.0)).unwrap();
        
        // Deallocate operand stack
        allocator.deallocate_operand_stack(stack);
        let stats = allocator.get_pool_stats();
        assert_eq!(stats.operand_stack_pool_size, 1);
    }

    #[tokio::test]
    async fn test_stack_guard_creation() {
        let guard = StackGuard::new(10);
        assert_eq!(guard.current_depth(), 0);
        assert_eq!(guard.max_depth(), 10);
        assert!(!guard.is_at_max_depth());
    }

    #[tokio::test]
    async fn test_stack_guard_operations() {
        let mut guard = StackGuard::new(3);
        
        // Enter levels
        guard.enter().unwrap();
        assert_eq!(guard.current_depth(), 1);
        
        guard.enter().unwrap();
        assert_eq!(guard.current_depth(), 2);
        
        guard.enter().unwrap();
        assert_eq!(guard.current_depth(), 3);
        assert!(guard.is_at_max_depth());
        
        // This should fail
        let result = guard.enter();
        assert!(result.is_err());
        
        // Exit levels
        guard.exit();
        assert_eq!(guard.current_depth(), 2);
        
        guard.exit();
        assert_eq!(guard.current_depth(), 1);
        
        guard.exit();
        assert_eq!(guard.current_depth(), 0);
    }

    #[tokio::test]
    async fn test_stack_guard_reset() {
        let mut guard = StackGuard::new(10);
        guard.enter().unwrap();
        guard.enter().unwrap();
        assert_eq!(guard.current_depth(), 2);
        
        guard.reset();
        assert_eq!(guard.current_depth(), 0);
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
            Value::Object(std::collections::HashMap::new()),
            Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]),
        ];
        
        let mut stack = OperandStack::new(10);
        for value in values {
            stack.push(value.clone()).unwrap();
            let popped = stack.pop().unwrap();
            assert_eq!(popped, value);
        }
    }

    #[tokio::test]
    async fn test_function_value_operations() {
        let function = FunctionValue {
            name: "test_function".to_string(),
            param_count: 3,
            local_count: 5,
            closure: {
                let mut map = std::collections::HashMap::new();
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
            methods: std::collections::HashMap::new(),
            static_methods: std::collections::HashMap::new(),
            properties: {
                let mut map = std::collections::HashMap::new();
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
    async fn test_stack_integration() {
        let mut manager = StackManager::new(10, 100);
        let mut allocator = StackAllocator::new(5);
        let mut guard = StackGuard::new(10);
        
        // Create a function
        let function = FunctionValue {
            name: "test".to_string(),
            param_count: 2,
            local_count: 3,
            closure: std::collections::HashMap::new(),
        };
        
        // Enter stack level
        guard.enter().unwrap();
        
        // Allocate frame
        let frame = allocator.allocate_frame(function, Some(100));
        
        // Push to call stack
        manager.call_stack_mut().push(frame).unwrap();
        
        // Push to operand stack
        manager.operand_stack_mut().push(Value::Number(42.0)).unwrap();
        
        // Verify state
        assert_eq!(guard.current_depth(), 1);
        assert_eq!(manager.call_stack_depth(), 1);
        assert_eq!(manager.operand_stack_size(), 1);
        
        // Cleanup
        guard.exit();
        let _ = manager.call_stack_mut().pop().unwrap();
        manager.operand_stack_mut().clear();
        
        assert_eq!(guard.current_depth(), 0);
        assert_eq!(manager.call_stack_depth(), 0);
        assert_eq!(manager.operand_stack_size(), 0);
    }
}
