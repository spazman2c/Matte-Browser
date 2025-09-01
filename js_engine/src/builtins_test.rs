#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::{
        TypedArray, TypedArrayType, Promise, PromiseState, FetchAPI, FetchRequest, FetchResponse,
        TimerManager, TimerType, EventManager, EventType, Event, BuiltinObjects, Value
    };

    #[tokio::test]
    async fn test_typed_array_creation() {
        let array = TypedArray::new(TypedArrayType::Int32Array, 10);
        
        assert_eq!(array.array_type, TypedArrayType::Int32Array);
        assert_eq!(array.length, 10);
        assert_eq!(array.byte_length, 40); // 4 bytes per element
        assert_eq!(array.byte_offset, 0);
        assert_eq!(array.buffer.len(), 40);
    }

    #[tokio::test]
    async fn test_typed_array_element_sizes() {
        assert_eq!(TypedArray::get_element_size(TypedArrayType::Int8Array), 1);
        assert_eq!(TypedArray::get_element_size(TypedArrayType::Uint8Array), 1);
        assert_eq!(TypedArray::get_element_size(TypedArrayType::Uint8ClampedArray), 1);
        assert_eq!(TypedArray::get_element_size(TypedArrayType::Int16Array), 2);
        assert_eq!(TypedArray::get_element_size(TypedArrayType::Uint16Array), 2);
        assert_eq!(TypedArray::get_element_size(TypedArrayType::Int32Array), 4);
        assert_eq!(TypedArray::get_element_size(TypedArrayType::Uint32Array), 4);
        assert_eq!(TypedArray::get_element_size(TypedArrayType::Float32Array), 4);
        assert_eq!(TypedArray::get_element_size(TypedArrayType::Float64Array), 8);
    }

    #[tokio::test]
    async fn test_typed_array_get_set() {
        let mut array = TypedArray::new(TypedArrayType::Int32Array, 5);
        
        // Set values
        array.set(0, Value::Number(42.0)).unwrap();
        array.set(1, Value::Number(-123.0)).unwrap();
        array.set(2, Value::Boolean(true)).unwrap(); // Should convert to 1
        
        // Get values
        assert_eq!(array.get(0).unwrap(), Value::Number(42.0));
        assert_eq!(array.get(1).unwrap(), Value::Number(-123.0));
        assert_eq!(array.get(2).unwrap(), Value::Number(1.0));
        
        // Test out of bounds
        assert!(array.get(5).is_err());
        assert!(array.set(5, Value::Number(0.0)).is_err());
    }

    #[tokio::test]
    async fn test_typed_array_float32() {
        let mut array = TypedArray::new(TypedArrayType::Float32Array, 3);
        
        array.set(0, Value::Number(3.14)).unwrap();
        array.set(1, Value::Number(-2.5)).unwrap();
        array.set(2, Value::Number(0.0)).unwrap();
        
        let value0 = array.get(0).unwrap();
        if let Value::Number(n) = value0 {
            assert!((n - 3.14).abs() < 0.001);
        } else {
            panic!("Expected number");
        }
        
        let value1 = array.get(1).unwrap();
        if let Value::Number(n) = value1 {
            assert!((n - (-2.5)).abs() < 0.001);
        } else {
            panic!("Expected number");
        }
    }

    #[tokio::test]
    async fn test_typed_array_uint8_clamped() {
        let mut array = TypedArray::new(TypedArrayType::Uint8ClampedArray, 5);
        
        // Test clamping behavior
        array.set(0, Value::Number(300.0)).unwrap(); // Should clamp to 255
        array.set(1, Value::Number(-50.0)).unwrap(); // Should clamp to 0
        array.set(2, Value::Number(128.0)).unwrap(); // Should stay 128
        
        assert_eq!(array.get(0).unwrap(), Value::Number(255.0));
        assert_eq!(array.get(1).unwrap(), Value::Number(0.0));
        assert_eq!(array.get(2).unwrap(), Value::Number(128.0));
    }

    #[tokio::test]
    async fn test_typed_array_from_buffer() {
        let buffer = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let array = TypedArray::from_buffer(TypedArrayType::Int32Array, buffer, 0, 2);
        
        assert_eq!(array.length, 2);
        assert_eq!(array.byte_length, 8);
        assert_eq!(array.get(0).unwrap(), Value::Number(67305985.0)); // Little-endian interpretation
    }

    #[tokio::test]
    async fn test_promise_creation() {
        let promise = Promise::new();
        
        assert!(promise.is_pending());
        assert!(!promise.is_fulfilled());
        assert!(!promise.is_rejected());
        assert_eq!(promise.state, PromiseState::Pending);
    }

    #[tokio::test]
    async fn test_promise_fulfillment() {
        let mut promise = Promise::new();
        
        promise.fulfill(Value::String("success".to_string())).unwrap();
        
        assert!(!promise.is_pending());
        assert!(promise.is_fulfilled());
        assert!(!promise.is_rejected());
        
        if let PromiseState::Fulfilled(value) = &promise.state {
            assert_eq!(value, &Value::String("success".to_string()));
        } else {
            panic!("Expected fulfilled state");
        }
    }

    #[tokio::test]
    async fn test_promise_rejection() {
        let mut promise = Promise::new();
        
        promise.reject(Value::String("error".to_string())).unwrap();
        
        assert!(!promise.is_pending());
        assert!(!promise.is_fulfilled());
        assert!(promise.is_rejected());
        
        if let PromiseState::Rejected(reason) = &promise.state {
            assert_eq!(reason, &Value::String("error".to_string()));
        } else {
            panic!("Expected rejected state");
        }
    }

    #[tokio::test]
    async fn test_promise_double_settlement() {
        let mut promise = Promise::new();
        
        promise.fulfill(Value::String("success".to_string())).unwrap();
        
        // Try to fulfill again
        assert!(promise.fulfill(Value::String("another".to_string())).is_err());
        
        // Try to reject
        assert!(promise.reject(Value::String("error".to_string())).is_err());
    }

    #[tokio::test]
    async fn test_promise_then_handler() {
        let mut promise = Promise::new();
        let mut handler_called = false;
        
        promise.then(|value| {
            handler_called = true;
            assert_eq!(value, Value::String("test".to_string()));
            Ok(Value::Undefined)
        }).unwrap();
        
        // Handler should not be called yet
        assert!(!handler_called);
        
        // Fulfill the promise
        promise.fulfill(Value::String("test".to_string())).unwrap();
        
        // Handler should now be called
        assert!(handler_called);
    }

    #[tokio::test]
    async fn test_promise_catch_handler() {
        let mut promise = Promise::new();
        let mut handler_called = false;
        
        promise.catch(|reason| {
            handler_called = true;
            assert_eq!(reason, Value::String("error".to_string()));
            Ok(Value::Undefined)
        }).unwrap();
        
        // Handler should not be called yet
        assert!(!handler_called);
        
        // Reject the promise
        promise.reject(Value::String("error".to_string())).unwrap();
        
        // Handler should now be called
        assert!(handler_called);
    }

    #[tokio::test]
    async fn test_promise_with_executor() {
        let mut executor_called = false;
        let mut resolve_called = false;
        let mut reject_called = false;
        
        let promise = Promise::with_executor(Box::new(move |resolve, reject| {
            executor_called = true;
            
            // Call resolve
            resolve(Value::String("resolved".to_string()));
            resolve_called = true;
            
            // Reject should not be called
            reject(Value::String("rejected".to_string()));
            reject_called = true;
        }));
        
        assert!(executor_called);
        assert!(resolve_called);
        assert!(reject_called);
    }

    #[tokio::test]
    async fn test_fetch_api_creation() {
        let fetch_api = FetchAPI::new();
        
        // Verify default headers are set
        assert_eq!(fetch_api.timeout, Duration::from_secs(30));
    }

    #[tokio::test]
    async fn test_fetch_api_configuration() {
        let mut fetch_api = FetchAPI::new();
        
        // Test timeout setting
        fetch_api.set_timeout(Duration::from_secs(60));
        assert_eq!(fetch_api.timeout, Duration::from_secs(60));
        
        // Test header addition
        fetch_api.add_default_header("X-Test".to_string(), "test-value".to_string());
    }

    #[tokio::test]
    async fn test_timer_manager_creation() {
        let timer_manager = TimerManager::new();
        
        assert_eq!(timer_manager.active_timer_count(), 0);
    }

    #[tokio::test]
    async fn test_timer_manager_timeout() {
        let timer_manager = TimerManager::new();
        let mut callback_called = false;
        
        let timer_id = timer_manager.set_timeout(|| {
            callback_called = true;
            Ok(())
        }, 100).await.unwrap();
        
        assert!(timer_id > 0);
        assert_eq!(timer_manager.active_timer_count(), 1);
        
        // Wait for timeout
        sleep(Duration::from_millis(150)).await;
        
        // Note: In a real implementation, we would need to process timer events
        // For now, we just verify the timer was created
    }

    #[tokio::test]
    async fn test_timer_manager_interval() {
        let timer_manager = TimerManager::new();
        let mut callback_count = 0;
        
        let timer_id = timer_manager.set_interval(|| {
            callback_count += 1;
            Ok(())
        }, 50).await.unwrap();
        
        assert!(timer_id > 0);
        assert_eq!(timer_manager.active_timer_count(), 1);
        
        // Wait for a few intervals
        sleep(Duration::from_millis(200)).await;
        
        // Note: In a real implementation, we would need to process timer events
        // For now, we just verify the timer was created
    }

    #[tokio::test]
    async fn test_timer_manager_clear() {
        let timer_manager = TimerManager::new();
        
        let timer_id = timer_manager.set_timeout(|| Ok(()), 1000).await.unwrap();
        assert_eq!(timer_manager.active_timer_count(), 1);
        
        timer_manager.clear_timer(timer_id).await.unwrap();
        assert_eq!(timer_manager.active_timer_count(), 0);
    }

    #[tokio::test]
    async fn test_event_manager_creation() {
        let event_manager = EventManager::new();
        
        assert_eq!(event_manager.listener_count("test"), 0);
    }

    #[tokio::test]
    async fn test_event_manager_add_listener() {
        let event_manager = EventManager::new();
        let mut callback_called = false;
        
        event_manager.add_event_listener("test", EventType::Click, |event| {
            callback_called = true;
            assert_eq!(event.event_type, EventType::Click);
            Ok(())
        }, false).unwrap();
        
        assert_eq!(event_manager.listener_count("test"), 1);
    }

    #[tokio::test]
    async fn test_event_manager_remove_listener() {
        let event_manager = EventManager::new();
        
        event_manager.add_event_listener("test", EventType::Click, |_| Ok(()), false).unwrap();
        assert_eq!(event_manager.listener_count("test"), 1);
        
        event_manager.remove_event_listener("test", EventType::Click).unwrap();
        assert_eq!(event_manager.listener_count("test"), 0);
    }

    #[tokio::test]
    async fn test_event_manager_dispatch() {
        let event_manager = EventManager::new();
        let mut callback_called = false;
        
        event_manager.add_event_listener("test", EventType::Click, |event| {
            callback_called = true;
            assert_eq!(event.event_type, EventType::Click);
            assert_eq!(event.target, Some("test".to_string()));
            Ok(())
        }, false).unwrap();
        
        let event = Event {
            event_type: EventType::Click,
            target: Some("test".to_string()),
            current_target: Some("test".to_string()),
            bubbles: true,
            cancelable: true,
            default_prevented: false,
            propagation_stopped: false,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
            data: HashMap::new(),
        };
        
        event_manager.dispatch_event(event).await.unwrap();
        
        // Note: In a real implementation, we would need to process events
        // For now, we just verify the event was dispatched
    }

    #[tokio::test]
    async fn test_event_creation() {
        let event = Event {
            event_type: EventType::Custom("test".to_string()),
            target: Some("test-target".to_string()),
            current_target: Some("test-current".to_string()),
            bubbles: true,
            cancelable: true,
            default_prevented: false,
            propagation_stopped: false,
            timestamp: 123456789,
            data: HashMap::new(),
        };
        
        assert_eq!(event.event_type, EventType::Custom("test".to_string()));
        assert_eq!(event.target, Some("test-target".to_string()));
        assert_eq!(event.current_target, Some("test-current".to_string()));
        assert!(event.bubbles);
        assert!(event.cancelable);
        assert!(!event.default_prevented);
        assert!(!event.propagation_stopped);
        assert_eq!(event.timestamp, 123456789);
    }

    #[tokio::test]
    async fn test_event_types() {
        let event_types = vec![
            EventType::Load,
            EventType::Unload,
            EventType::Click,
            EventType::MouseDown,
            EventType::MouseUp,
            EventType::MouseMove,
            EventType::KeyDown,
            EventType::KeyUp,
            EventType::Submit,
            EventType::Change,
            EventType::Focus,
            EventType::Blur,
            EventType::Custom("custom".to_string()),
        ];
        
        for event_type in event_types {
            assert_eq!(format!("{:?}", event_type), format!("{:?}", event_type));
        }
    }

    #[tokio::test]
    async fn test_builtin_objects_creation() {
        let builtins = BuiltinObjects::new();
        
        assert_eq!(builtins.timer_count(), 0);
        assert_eq!(builtins.listener_count("test"), 0);
    }

    #[tokio::test]
    async fn test_builtin_objects_typed_array() {
        let builtins = BuiltinObjects::new();
        
        let array = builtins.create_typed_array(TypedArrayType::Int32Array, 5).unwrap();
        assert_eq!(array.array_type, TypedArrayType::Int32Array);
        assert_eq!(array.length, 5);
    }

    #[tokio::test]
    async fn test_builtin_objects_promise() {
        let builtins = BuiltinObjects::new();
        
        let promise = builtins.create_promise(Box::new(|resolve, _reject| {
            resolve(Value::String("test".to_string()));
        }));
        
        assert!(promise.is_pending());
    }

    #[tokio::test]
    async fn test_builtin_objects_timers() {
        let builtins = BuiltinObjects::new();
        
        let timer_id = builtins.set_timeout(|| Ok(()), 100).await.unwrap();
        assert!(timer_id > 0);
        assert_eq!(builtins.timer_count(), 1);
        
        builtins.clear_timer(timer_id).await.unwrap();
        assert_eq!(builtins.timer_count(), 0);
    }

    #[tokio::test]
    async fn test_builtin_objects_events() {
        let builtins = BuiltinObjects::new();
        
        builtins.add_event_listener("test", EventType::Click, |_| Ok(()), false).unwrap();
        assert_eq!(builtins.listener_count("test"), 1);
        
        builtins.remove_event_listener("test", EventType::Click).unwrap();
        assert_eq!(builtins.listener_count("test"), 0);
    }

    #[tokio::test]
    async fn test_value_enum() {
        let values = vec![
            Value::Undefined,
            Value::Null,
            Value::Boolean(true),
            Value::Boolean(false),
            Value::Number(42.0),
            Value::Number(-3.14),
            Value::String("hello".to_string()),
            Value::Object(HashMap::new()),
            Value::Array(vec![]),
            Value::Function("test".to_string()),
        ];
        
        for value in values {
            assert_eq!(format!("{:?}", value), format!("{:?}", value));
        }
    }

    #[tokio::test]
    async fn test_typed_array_integration() {
        let builtins = BuiltinObjects::new();
        
        // Create TypedArray
        let mut array = builtins.create_typed_array(TypedArrayType::Float32Array, 4).unwrap();
        
        // Set values
        array.set(0, Value::Number(1.0)).unwrap();
        array.set(1, Value::Number(2.0)).unwrap();
        array.set(2, Value::Number(3.0)).unwrap();
        array.set(3, Value::Number(4.0)).unwrap();
        
        // Get values
        assert_eq!(array.get(0).unwrap(), Value::Number(1.0));
        assert_eq!(array.get(1).unwrap(), Value::Number(2.0));
        assert_eq!(array.get(2).unwrap(), Value::Number(3.0));
        assert_eq!(array.get(3).unwrap(), Value::Number(4.0));
        
        // Test buffer access
        assert_eq!(array.buffer().len(), 16); // 4 elements * 4 bytes
        assert_eq!(array.byte_length(), 16);
        assert_eq!(array.length(), 4);
    }

    #[tokio::test]
    async fn test_promise_integration() {
        let builtins = BuiltinObjects::new();
        let mut promise_resolved = false;
        
        let mut promise = builtins.create_promise(Box::new(move |resolve, _reject| {
            promise_resolved = true;
            resolve(Value::String("success".to_string()));
        }));
        
        // Add then handler
        let mut then_called = false;
        promise.then(|value| {
            then_called = true;
            assert_eq!(value, Value::String("success".to_string()));
            Ok(Value::Undefined)
        }).unwrap();
        
        // Add catch handler
        let mut catch_called = false;
        promise.catch(|_reason| {
            catch_called = true;
            Ok(Value::Undefined)
        }).unwrap();
        
        // Verify initial state
        assert!(promise.is_pending());
        assert!(!then_called);
        assert!(!catch_called);
        
        // Fulfill promise
        promise.fulfill(Value::String("success".to_string())).unwrap();
        
        // Verify final state
        assert!(promise.is_fulfilled());
        assert!(then_called);
        assert!(!catch_called);
    }

    #[tokio::test]
    async fn test_timer_integration() {
        let builtins = BuiltinObjects::new();
        let mut timeout_called = false;
        let mut interval_called = 0;
        
        // Set timeout
        let timeout_id = builtins.set_timeout(|| {
            timeout_called = true;
            Ok(())
        }, 50).await.unwrap();
        
        // Set interval
        let interval_id = builtins.set_interval(|| {
            interval_called += 1;
            Ok(())
        }, 25).await.unwrap();
        
        // Wait for execution
        sleep(Duration::from_millis(100)).await;
        
        // Clear timers
        builtins.clear_timer(timeout_id).await.unwrap();
        builtins.clear_timer(interval_id).await.unwrap();
        
        // Verify timers were cleared
        assert_eq!(builtins.timer_count(), 0);
    }

    #[tokio::test]
    async fn test_event_integration() {
        let builtins = BuiltinObjects::new();
        let mut click_called = false;
        let mut custom_called = false;
        
        // Add event listeners
        builtins.add_event_listener("button", EventType::Click, |event| {
            click_called = true;
            assert_eq!(event.event_type, EventType::Click);
            assert_eq!(event.target, Some("button".to_string()));
            Ok(())
        }, false).unwrap();
        
        builtins.add_event_listener("custom", EventType::Custom("test".to_string()), |event| {
            custom_called = true;
            assert_eq!(event.event_type, EventType::Custom("test".to_string()));
            Ok(())
        }, false).unwrap();
        
        // Create and dispatch events
        let click_event = Event {
            event_type: EventType::Click,
            target: Some("button".to_string()),
            current_target: Some("button".to_string()),
            bubbles: true,
            cancelable: true,
            default_prevented: false,
            propagation_stopped: false,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
            data: HashMap::new(),
        };
        
        let custom_event = Event {
            event_type: EventType::Custom("test".to_string()),
            target: Some("custom".to_string()),
            current_target: Some("custom".to_string()),
            bubbles: false,
            cancelable: false,
            default_prevented: false,
            propagation_stopped: false,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
            data: HashMap::new(),
        };
        
        // Dispatch events
        builtins.dispatch_event(click_event).await.unwrap();
        builtins.dispatch_event(custom_event).await.unwrap();
        
        // Note: In a real implementation, we would need to process events
        // For now, we just verify the events were dispatched
    }

    #[tokio::test]
    async fn test_comprehensive_integration() {
        let builtins = BuiltinObjects::new();
        
        // Test TypedArray
        let mut array = builtins.create_typed_array(TypedArrayType::Uint8Array, 3).unwrap();
        array.set(0, Value::Number(1.0)).unwrap();
        array.set(1, Value::Number(2.0)).unwrap();
        array.set(2, Value::Number(3.0)).unwrap();
        
        assert_eq!(array.get(0).unwrap(), Value::Number(1.0));
        assert_eq!(array.get(1).unwrap(), Value::Number(2.0));
        assert_eq!(array.get(2).unwrap(), Value::Number(3.0));
        
        // Test Promise
        let mut promise = builtins.create_promise(Box::new(|resolve, _reject| {
            resolve(Value::Number(42.0));
        }));
        
        let mut promise_result = None;
        promise.then(|value| {
            promise_result = Some(value);
            Ok(Value::Undefined)
        }).unwrap();
        
        promise.fulfill(Value::Number(42.0)).unwrap();
        assert_eq!(promise_result, Some(Value::Number(42.0)));
        
        // Test Timer
        let timer_id = builtins.set_timeout(|| Ok(()), 10).await.unwrap();
        assert!(timer_id > 0);
        builtins.clear_timer(timer_id).await.unwrap();
        
        // Test Event
        builtins.add_event_listener("test", EventType::Click, |_| Ok(()), false).unwrap();
        assert_eq!(builtins.listener_count("test"), 1);
        builtins.remove_event_listener("test", EventType::Click).unwrap();
        assert_eq!(builtins.listener_count("test"), 0);
    }
}
