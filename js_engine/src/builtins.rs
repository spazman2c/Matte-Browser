use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout};
use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};

/// TypedArray types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypedArrayType {
    Int8Array,
    Uint8Array,
    Uint8ClampedArray,
    Int16Array,
    Uint16Array,
    Int32Array,
    Uint32Array,
    Float32Array,
    Float64Array,
    BigInt64Array,
    BigUint64Array,
}

/// TypedArray implementation
#[derive(Debug, Clone)]
pub struct TypedArray {
    /// Array type
    pub array_type: TypedArrayType,
    /// Underlying buffer
    pub buffer: Vec<u8>,
    /// Byte offset into buffer
    pub byte_offset: usize,
    /// Length in elements
    pub length: usize,
    /// Byte length
    pub byte_length: usize,
}

/// TypedArray constructor
pub struct TypedArrayConstructor {
    /// Array type
    array_type: TypedArrayType,
    /// Element size in bytes
    element_size: usize,
    /// Constructor function
    constructor_fn: fn(&[u8], usize, usize) -> TypedArray,
}

/// Promise states
#[derive(Debug, Clone, PartialEq)]
pub enum PromiseState {
    Pending,
    Fulfilled(Value),
    Rejected(Value),
}

/// Promise implementation
#[derive(Debug, Clone)]
pub struct Promise {
    /// Promise state
    pub state: PromiseState,
    /// Fulfillment handlers
    pub on_fulfilled: Vec<Box<dyn Fn(Value) -> Result<Value> + Send + Sync>>,
    /// Rejection handlers
    pub on_rejected: Vec<Box<dyn Fn(Value) -> Result<Value> + Send + Sync>>,
    /// Promise executor
    pub executor: Option<Box<dyn Fn(Box<dyn Fn(Value) + Send + Sync>, Box<dyn Fn(Value) + Send + Sync>) + Send + Sync>>,
}

/// Promise constructor
pub struct PromiseConstructor {
    /// Constructor function
    constructor_fn: fn(Box<dyn Fn(Box<dyn Fn(Value) + Send + Sync>, Box<dyn Fn(Value) + Send + Sync>) + Send + Sync>) -> Promise,
}

/// Fetch request configuration
#[derive(Debug, Clone)]
pub struct FetchRequest {
    /// Request URL
    pub url: String,
    /// HTTP method
    pub method: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: Option<Vec<u8>>,
    /// Request mode
    pub mode: String,
    /// Request credentials
    pub credentials: String,
    /// Request cache
    pub cache: String,
    /// Request redirect
    pub redirect: String,
    /// Request referrer
    pub referrer: String,
    /// Request integrity
    pub integrity: Option<String>,
}

/// Fetch response
#[derive(Debug, Clone)]
pub struct FetchResponse {
    /// Response URL
    pub url: String,
    /// Response status
    pub status: u16,
    /// Response status text
    pub status_text: String,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
    /// Response type
    pub response_type: String,
    /// Response redirected
    pub redirected: bool,
}

/// Fetch API implementation
pub struct FetchAPI {
    /// HTTP client
    client: reqwest::Client,
    /// Request timeout
    timeout: Duration,
    /// Default headers
    default_headers: HashMap<String, String>,
}

/// Timer types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimerType {
    Timeout,
    Interval,
}

/// Timer implementation
#[derive(Debug, Clone)]
pub struct Timer {
    /// Timer ID
    pub id: u64,
    /// Timer type
    pub timer_type: TimerType,
    /// Callback function
    pub callback: Box<dyn Fn() -> Result<()> + Send + Sync>,
    /// Delay in milliseconds
    pub delay: u64,
    /// Whether timer is active
    pub active: bool,
    /// Creation time
    pub created_at: Instant,
    /// Next execution time
    pub next_execution: Instant,
}

/// Timer manager
pub struct TimerManager {
    /// Active timers
    timers: Arc<RwLock<HashMap<u64, Timer>>>,
    /// Next timer ID
    next_timer_id: Arc<RwLock<u64>>,
    /// Timer channel
    timer_tx: mpsc::Sender<TimerEvent>,
    /// Timer receiver
    timer_rx: mpsc::Receiver<TimerEvent>,
}

/// Timer events
#[derive(Debug)]
pub enum TimerEvent {
    CreateTimer(Timer),
    CancelTimer(u64),
    ExecuteTimer(u64),
}

/// Event types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    Load,
    Unload,
    Click,
    MouseDown,
    MouseUp,
    MouseMove,
    KeyDown,
    KeyUp,
    Submit,
    Change,
    Focus,
    Blur,
    Custom(String),
}

/// Event implementation
#[derive(Debug, Clone)]
pub struct Event {
    /// Event type
    pub event_type: EventType,
    /// Event target
    pub target: Option<String>,
    /// Event current target
    pub current_target: Option<String>,
    /// Event bubbles
    pub bubbles: bool,
    /// Event cancelable
    pub cancelable: bool,
    /// Event default prevented
    pub default_prevented: bool,
    /// Event propagation stopped
    pub propagation_stopped: bool,
    /// Event timestamp
    pub timestamp: u64,
    /// Event data
    pub data: HashMap<String, Value>,
}

/// Event listener
#[derive(Debug, Clone)]
pub struct EventListener {
    /// Event type
    pub event_type: EventType,
    /// Callback function
    pub callback: Box<dyn Fn(&Event) -> Result<()> + Send + Sync>,
    /// Whether to capture
    pub capture: bool,
    /// Whether to use once
    pub once: bool,
    /// Whether passive
    pub passive: bool,
}

/// Event manager
pub struct EventManager {
    /// Event listeners by target
    listeners: Arc<RwLock<HashMap<String, Vec<EventListener>>>>,
    /// Event queue
    event_queue: Arc<RwLock<VecDeque<Event>>>,
    /// Event processing channel
    event_tx: mpsc::Sender<Event>,
    /// Event receiver
    event_rx: mpsc::Receiver<Event>,
}

/// Built-in objects manager
pub struct BuiltinObjects {
    /// TypedArray constructors
    typed_array_constructors: HashMap<TypedArrayType, TypedArrayConstructor>,
    /// Promise constructor
    promise_constructor: PromiseConstructor,
    /// Fetch API
    fetch_api: FetchAPI,
    /// Timer manager
    timer_manager: TimerManager,
    /// Event manager
    event_manager: EventManager,
}

// Placeholder Value type for compilation
#[derive(Debug, Clone)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Function(String),
    TypedArray(TypedArray),
    Promise(Promise),
    Event(Event),
}

impl TypedArray {
    /// Create a new TypedArray
    pub fn new(array_type: TypedArrayType, length: usize) -> Self {
        let element_size = Self::get_element_size(array_type);
        let byte_length = length * element_size;
        
        Self {
            array_type,
            buffer: vec![0; byte_length],
            byte_offset: 0,
            length,
            byte_length,
        }
    }

    /// Create from buffer
    pub fn from_buffer(array_type: TypedArrayType, buffer: Vec<u8>, byte_offset: usize, length: usize) -> Self {
        let element_size = Self::get_element_size(array_type);
        let byte_length = length * element_size;
        
        Self {
            array_type,
            buffer,
            byte_offset,
            length,
            byte_length,
        }
    }

    /// Get element size for array type
    fn get_element_size(array_type: TypedArrayType) -> usize {
        match array_type {
            TypedArrayType::Int8Array | TypedArrayType::Uint8Array | TypedArrayType::Uint8ClampedArray => 1,
            TypedArrayType::Int16Array | TypedArrayType::Uint16Array => 2,
            TypedArrayType::Int32Array | TypedArrayType::Uint32Array | TypedArrayType::Float32Array => 4,
            TypedArrayType::Float64Array => 8,
            TypedArrayType::BigInt64Array | TypedArrayType::BigUint64Array => 8,
        }
    }

    /// Get element at index
    pub fn get(&self, index: usize) -> Result<Value> {
        if index >= self.length {
            return Err(Error::parsing("Index out of bounds".to_string()));
        }

        let element_size = Self::get_element_size(self.array_type);
        let start = self.byte_offset + index * element_size;
        let end = start + element_size;

        if end > self.buffer.len() {
            return Err(Error::parsing("Buffer access out of bounds".to_string()));
        }

        let bytes = &self.buffer[start..end];
        
        match self.array_type {
            TypedArrayType::Int8Array => {
                let value = bytes[0] as i8;
                Ok(Value::Number(value as f64))
            }
            TypedArrayType::Uint8Array => {
                let value = bytes[0];
                Ok(Value::Number(value as f64))
            }
            TypedArrayType::Uint8ClampedArray => {
                let value = bytes[0];
                Ok(Value::Number(value as f64))
            }
            TypedArrayType::Int16Array => {
                let value = i16::from_le_bytes([bytes[0], bytes[1]]);
                Ok(Value::Number(value as f64))
            }
            TypedArrayType::Uint16Array => {
                let value = u16::from_le_bytes([bytes[0], bytes[1]]);
                Ok(Value::Number(value as f64))
            }
            TypedArrayType::Int32Array => {
                let value = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Ok(Value::Number(value as f64))
            }
            TypedArrayType::Uint32Array => {
                let value = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Ok(Value::Number(value as f64))
            }
            TypedArrayType::Float32Array => {
                let value = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                Ok(Value::Number(value as f64))
            }
            TypedArrayType::Float64Array => {
                let value = f64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3],
                    bytes[4], bytes[5], bytes[6], bytes[7]
                ]);
                Ok(Value::Number(value))
            }
            _ => Err(Error::parsing("Unsupported array type".to_string())),
        }
    }

    /// Set element at index
    pub fn set(&mut self, index: usize, value: Value) -> Result<()> {
        if index >= self.length {
            return Err(Error::parsing("Index out of bounds".to_string()));
        }

        let element_size = Self::get_element_size(self.array_type);
        let start = self.byte_offset + index * element_size;
        let end = start + element_size;

        if end > self.buffer.len() {
            return Err(Error::parsing("Buffer access out of bounds".to_string()));
        }

        let number_value = match value {
            Value::Number(n) => n,
            Value::Boolean(b) => if b { 1.0 } else { 0.0 },
            _ => return Err(Error::parsing("Invalid value type".to_string())),
        };

        let bytes = &mut self.buffer[start..end];
        
        match self.array_type {
            TypedArrayType::Int8Array => {
                let value = number_value as i8;
                bytes[0] = value as u8;
            }
            TypedArrayType::Uint8Array => {
                let value = number_value as u8;
                bytes[0] = value;
            }
            TypedArrayType::Uint8ClampedArray => {
                let value = number_value.max(0.0).min(255.0) as u8;
                bytes[0] = value;
            }
            TypedArrayType::Int16Array => {
                let value = number_value as i16;
                bytes.copy_from_slice(&value.to_le_bytes());
            }
            TypedArrayType::Uint16Array => {
                let value = number_value as u16;
                bytes.copy_from_slice(&value.to_le_bytes());
            }
            TypedArrayType::Int32Array => {
                let value = number_value as i32;
                bytes.copy_from_slice(&value.to_le_bytes());
            }
            TypedArrayType::Uint32Array => {
                let value = number_value as u32;
                bytes.copy_from_slice(&value.to_le_bytes());
            }
            TypedArrayType::Float32Array => {
                let value = number_value as f32;
                bytes.copy_from_slice(&value.to_le_bytes());
            }
            TypedArrayType::Float64Array => {
                let value = number_value;
                bytes.copy_from_slice(&value.to_le_bytes());
            }
            _ => return Err(Error::parsing("Unsupported array type".to_string())),
        }

        Ok(())
    }

    /// Get array buffer
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Get byte length
    pub fn byte_length(&self) -> usize {
        self.byte_length
    }

    /// Get length
    pub fn length(&self) -> usize {
        self.length
    }
}

impl Promise {
    /// Create a new pending promise
    pub fn new() -> Self {
        Self {
            state: PromiseState::Pending,
            on_fulfilled: Vec::new(),
            on_rejected: Vec::new(),
            executor: None,
        }
    }

    /// Create a new promise with executor
    pub fn with_executor(executor: Box<dyn Fn(Box<dyn Fn(Value) + Send + Sync>, Box<dyn Fn(Value) + Send + Sync>) + Send + Sync>) -> Self {
        Self {
            state: PromiseState::Pending,
            on_fulfilled: Vec::new(),
            on_rejected: Vec::new(),
            executor: Some(executor),
        }
    }

    /// Fulfill the promise
    pub fn fulfill(&mut self, value: Value) -> Result<()> {
        if let PromiseState::Pending = self.state {
            self.state = PromiseState::Fulfilled(value.clone());
            
            // Execute fulfillment handlers
            for handler in &self.on_fulfilled {
                handler(value.clone())?;
            }
            
            Ok(())
        } else {
            Err(Error::parsing("Promise already settled".to_string()))
        }
    }

    /// Reject the promise
    pub fn reject(&mut self, reason: Value) -> Result<()> {
        if let PromiseState::Pending = self.state {
            self.state = PromiseState::Rejected(reason.clone());
            
            // Execute rejection handlers
            for handler in &self.on_rejected {
                handler(reason.clone())?;
            }
            
            Ok(())
        } else {
            Err(Error::parsing("Promise already settled".to_string()))
        }
    }

    /// Add fulfillment handler
    pub fn then<F>(&mut self, handler: F) -> Result<()>
    where
        F: Fn(Value) -> Result<Value> + Send + Sync + 'static,
    {
        match &self.state {
            PromiseState::Pending => {
                self.on_fulfilled.push(Box::new(handler));
                Ok(())
            }
            PromiseState::Fulfilled(value) => {
                handler(value.clone())?;
                Ok(())
            }
            PromiseState::Rejected(_) => Ok(()),
        }
    }

    /// Add rejection handler
    pub fn catch<F>(&mut self, handler: F) -> Result<()>
    where
        F: Fn(Value) -> Result<Value> + Send + Sync + 'static,
    {
        match &self.state {
            PromiseState::Pending => {
                self.on_rejected.push(Box::new(handler));
                Ok(())
            }
            PromiseState::Fulfilled(_) => Ok(()),
            PromiseState::Rejected(reason) => {
                handler(reason.clone())?;
                Ok(())
            }
        }
    }

    /// Check if promise is pending
    pub fn is_pending(&self) -> bool {
        matches!(self.state, PromiseState::Pending)
    }

    /// Check if promise is fulfilled
    pub fn is_fulfilled(&self) -> bool {
        matches!(self.state, PromiseState::Fulfilled(_))
    }

    /// Check if promise is rejected
    pub fn is_rejected(&self) -> bool {
        matches!(self.state, PromiseState::Rejected(_))
    }
}

impl FetchAPI {
    /// Create a new Fetch API instance
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        
        let mut default_headers = HashMap::new();
        default_headers.insert("User-Agent".to_string(), "Matte-Browser/1.0".to_string());
        default_headers.insert("Accept".to_string(), "*/*".to_string());

        Self {
            client,
            timeout: Duration::from_secs(30),
            default_headers,
        }
    }

    /// Fetch a resource
    pub async fn fetch(&self, request: FetchRequest) -> Result<FetchResponse> {
        let mut req = self.client
            .request(
                reqwest::Method::from_bytes(request.method.as_bytes()).unwrap_or(reqwest::Method::GET),
                &request.url
            )
            .timeout(self.timeout);

        // Add headers
        for (key, value) in &request.headers {
            req = req.header(key, value);
        }

        // Add default headers
        for (key, value) in &self.default_headers {
            if !request.headers.contains_key(key) {
                req = req.header(key, value);
            }
        }

        // Add body if present
        if let Some(body) = request.body {
            req = req.body(body);
        }

        // Execute request
        let response = req.send().await
            .map_err(|e| Error::parsing(format!("Fetch request failed: {}", e)))?;

        // Get response status
        let status = response.status().as_u16();
        let status_text = response.status().canonical_reason().unwrap_or("Unknown").to_string();

        // Get response headers
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            headers.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
        }

        // Get response body
        let body = response.bytes().await
            .map_err(|e| Error::parsing(format!("Failed to read response body: {}", e)))?
            .to_vec();

        Ok(FetchResponse {
            url: request.url,
            status,
            status_text,
            headers,
            body,
            response_type: "basic".to_string(),
            redirected: false,
        })
    }

    /// Set request timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Add default header
    pub fn add_default_header(&mut self, key: String, value: String) {
        self.default_headers.insert(key, value);
    }
}

impl TimerManager {
    /// Create a new timer manager
    pub fn new() -> Self {
        let (timer_tx, timer_rx) = mpsc::channel(100);
        
        Self {
            timers: Arc::new(RwLock::new(HashMap::new())),
            next_timer_id: Arc::new(RwLock::new(1)),
            timer_tx,
            timer_rx,
        }
    }

    /// Set a timeout
    pub async fn set_timeout<F>(&self, callback: F, delay: u64) -> Result<u64>
    where
        F: Fn() -> Result<()> + Send + Sync + 'static,
    {
        let mut next_id = self.next_timer_id.write();
        let timer_id = *next_id;
        *next_id += 1;

        let timer = Timer {
            id: timer_id,
            timer_type: TimerType::Timeout,
            callback: Box::new(callback),
            delay,
            active: true,
            created_at: Instant::now(),
            next_execution: Instant::now() + Duration::from_millis(delay),
        };

        let mut timers = self.timers.write();
        timers.insert(timer_id, timer);

        // Send timer event
        self.timer_tx.send(TimerEvent::CreateTimer(timer.clone())).await
            .map_err(|e| Error::parsing(format!("Failed to send timer event: {}", e)))?;

        Ok(timer_id)
    }

    /// Set an interval
    pub async fn set_interval<F>(&self, callback: F, delay: u64) -> Result<u64>
    where
        F: Fn() -> Result<()> + Send + Sync + 'static,
    {
        let mut next_id = self.next_timer_id.write();
        let timer_id = *next_id;
        *next_id += 1;

        let timer = Timer {
            id: timer_id,
            timer_type: TimerType::Interval,
            callback: Box::new(callback),
            delay,
            active: true,
            created_at: Instant::now(),
            next_execution: Instant::now() + Duration::from_millis(delay),
        };

        let mut timers = self.timers.write();
        timers.insert(timer_id, timer);

        // Send timer event
        self.timer_tx.send(TimerEvent::CreateTimer(timer.clone())).await
            .map_err(|e| Error::parsing(format!("Failed to send timer event: {}", e)))?;

        Ok(timer_id)
    }

    /// Clear a timeout or interval
    pub async fn clear_timer(&self, timer_id: u64) -> Result<()> {
        let mut timers = self.timers.write();
        
        if timers.remove(&timer_id).is_some() {
            // Send cancel event
            self.timer_tx.send(TimerEvent::CancelTimer(timer_id)).await
                .map_err(|e| Error::parsing(format!("Failed to send cancel event: {}", e)))?;
        }

        Ok(())
    }

    /// Get active timer count
    pub fn active_timer_count(&self) -> usize {
        self.timers.read().len()
    }

    /// Process timer events
    pub async fn process_events(&mut self) -> Result<()> {
        while let Some(event) = self.timer_rx.recv().await {
            match event {
                TimerEvent::CreateTimer(timer) => {
                    // Timer creation is handled in set_timeout/set_interval
                }
                TimerEvent::CancelTimer(timer_id) => {
                    let mut timers = self.timers.write();
                    timers.remove(&timer_id);
                }
                TimerEvent::ExecuteTimer(timer_id) => {
                    let mut timers = self.timers.write();
                    if let Some(timer) = timers.get_mut(&timer_id) {
                        if timer.active {
                            // Execute callback
                            if let Err(e) = (timer.callback)() {
                                eprintln!("Timer callback error: {}", e);
                            }

                            match timer.timer_type {
                                TimerType::Timeout => {
                                    // Remove timeout after execution
                                    timers.remove(&timer_id);
                                }
                                TimerType::Interval => {
                                    // Schedule next execution
                                    timer.next_execution = Instant::now() + Duration::from_millis(timer.delay);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl EventManager {
    /// Create a new event manager
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::channel(100);
        
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
            event_queue: Arc::new(RwLock::new(VecDeque::new())),
            event_tx,
            event_rx,
        }
    }

    /// Add event listener
    pub fn add_event_listener<F>(&self, target: &str, event_type: EventType, callback: F, capture: bool) -> Result<()>
    where
        F: Fn(&Event) -> Result<()> + Send + Sync + 'static,
    {
        let listener = EventListener {
            event_type,
            callback: Box::new(callback),
            capture,
            once: false,
            passive: false,
        };

        let mut listeners = self.listeners.write();
        listeners.entry(target.to_string())
            .or_insert_with(Vec::new)
            .push(listener);

        Ok(())
    }

    /// Remove event listener
    pub fn remove_event_listener(&self, target: &str, event_type: EventType) -> Result<()> {
        let mut listeners = self.listeners.write();
        
        if let Some(target_listeners) = listeners.get_mut(target) {
            target_listeners.retain(|listener| listener.event_type != event_type);
        }

        Ok(())
    }

    /// Dispatch event
    pub async fn dispatch_event(&self, event: Event) -> Result<()> {
        // Add to event queue
        {
            let mut queue = self.event_queue.write();
            queue.push_back(event.clone());
        }

        // Send event for processing
        self.event_tx.send(event).await
            .map_err(|e| Error::parsing(format!("Failed to send event: {}", e)))?;

        Ok(())
    }

    /// Process events
    pub async fn process_events(&mut self) -> Result<()> {
        while let Some(event) = self.event_rx.recv().await {
            // Find listeners for this event
            let listeners = {
                let listeners = self.listeners.read();
                listeners.get(&event.event_type.to_string())
                    .cloned()
                    .unwrap_or_default()
            };

            // Execute listeners
            for listener in listeners {
                if let Err(e) = (listener.callback)(&event) {
                    eprintln!("Event listener error: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Get listener count for target
    pub fn listener_count(&self, target: &str) -> usize {
        self.listeners.read()
            .get(target)
            .map(|listeners| listeners.len())
            .unwrap_or(0)
    }
}

impl BuiltinObjects {
    /// Create a new built-in objects manager
    pub fn new() -> Self {
        let mut typed_array_constructors = HashMap::new();
        
        // Initialize TypedArray constructors
        for array_type in &[
            TypedArrayType::Int8Array,
            TypedArrayType::Uint8Array,
            TypedArrayType::Uint8ClampedArray,
            TypedArrayType::Int16Array,
            TypedArrayType::Uint16Array,
            TypedArrayType::Int32Array,
            TypedArrayType::Uint32Array,
            TypedArrayType::Float32Array,
            TypedArrayType::Float64Array,
        ] {
            let constructor = TypedArrayConstructor {
                array_type: *array_type,
                element_size: TypedArray::get_element_size(*array_type),
                constructor_fn: |buffer, offset, length| {
                    TypedArray::from_buffer(*array_type, buffer.to_vec(), offset, length)
                },
            };
            typed_array_constructors.insert(*array_type, constructor);
        }

        let promise_constructor = PromiseConstructor {
            constructor_fn: |executor| Promise::with_executor(executor),
        };

        let fetch_api = FetchAPI::new();
        let timer_manager = TimerManager::new();
        let event_manager = EventManager::new();

        Self {
            typed_array_constructors,
            promise_constructor,
            fetch_api,
            timer_manager,
            event_manager,
        }
    }

    /// Create TypedArray
    pub fn create_typed_array(&self, array_type: TypedArrayType, length: usize) -> Result<TypedArray> {
        Ok(TypedArray::new(array_type, length))
    }

    /// Create Promise
    pub fn create_promise(&self, executor: Box<dyn Fn(Box<dyn Fn(Value) + Send + Sync>, Box<dyn Fn(Value) + Send + Sync>) + Send + Sync>) -> Promise {
        (self.promise_constructor.constructor_fn)(executor)
    }

    /// Fetch resource
    pub async fn fetch(&self, request: FetchRequest) -> Result<FetchResponse> {
        self.fetch_api.fetch(request).await
    }

    /// Set timeout
    pub async fn set_timeout<F>(&self, callback: F, delay: u64) -> Result<u64>
    where
        F: Fn() -> Result<()> + Send + Sync + 'static,
    {
        self.timer_manager.set_timeout(callback, delay).await
    }

    /// Set interval
    pub async fn set_interval<F>(&self, callback: F, delay: u64) -> Result<u64>
    where
        F: Fn() -> Result<()> + Send + Sync + 'static,
    {
        self.timer_manager.set_interval(callback, delay).await
    }

    /// Clear timer
    pub async fn clear_timer(&self, timer_id: u64) -> Result<()> {
        self.timer_manager.clear_timer(timer_id).await
    }

    /// Add event listener
    pub fn add_event_listener<F>(&self, target: &str, event_type: EventType, callback: F, capture: bool) -> Result<()>
    where
        F: Fn(&Event) -> Result<()> + Send + Sync + 'static,
    {
        self.event_manager.add_event_listener(target, event_type, callback, capture)
    }

    /// Remove event listener
    pub fn remove_event_listener(&self, target: &str, event_type: EventType) -> Result<()> {
        self.event_manager.remove_event_listener(target, event_type)
    }

    /// Dispatch event
    pub async fn dispatch_event(&self, event: Event) -> Result<()> {
        self.event_manager.dispatch_event(event).await
    }

    /// Get timer count
    pub fn timer_count(&self) -> usize {
        self.timer_manager.active_timer_count()
    }

    /// Get listener count
    pub fn listener_count(&self, target: &str) -> usize {
        self.event_manager.listener_count(target)
    }
}

use std::collections::VecDeque;
