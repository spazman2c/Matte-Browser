//! Inter-process communication (IPC) for the Matte browser.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{TabId, RendererId, Url, Permission, PermissionState};

/// IPC message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    // Browser process messages
    CreateTab(CreateTabRequest),
    CloseTab(CloseTabRequest),
    NavigateTab(NavigateTabRequest),
    UpdateTab(UpdateTabRequest),
    
    // Window management
    CreateWindow(CreateWindowRequest),
    CloseWindow(CloseWindowRequest),
    UpdateWindow(UpdateWindowRequest),
    
    // Renderer process messages
    RendererReady(RendererReadyRequest),
    RendererCrashed(RendererCrashedRequest),
    RendererResponse(RendererResponseRequest),
    
    // Network process messages
    NetworkRequest(NetworkRequestMessage),
    NetworkResponse(NetworkResponseMessage),
    NetworkError(NetworkErrorMessage),
    
    // GPU process messages
    GpuCommand(GpuCommandMessage),
    GpuResponse(GpuResponseMessage),
    
    // Permission requests
    PermissionRequest(PermissionRequestMessage),
    PermissionResponse(PermissionResponseMessage),
    
    // System messages
    Ping(PingMessage),
    Pong(PongMessage),
    Shutdown(ShutdownMessage),
    Error(ErrorMessage),
}

/// Message priority for IPC
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for MessagePriority {
    fn default() -> Self {
        MessagePriority::Normal
    }
}

/// IPC message wrapper with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcEnvelope {
    pub message_id: u64,
    pub source_process: String,
    pub target_process: String,
    pub priority: MessagePriority,
    pub timestamp: std::time::SystemTime,
    pub message: IpcMessage,
    pub sequence_id: u64,
}

impl IpcEnvelope {
    pub fn new(
        message_id: u64,
        source_process: String,
        target_process: String,
        message: IpcMessage,
    ) -> Self {
        Self {
            message_id,
            source_process,
            target_process,
            priority: MessagePriority::Normal,
            timestamp: std::time::SystemTime::now(),
            message,
            sequence_id: Self::generate_sequence_id(),
        }
    }

    pub fn with_priority(mut self, priority: MessagePriority) -> Self {
        self.priority = priority;
        self
    }

    fn generate_sequence_id() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static SEQUENCE_COUNTER: AtomicU64 = AtomicU64::new(0);
        SEQUENCE_COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

// Request/Response message types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTabRequest {
    pub window_id: u64,
    pub url: Option<Url>,
    pub incognito: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTabResponse {
    pub tab_id: TabId,
    pub renderer_id: Option<RendererId>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseTabRequest {
    pub tab_id: TabId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseTabResponse {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigateTabRequest {
    pub tab_id: TabId,
    pub url: Url,
    pub reload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigateTabResponse {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTabRequest {
    pub tab_id: TabId,
    pub title: Option<String>,
    pub url: Option<Url>,
    pub loading: Option<bool>,
    pub can_go_back: Option<bool>,
    pub can_go_forward: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWindowRequest {
    pub width: u32,
    pub height: u32,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub maximized: bool,
    pub fullscreen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWindowResponse {
    pub window_id: u64,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseWindowRequest {
    pub window_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWindowRequest {
    pub window_id: u64,
    pub title: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub maximized: Option<bool>,
    pub fullscreen: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RendererReadyRequest {
    pub renderer_id: RendererId,
    pub tab_id: TabId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RendererCrashedRequest {
    pub renderer_id: RendererId,
    pub tab_id: TabId,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RendererResponseRequest {
    pub renderer_id: RendererId,
    pub tab_id: TabId,
    pub response_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRequestMessage {
    pub request_id: u64,
    pub tab_id: TabId,
    pub url: Url,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResponseMessage {
    pub request_id: u64,
    pub tab_id: TabId,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkErrorMessage {
    pub request_id: u64,
    pub tab_id: TabId,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuCommandMessage {
    pub command_id: u64,
    pub command_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuResponseMessage {
    pub command_id: u64,
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequestMessage {
    pub request_id: u64,
    pub tab_id: TabId,
    pub origin: String,
    pub permission: Permission,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionResponseMessage {
    pub request_id: u64,
    pub tab_id: TabId,
    pub permission: Permission,
    pub state: PermissionState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingMessage {
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PongMessage {
    pub timestamp: std::time::SystemTime,
    pub response_time: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownMessage {
    pub reason: String,
    pub graceful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub error_code: String,
    pub error_message: String,
    pub context: Option<String>,
}

/// IPC connection trait
pub trait IpcConnectionTrait: Send + Sync {
    fn send(&self, envelope: IpcEnvelope) -> crate::Result<()>;
    fn receive(&self) -> crate::Result<Option<IpcEnvelope>>;
    fn close(&self) -> crate::Result<()>;
}

/// IPC message handler trait
#[async_trait::async_trait]
pub trait IpcHandler: Send + Sync {
    async fn handle_message(&self, envelope: IpcEnvelope) -> crate::Result<()>;
}

/// IPC message router
pub struct IpcRouter {
    handlers: std::collections::HashMap<String, std::sync::Arc<dyn IpcHandler + Send + Sync>>,
}

impl IpcRouter {
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }

    pub fn register_handler(&mut self, process_name: String, handler: std::sync::Arc<dyn IpcHandler + Send + Sync>) {
        self.handlers.insert(process_name, handler);
    }

    pub async fn route_message(&self, envelope: IpcEnvelope) -> crate::Result<()> {
        if let Some(handler) = self.handlers.get(&envelope.target_process) {
            handler.handle_message(envelope).await
        } else {
            Err(crate::error::Error::IpcError(format!(
                "No handler registered for process: {}",
                envelope.target_process
            )))
        }
    }
}

impl Default for IpcRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// IPC message builder for common operations
pub struct IpcMessageBuilder {
    next_message_id: u64,
    source_process: String,
}

impl IpcMessageBuilder {
    pub fn new(source_process: String) -> Self {
        Self {
            next_message_id: 1,
            source_process,
        }
    }

    pub fn create_tab(&mut self, target_process: String, request: CreateTabRequest) -> IpcEnvelope {
        IpcEnvelope::new(
            self.next_message_id(),
            self.source_process.clone(),
            target_process,
            IpcMessage::CreateTab(request),
        )
    }

    pub fn close_tab(&mut self, target_process: String, request: CloseTabRequest) -> IpcEnvelope {
        IpcEnvelope::new(
            self.next_message_id(),
            self.source_process.clone(),
            target_process,
            IpcMessage::CloseTab(request),
        )
    }

    pub fn navigate_tab(&mut self, target_process: String, request: NavigateTabRequest) -> IpcEnvelope {
        IpcEnvelope::new(
            self.next_message_id(),
            self.source_process.clone(),
            target_process,
            IpcMessage::NavigateTab(request),
        )
    }

    pub fn ping(&mut self, target_process: String) -> IpcEnvelope {
        IpcEnvelope::new(
            self.next_message_id(),
            self.source_process.clone(),
            target_process,
            IpcMessage::Ping(PingMessage {
                timestamp: std::time::SystemTime::now(),
            }),
        )
    }

    fn next_message_id(&mut self) -> u64 {
        let id = self.next_message_id;
        self.next_message_id += 1;
        id
    }
}

// IPC Implementation Components

/// IPC connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Disconnected,
    Error(String),
}

/// IPC connection for communication between processes
#[derive(Clone)]
pub struct IpcConnection {
    name: String,
    state: std::sync::Arc<tokio::sync::RwLock<ConnectionState>>,
    sender: tokio::sync::mpsc::UnboundedSender<IpcEnvelope>,
    receiver: std::sync::Arc<tokio::sync::RwLock<Option<tokio::sync::mpsc::UnboundedReceiver<IpcEnvelope>>>>,
}

impl IpcConnection {
    /// Create a new IPC connection
    pub fn new(name: String) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Self {
            name,
            state: std::sync::Arc::new(tokio::sync::RwLock::new(ConnectionState::Connecting)),
            sender,
            receiver: std::sync::Arc::new(tokio::sync::RwLock::new(Some(receiver))),
        }
    }

    /// Get the connection name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the current connection state
    pub async fn state(&self) -> ConnectionState {
        self.state.read().await.clone()
    }

    /// Set the connection state
    pub async fn set_state(&self, state: ConnectionState) {
        let mut current_state = self.state.write().await;
        *current_state = state.clone();
        tracing::debug!("Connection {} state changed to {:?}", self.name, state);
    }

    /// Send a message through this connection
    pub async fn send_message(&self, message: IpcMessage) -> crate::Result<()> {
        let state = self.state().await;
        if state != ConnectionState::Connected {
            return Err(crate::error::Error::InvalidState(format!(
                "Cannot send message: connection is in {:?} state",
                state
            )));
        }

        let envelope = IpcEnvelope::new(
            self.generate_message_id(),
            self.name.clone(),
            "target".to_string(), // TODO: Make this configurable
            message,
        );

        self.sender
            .send(envelope)
            .map_err(|e| crate::error::Error::IoError(format!("Failed to send message: {}", e)))?;

        tracing::debug!("Sent message through connection {}", self.name);
        Ok(())
    }

    /// Receive a message from this connection
    pub async fn receive_message(&self) -> crate::Result<Option<IpcEnvelope>> {
        let mut receiver = self.receiver.write().await;
        if let Some(ref mut rx) = *receiver {
            match rx.try_recv() {
                Ok(envelope) => {
                    tracing::debug!("Received message from connection {}", self.name);
                    Ok(Some(envelope))
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => Ok(None),
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                    self.set_state(ConnectionState::Disconnected).await;
                    Err(crate::error::Error::IoError("Connection disconnected".to_string()))
                }
            }
        } else {
            Err(crate::error::Error::InvalidState("No receiver available".to_string()))
        }
    }

    /// Connect the connection
    pub async fn connect(&self) -> crate::Result<()> {
        self.set_state(ConnectionState::Connected).await;
        tracing::info!("Connection {} established", self.name);
        Ok(())
    }

    /// Disconnect the connection
    pub async fn disconnect(&self) -> crate::Result<()> {
        self.set_state(ConnectionState::Disconnected).await;
        tracing::info!("Connection {} disconnected", self.name);
        Ok(())
    }

    /// Check if the connection is connected
    pub async fn is_connected(&self) -> bool {
        self.state().await == ConnectionState::Connected
    }

    /// Generate a unique message ID
    fn generate_message_id(&self) -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static MESSAGE_COUNTER: AtomicU64 = AtomicU64::new(0);
        MESSAGE_COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

/// IPC connection manager for handling multiple connections
pub struct IpcManager {
    connections: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, IpcConnection>>>,
    router: std::sync::Arc<IpcRouter>,
}

impl IpcManager {
    pub fn new() -> Self {
        Self {
            connections: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            router: std::sync::Arc::new(IpcRouter::new()),
        }
    }

    /// Add a new connection to the manager
    pub async fn add_connection(&self, name: String, connection: IpcConnection) -> crate::Result<()> {
        let mut connections = self.connections.write().await;
        connections.insert(name, connection);
        Ok(())
    }

    /// Remove a connection from the manager
    pub async fn remove_connection(&self, name: &str) -> crate::Result<()> {
        let mut connections = self.connections.write().await;
        connections.remove(name);
        Ok(())
    }

    /// Get a connection by name
    pub async fn get_connection(&self, name: &str) -> crate::Result<IpcConnection> {
        let connections = self.connections.read().await;
        connections
            .get(name)
            .cloned()
            .ok_or_else(|| crate::error::Error::NotFound(format!("Connection '{}' not found", name)))
    }

    /// Send a message to a specific connection
    pub async fn send_message(&self, connection_name: &str, message: IpcMessage) -> crate::Result<()> {
        let connection = self.get_connection(connection_name).await?;
        connection.send_message(message).await
    }

    /// Broadcast a message to all connections
    pub async fn broadcast_message(&self, message: IpcMessage) -> crate::Result<()> {
        let connections = self.connections.read().await;
        for connection in connections.values() {
            if let Err(e) = connection.send_message(message.clone()).await {
                tracing::warn!("Failed to broadcast message: {}", e);
            }
        }
        Ok(())
    }

    /// Get the router for handling incoming messages
    pub fn router(&self) -> std::sync::Arc<IpcRouter> {
        self.router.clone()
    }
}

impl Default for IpcManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_envelope() {
        let message = IpcMessage::Ping(PingMessage {
            timestamp: std::time::SystemTime::now(),
        });
        let envelope = IpcEnvelope::new(1, "browser".to_string(), "renderer".to_string(), message);
        
        assert_eq!(envelope.message_id, 1);
        assert_eq!(envelope.source_process, "browser");
        assert_eq!(envelope.target_process, "renderer");
        assert_eq!(envelope.priority, MessagePriority::Normal);
    }

    #[tokio::test]
    async fn test_ipc_router() {
        let mut router = IpcRouter::new();
        let envelope = IpcEnvelope::new(
            1,
            "browser".to_string(),
            "nonexistent".to_string(),
            IpcMessage::Ping(PingMessage {
                timestamp: std::time::SystemTime::now(),
            }),
        );
        
        let result = router.route_message(envelope).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_ipc_message_builder() {
        let mut builder = IpcMessageBuilder::new("browser".to_string());
        let request = CreateTabRequest {
            window_id: 1,
            url: None,
            incognito: false,
        };
        
        let envelope = builder.create_tab("renderer".to_string(), request);
        assert_eq!(envelope.message_id, 1);
        assert_eq!(envelope.source_process, "browser");
        assert_eq!(envelope.target_process, "renderer");
    }

    #[tokio::test]
    async fn test_ipc_connection() {
        let connection = IpcConnection::new("test".to_string());
        
        // Test initial state
        assert_eq!(connection.state().await, ConnectionState::Connecting);
        
        // Test connection
        connection.connect().await.unwrap();
        assert_eq!(connection.state().await, ConnectionState::Connected);
        assert!(connection.is_connected().await);
        
        // Test disconnection
        connection.disconnect().await.unwrap();
        assert_eq!(connection.state().await, ConnectionState::Disconnected);
        assert!(!connection.is_connected().await);
    }

    #[tokio::test]
    async fn test_ipc_manager() {
        let manager = IpcManager::new();
        
        // Test adding and getting a connection
        let connection = IpcConnection::new("test".to_string());
        manager.add_connection("test".to_string(), connection).await.unwrap();
        
        let retrieved = manager.get_connection("test").await.unwrap();
        assert_eq!(retrieved.name(), "test");
        
        // Test removing a connection
        manager.remove_connection("test").await.unwrap();
        assert!(manager.get_connection("test").await.is_err());
    }
}
