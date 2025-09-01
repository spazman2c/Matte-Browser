use crate::error::{Error, Result};
use std::collections::HashMap;
use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};
use tokio::net::TcpStream as TokioTcpStream;
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, timeout};
use std::sync::Arc;
use parking_lot::RwLock;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;

/// HTTP methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

/// HTTP versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http1_0,
    Http1_1,
    Http2_0,
}

/// HTTP status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Continue = 100,
    SwitchingProtocols = 101,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    Found = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
}

/// HTTP headers
#[derive(Debug, Clone)]
pub struct HttpHeaders {
    headers: HashMap<String, String>,
}

/// HTTP request
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// HTTP method
    pub method: HttpMethod,
    /// Request URI
    pub uri: String,
    /// HTTP version
    pub version: HttpVersion,
    /// Request headers
    pub headers: HttpHeaders,
    /// Request body
    pub body: Option<Vec<u8>>,
    /// Request timeout
    pub timeout: Option<Duration>,
    /// Whether to follow redirects
    pub follow_redirects: bool,
    /// Maximum redirects to follow
    pub max_redirects: usize,
}

/// HTTP response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// HTTP version
    pub version: HttpVersion,
    /// Status code
    pub status: HttpStatus,
    /// Status text
    pub status_text: String,
    /// Response headers
    pub headers: HttpHeaders,
    /// Response body
    pub body: Vec<u8>,
    /// Response size
    pub content_length: Option<usize>,
    /// Whether response is chunked
    pub chunked: bool,
    /// Response timestamp
    pub timestamp: Instant,
}

/// HTTP connection
pub struct HttpConnection {
    /// Underlying TCP stream
    stream: TcpStream,
    /// Connection address
    address: SocketAddr,
    /// Connection timeout
    timeout: Duration,
    /// Whether connection is keep-alive
    keep_alive: bool,
    /// Last used timestamp
    last_used: Instant,
    /// Connection ID
    id: u64,
}

/// HTTP connection pool
pub struct HttpConnectionPool {
    /// Available connections
    connections: Arc<RwLock<HashMap<String, Vec<HttpConnection>>>>,
    /// Connection timeout
    timeout: Duration,
    /// Maximum connections per host
    max_connections_per_host: usize,
    /// Maximum total connections
    max_total_connections: usize,
    /// Connection pool stats
    stats: Arc<RwLock<ConnectionPoolStats>>,
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct ConnectionPoolStats {
    /// Total connections created
    pub total_connections: usize,
    /// Active connections
    pub active_connections: usize,
    /// Idle connections
    pub idle_connections: usize,
    /// Connection creation time
    pub avg_creation_time: Duration,
    /// Connection reuse count
    pub reuse_count: usize,
}

/// HTTP client
pub struct HttpClient {
    /// Connection pool
    connection_pool: HttpConnectionPool,
    /// Default timeout
    default_timeout: Duration,
    /// Default user agent
    user_agent: String,
    /// Default headers
    default_headers: HttpHeaders,
    /// Whether to follow redirects by default
    follow_redirects: bool,
    /// Maximum redirects to follow by default
    max_redirects: usize,
}

/// HTTP/2 specific structures
#[derive(Debug, Clone)]
pub struct Http2Settings {
    /// Header table size
    pub header_table_size: u32,
    /// Enable push
    pub enable_push: bool,
    /// Max concurrent streams
    pub max_concurrent_streams: u32,
    /// Initial window size
    pub initial_window_size: u32,
    /// Max frame size
    pub max_frame_size: u32,
    /// Max header list size
    pub max_header_list_size: u32,
}

/// HTTP/2 frame types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Http2FrameType {
    Data = 0,
    Headers = 1,
    Priority = 2,
    RstStream = 3,
    Settings = 4,
    PushPromise = 5,
    Ping = 6,
    Goaway = 7,
    WindowUpdate = 8,
    Continuation = 9,
}

/// HTTP/2 frame
#[derive(Debug, Clone)]
pub struct Http2Frame {
    /// Frame length
    pub length: u32,
    /// Frame type
    pub frame_type: Http2FrameType,
    /// Frame flags
    pub flags: u8,
    /// Stream identifier
    pub stream_id: u32,
    /// Frame payload
    pub payload: Vec<u8>,
}

/// HTTP/2 stream
#[derive(Debug)]
pub struct Http2Stream {
    /// Stream ID
    pub id: u32,
    /// Stream state
    pub state: Http2StreamState,
    /// Stream priority
    pub priority: u8,
    /// Stream window size
    pub window_size: u32,
    /// Stream headers
    pub headers: HttpHeaders,
    /// Stream data
    pub data: Vec<u8>,
}

/// HTTP/2 stream states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Http2StreamState {
    Idle,
    ReservedLocal,
    ReservedRemote,
    Open,
    HalfClosedLocal,
    HalfClosedRemote,
    Closed,
}

/// HTTP/2 connection
pub struct Http2Connection {
    /// Underlying HTTP/1.1 connection
    http1_connection: HttpConnection,
    /// HTTP/2 settings
    settings: Http2Settings,
    /// Active streams
    streams: HashMap<u32, Http2Stream>,
    /// Next stream ID
    next_stream_id: u32,
    /// Connection window size
    connection_window_size: u32,
    /// Whether connection is established
    established: bool,
}

/// HPACK encoder/decoder
pub struct HpackCodec {
    /// Dynamic table
    dynamic_table: Vec<(String, String)>,
    /// Maximum table size
    max_table_size: usize,
    /// Current table size
    current_table_size: usize,
}

impl HttpMethod {
    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::CONNECT => "CONNECT",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            "HEAD" => Some(HttpMethod::HEAD),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            "PATCH" => Some(HttpMethod::PATCH),
            "TRACE" => Some(HttpMethod::TRACE),
            "CONNECT" => Some(HttpMethod::CONNECT),
            _ => None,
        }
    }
}

impl HttpVersion {
    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpVersion::Http1_0 => "HTTP/1.0",
            HttpVersion::Http1_1 => "HTTP/1.1",
            HttpVersion::Http2_0 => "HTTP/2.0",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "HTTP/1.0" => Some(HttpVersion::Http1_0),
            "HTTP/1.1" => Some(HttpVersion::Http1_1),
            "HTTP/2.0" => Some(HttpVersion::Http2_0),
            _ => None,
        }
    }
}

impl HttpStatus {
    /// Get status text
    pub fn status_text(&self) -> &'static str {
        match self {
            HttpStatus::Continue => "Continue",
            HttpStatus::SwitchingProtocols => "Switching Protocols",
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::NoContent => "No Content",
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Found",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::RequestTimeout => "Request Timeout",
            HttpStatus::Conflict => "Conflict",
            HttpStatus::Gone => "Gone",
            HttpStatus::LengthRequired => "Length Required",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            HttpStatus::UriTooLong => "URI Too Long",
            HttpStatus::UnsupportedMediaType => "Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "Expectation Failed",
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::GatewayTimeout => "Gateway Timeout",
            HttpStatus::HttpVersionNotSupported => "HTTP Version Not Supported",
        }
    }

    /// Check if status is informational
    pub fn is_informational(&self) -> bool {
        matches!(self, HttpStatus::Continue | HttpStatus::SwitchingProtocols)
    }

    /// Check if status is successful
    pub fn is_success(&self) -> bool {
        matches!(self, HttpStatus::Ok | HttpStatus::Created | HttpStatus::Accepted | HttpStatus::NoContent)
    }

    /// Check if status is redirection
    pub fn is_redirection(&self) -> bool {
        matches!(self, HttpStatus::MovedPermanently | HttpStatus::Found | HttpStatus::NotModified)
    }

    /// Check if status is client error
    pub fn is_client_error(&self) -> bool {
        matches!(self, HttpStatus::BadRequest | HttpStatus::Unauthorized | HttpStatus::Forbidden | 
                       HttpStatus::NotFound | HttpStatus::MethodNotAllowed | HttpStatus::RequestTimeout |
                       HttpStatus::Conflict | HttpStatus::Gone | HttpStatus::LengthRequired |
                       HttpStatus::PayloadTooLarge | HttpStatus::UriTooLong | HttpStatus::UnsupportedMediaType |
                       HttpStatus::RangeNotSatisfiable | HttpStatus::ExpectationFailed)
    }

    /// Check if status is server error
    pub fn is_server_error(&self) -> bool {
        matches!(self, HttpStatus::InternalServerError | HttpStatus::NotImplemented | HttpStatus::BadGateway |
                       HttpStatus::ServiceUnavailable | HttpStatus::GatewayTimeout | HttpStatus::HttpVersionNotSupported)
    }
}

impl HttpHeaders {
    /// Create new headers
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    /// Set header
    pub fn set(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_lowercase(), value.to_string());
    }

    /// Get header
    pub fn get(&self, name: &str) -> Option<&String> {
        self.headers.get(&name.to_lowercase())
    }

    /// Remove header
    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.headers.remove(&name.to_lowercase())
    }

    /// Check if header exists
    pub fn contains(&self, name: &str) -> bool {
        self.headers.contains_key(&name.to_lowercase())
    }

    /// Get all headers
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.headers.iter()
    }

    /// Get content length
    pub fn content_length(&self) -> Option<usize> {
        self.get("content-length")
            .and_then(|s| s.parse().ok())
    }

    /// Get transfer encoding
    pub fn transfer_encoding(&self) -> Option<&String> {
        self.get("transfer-encoding")
    }

    /// Check if chunked
    pub fn is_chunked(&self) -> bool {
        self.transfer_encoding()
            .map(|s| s.to_lowercase().contains("chunked"))
            .unwrap_or(false)
    }

    /// Get connection type
    pub fn connection(&self) -> Option<&String> {
        self.get("connection")
    }

    /// Check if keep-alive
    pub fn is_keep_alive(&self) -> bool {
        self.connection()
            .map(|s| s.to_lowercase().contains("keep-alive"))
            .unwrap_or(false)
    }

    /// Get host
    pub fn host(&self) -> Option<&String> {
        self.get("host")
    }

    /// Get user agent
    pub fn user_agent(&self) -> Option<&String> {
        self.get("user-agent")
    }
}

impl Default for HttpHeaders {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpRequest {
    /// Create new request
    pub fn new(method: HttpMethod, uri: String) -> Self {
        Self {
            method,
            uri,
            version: HttpVersion::Http1_1,
            headers: HttpHeaders::new(),
            body: None,
            timeout: None,
            follow_redirects: true,
            max_redirects: 5,
        }
    }

    /// Set method
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }

    /// Set URI
    pub fn uri(mut self, uri: String) -> Self {
        self.uri = uri;
        self
    }

    /// Set version
    pub fn version(mut self, version: HttpVersion) -> Self {
        self.version = version;
        self
    }

    /// Set header
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.set(name, value);
        self
    }

    /// Set body
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// Set timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set follow redirects
    pub fn follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }

    /// Set max redirects
    pub fn max_redirects(mut self, max: usize) -> Self {
        self.max_redirects = max;
        self
    }

    /// Serialize request
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut request = Vec::new();
        
        // Request line
        let request_line = format!("{} {} {}\r\n", 
            self.method.as_str(), 
            self.uri, 
            self.version.as_str()
        );
        request.extend_from_slice(request_line.as_bytes());
        
        // Headers
        for (name, value) in self.headers.iter() {
            let header_line = format!("{}: {}\r\n", name, value);
            request.extend_from_slice(header_line.as_bytes());
        }
        
        // Empty line
        request.extend_from_slice(b"\r\n");
        
        // Body
        if let Some(ref body) = self.body {
            request.extend_from_slice(body);
        }
        
        Ok(request)
    }
}

impl HttpResponse {
    /// Create new response
    pub fn new(version: HttpVersion, status: HttpStatus) -> Self {
        Self {
            version,
            status,
            status_text: status.status_text().to_string(),
            headers: HttpHeaders::new(),
            body: Vec::new(),
            content_length: None,
            chunked: false,
            timestamp: Instant::now(),
        }
    }

    /// Set status
    pub fn status(mut self, status: HttpStatus) -> Self {
        self.status = status;
        self.status_text = status.status_text().to_string();
        self
    }

    /// Set header
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.set(name, value);
        self
    }

    /// Set body
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self.content_length = Some(self.body.len());
        self
    }

    /// Check if response is successful
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// Check if response is redirection
    pub fn is_redirection(&self) -> bool {
        self.status.is_redirection()
    }

    /// Check if response is client error
    pub fn is_client_error(&self) -> bool {
        self.status.is_client_error()
    }

    /// Check if response is server error
    pub fn is_server_error(&self) -> bool {
        self.status.is_server_error()
    }

    /// Get redirect location
    pub fn redirect_location(&self) -> Option<&String> {
        if self.is_redirection() {
            self.headers.get("location")
        } else {
            None
        }
    }

    /// Parse response from bytes
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut lines = data.split(|&b| b == b'\n');
        
        // Parse status line
        let status_line = lines.next()
            .ok_or_else(|| Error::parsing("Missing status line".to_string()))?;
        let status_line = std::str::from_utf8(status_line)
            .map_err(|e| Error::parsing(format!("Invalid status line: {}", e)))?;
        
        let parts: Vec<&str> = status_line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(Error::parsing("Invalid status line format".to_string()));
        }
        
        let version = HttpVersion::from_str(parts[0])
            .ok_or_else(|| Error::parsing("Invalid HTTP version".to_string()))?;
        
        let status_code: u16 = parts[1].parse()
            .map_err(|e| Error::parsing(format!("Invalid status code: {}", e)))?;
        
        let status = match status_code {
            100 => HttpStatus::Continue,
            101 => HttpStatus::SwitchingProtocols,
            200 => HttpStatus::Ok,
            201 => HttpStatus::Created,
            202 => HttpStatus::Accepted,
            204 => HttpStatus::NoContent,
            301 => HttpStatus::MovedPermanently,
            302 => HttpStatus::Found,
            304 => HttpStatus::NotModified,
            400 => HttpStatus::BadRequest,
            401 => HttpStatus::Unauthorized,
            403 => HttpStatus::Forbidden,
            404 => HttpStatus::NotFound,
            405 => HttpStatus::MethodNotAllowed,
            408 => HttpStatus::RequestTimeout,
            409 => HttpStatus::Conflict,
            410 => HttpStatus::Gone,
            411 => HttpStatus::LengthRequired,
            413 => HttpStatus::PayloadTooLarge,
            414 => HttpStatus::UriTooLong,
            415 => HttpStatus::UnsupportedMediaType,
            416 => HttpStatus::RangeNotSatisfiable,
            417 => HttpStatus::ExpectationFailed,
            500 => HttpStatus::InternalServerError,
            501 => HttpStatus::NotImplemented,
            502 => HttpStatus::BadGateway,
            503 => HttpStatus::ServiceUnavailable,
            504 => HttpStatus::GatewayTimeout,
            505 => HttpStatus::HttpVersionNotSupported,
            _ => return Err(Error::parsing(format!("Unknown status code: {}", status_code))),
        };
        
        let status_text = parts[2..].join(" ");
        
        let mut response = HttpResponse::new(version, status);
        response.status_text = status_text;
        
        // Parse headers
        let mut body_start = 0;
        for (i, line) in lines.enumerate() {
            if line.is_empty() || (line.len() == 1 && line[0] == b'\r') {
                body_start = i + 1;
                break;
            }
            
            let line = std::str::from_utf8(line)
                .map_err(|e| Error::parsing(format!("Invalid header line: {}", e)))?;
            
            if let Some(colon_pos) = line.find(':') {
                let name = line[..colon_pos].trim();
                let value = line[colon_pos + 1..].trim();
                response.headers.set(name, value);
            }
        }
        
        // Parse body
        if body_start > 0 {
            let body_data = &data[body_start..];
            response.body = body_data.to_vec();
            response.content_length = Some(response.body.len());
        }
        
        // Check for chunked encoding
        response.chunked = response.headers.is_chunked();
        
        Ok(response)
    }
}

impl HttpConnection {
    /// Create new connection
    pub fn new(stream: TcpStream, address: SocketAddr, timeout: Duration) -> Self {
        Self {
            stream,
            address,
            timeout,
            keep_alive: true,
            last_used: Instant::now(),
            id: 0, // Will be set by connection pool
        }
    }

    /// Send request
    pub fn send_request(&mut self, request: &HttpRequest) -> Result<()> {
        let request_data = request.serialize()?;
        self.stream.write_all(&request_data)?;
        self.stream.flush()?;
        self.last_used = Instant::now();
        Ok(())
    }

    /// Receive response
    pub fn receive_response(&mut self) -> Result<HttpResponse> {
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(&self.stream);
        
        // Read headers
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            buffer.extend_from_slice(line.as_bytes());
            
            // Check for end of headers
            if line.trim().is_empty() {
                break;
            }
            
            line.clear();
        }
        
        // Parse response
        let response = HttpResponse::parse(&buffer)?;
        
        // Read body if present
        if let Some(content_length) = response.content_length {
            let mut body = vec![0; content_length];
            reader.read_exact(&mut body)?;
            // Note: This is simplified - in practice we'd need to handle the body properly
        }
        
        self.last_used = Instant::now();
        Ok(response)
    }

    /// Check if connection is expired
    pub fn is_expired(&self, max_idle_time: Duration) -> bool {
        self.last_used.elapsed() > max_idle_time
    }

    /// Get connection ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Set connection ID
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }
}

impl HttpConnectionPool {
    /// Create new connection pool
    pub fn new(timeout: Duration, max_connections_per_host: usize, max_total_connections: usize) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            timeout,
            max_connections_per_host,
            max_total_connections,
            stats: Arc::new(RwLock::new(ConnectionPoolStats {
                total_connections: 0,
                active_connections: 0,
                idle_connections: 0,
                avg_creation_time: Duration::from_millis(0),
                reuse_count: 0,
            })),
        }
    }

    /// Get connection for host
    pub async fn get_connection(&self, host: &str, port: u16) -> Result<HttpConnection> {
        let key = format!("{}:{}", host, port);
        
        // Try to get existing connection
        {
            let mut connections = self.connections.write();
            if let Some(host_connections) = connections.get_mut(&key) {
                if let Some(connection) = host_connections.pop() {
                    if !connection.is_expired(self.timeout) {
                        let mut stats = self.stats.write();
                        stats.reuse_count += 1;
                        stats.active_connections += 1;
                        return Ok(connection);
                    }
                }
            }
        }
        
        // Create new connection
        let start_time = Instant::now();
        let addr = format!("{}:{}", host, port)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| Error::parsing("Failed to resolve address".to_string()))?;
        
        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(self.timeout))?;
        stream.set_write_timeout(Some(self.timeout))?;
        
        let mut connection = HttpConnection::new(stream, addr, self.timeout);
        connection.set_id(self.generate_connection_id());
        
        // Update stats
        {
            let mut stats = self.stats.write();
            stats.total_connections += 1;
            stats.active_connections += 1;
            stats.avg_creation_time = (stats.avg_creation_time + start_time.elapsed()) / 2;
        }
        
        Ok(connection)
    }

    /// Return connection to pool
    pub fn return_connection(&self, connection: HttpConnection) {
        let key = format!("{}:{}", connection.address.ip(), connection.address.port());
        
        let mut connections = self.connections.write();
        let host_connections = connections.entry(key).or_insert_with(Vec::new);
        
        if host_connections.len() < self.max_connections_per_host {
            host_connections.push(connection);
            
            let mut stats = self.stats.write();
            stats.active_connections = stats.active_connections.saturating_sub(1);
            stats.idle_connections += 1;
        }
    }

    /// Clean up expired connections
    pub fn cleanup_expired(&self) {
        let mut connections = self.connections.write();
        let mut stats = self.stats.write();
        
        for host_connections in connections.values_mut() {
            host_connections.retain(|conn| !conn.is_expired(self.timeout));
        }
        
        // Update stats
        stats.idle_connections = connections.values()
            .map(|conns| conns.len())
            .sum();
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> ConnectionPoolStats {
        self.stats.read().clone()
    }

    /// Generate unique connection ID
    fn generate_connection_id(&self) -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

impl HttpClient {
    /// Create new HTTP client
    pub fn new() -> Self {
        let connection_pool = HttpConnectionPool::new(
            Duration::from_secs(30),
            10, // max connections per host
            100, // max total connections
        );
        
        let mut default_headers = HttpHeaders::new();
        default_headers.set("User-Agent", "Matte-Browser/1.0");
        default_headers.set("Accept", "*/*");
        default_headers.set("Connection", "keep-alive");
        
        Self {
            connection_pool,
            default_timeout: Duration::from_secs(30),
            user_agent: "Matte-Browser/1.0".to_string(),
            default_headers,
            follow_redirects: true,
            max_redirects: 5,
        }
    }

    /// Send request
    pub async fn send(&self, mut request: HttpRequest) -> Result<HttpResponse> {
        // Apply default headers
        for (name, value) in self.default_headers.iter() {
            if !request.headers.contains(name) {
                request.headers.set(name, value);
            }
        }
        
        // Apply default timeout
        if request.timeout.is_none() {
            request.timeout = Some(self.default_timeout);
        }
        
        // Parse URI to get host and port
        let (host, port) = self.parse_uri(&request.uri)?;
        
        let mut redirect_count = 0;
        let mut current_request = request;
        
        loop {
            // Get connection
            let mut connection = self.connection_pool.get_connection(&host, port).await?;
            
            // Send request
            connection.send_request(&current_request)?;
            
            // Receive response
            let response = connection.receive_response()?;
            
            // Return connection to pool
            self.connection_pool.return_connection(connection);
            
            // Handle redirects
            if response.is_redirection() && current_request.follow_redirects && redirect_count < current_request.max_redirects {
                if let Some(location) = response.redirect_location() {
                    redirect_count += 1;
                    current_request.uri = location.clone();
                    continue;
                }
            }
            
            return Ok(response);
        }
    }

    /// Parse URI to extract host and port
    fn parse_uri(&self, uri: &str) -> Result<(String, u16)> {
        if uri.starts_with("http://") {
            let uri = &uri[7..];
            if let Some(colon_pos) = uri.find(':') {
                let host = uri[..colon_pos].to_string();
                let port = uri[colon_pos + 1..].parse()
                    .map_err(|e| Error::parsing(format!("Invalid port: {}", e)))?;
                Ok((host, port))
            } else {
                Ok((uri.to_string(), 80))
            }
        } else if uri.starts_with("https://") {
            let uri = &uri[8..];
            if let Some(colon_pos) = uri.find(':') {
                let host = uri[..colon_pos].to_string();
                let port = uri[colon_pos + 1..].parse()
                    .map_err(|e| Error::parsing(format!("Invalid port: {}", e)))?;
                Ok((host, port))
            } else {
                Ok((uri.to_string(), 443))
            }
        } else {
            // Assume HTTP if no scheme
            if let Some(colon_pos) = uri.find(':') {
                let host = uri[..colon_pos].to_string();
                let port = uri[colon_pos + 1..].parse()
                    .map_err(|e| Error::parsing(format!("Invalid port: {}", e)))?;
                Ok((host, port))
            } else {
                Ok((uri.to_string(), 80))
            }
        }
    }

    /// Set default timeout
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Set user agent
    pub fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
        self.default_headers.set("User-Agent", &self.user_agent);
    }

    /// Set default header
    pub fn set_default_header(&mut self, name: &str, value: &str) {
        self.default_headers.set(name, value);
    }

    /// Set follow redirects
    pub fn set_follow_redirects(&mut self, follow: bool) {
        self.follow_redirects = follow;
    }

    /// Set max redirects
    pub fn set_max_redirects(&mut self, max: usize) {
        self.max_redirects = max;
    }

    /// Get connection pool stats
    pub fn get_connection_pool_stats(&self) -> ConnectionPoolStats {
        self.connection_pool.get_stats()
    }

    /// Clean up expired connections
    pub fn cleanup_expired_connections(&self) {
        self.connection_pool.cleanup_expired();
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl Http2Settings {
    /// Create default HTTP/2 settings
    pub fn default() -> Self {
        Self {
            header_table_size: 4096,
            enable_push: true,
            max_concurrent_streams: 100,
            initial_window_size: 65535,
            max_frame_size: 16384,
            max_header_list_size: 262144,
        }
    }
}

impl Http2Frame {
    /// Create new frame
    pub fn new(frame_type: Http2FrameType, stream_id: u32, payload: Vec<u8>) -> Self {
        Self {
            length: payload.len() as u32,
            frame_type,
            flags: 0,
            stream_id,
            payload,
        }
    }

    /// Serialize frame
    pub fn serialize(&self) -> Vec<u8> {
        let mut frame = Vec::new();
        
        // Frame header (9 bytes)
        frame.extend_from_slice(&(self.length >> 16) as u8);
        frame.extend_from_slice(&(self.length >> 8) as u8);
        frame.extend_from_slice(&(self.length) as u8);
        frame.push(self.frame_type as u8);
        frame.push(self.flags);
        frame.extend_from_slice(&(self.stream_id >> 24) as u8);
        frame.extend_from_slice(&(self.stream_id >> 16) as u8);
        frame.extend_from_slice(&(self.stream_id >> 8) as u8);
        frame.extend_from_slice(&(self.stream_id) as u8);
        
        // Frame payload
        frame.extend_from_slice(&self.payload);
        
        frame
    }

    /// Parse frame from bytes
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < 9 {
            return Err(Error::parsing("Frame too short".to_string()));
        }
        
        let length = ((data[0] as u32) << 16) | ((data[1] as u32) << 8) | (data[2] as u32);
        let frame_type = data[3];
        let flags = data[4];
        let stream_id = ((data[5] as u32) << 24) | ((data[6] as u32) << 16) | ((data[7] as u32) << 8) | (data[8] as u32);
        
        let frame_type = match frame_type {
            0 => Http2FrameType::Data,
            1 => Http2FrameType::Headers,
            2 => Http2FrameType::Priority,
            3 => Http2FrameType::RstStream,
            4 => Http2FrameType::Settings,
            5 => Http2FrameType::PushPromise,
            6 => Http2FrameType::Ping,
            7 => Http2FrameType::Goaway,
            8 => Http2FrameType::WindowUpdate,
            9 => Http2FrameType::Continuation,
            _ => return Err(Error::parsing(format!("Unknown frame type: {}", frame_type))),
        };
        
        let payload = if data.len() > 9 {
            data[9..].to_vec()
        } else {
            Vec::new()
        };
        
        Ok(Http2Frame {
            length,
            frame_type,
            flags,
            stream_id,
            payload,
        })
    }
}

impl HpackCodec {
    /// Create new HPACK codec
    pub fn new(max_table_size: usize) -> Self {
        Self {
            dynamic_table: Vec::new(),
            max_table_size,
            current_table_size: 0,
        }
    }

    /// Encode headers
    pub fn encode(&mut self, headers: &HttpHeaders) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        
        for (name, value) in headers.iter() {
            // Simple literal encoding for now
            encoded.extend_from_slice(&[0x40]); // Literal header field with incremental indexing
            encoded.extend_from_slice(&(name.len() as u8));
            encoded.extend_from_slice(name.as_bytes());
            encoded.extend_from_slice(&(value.len() as u8));
            encoded.extend_from_slice(value.as_bytes());
        }
        
        Ok(encoded)
    }

    /// Decode headers
    pub fn decode(&mut self, data: &[u8]) -> Result<HttpHeaders> {
        let mut headers = HttpHeaders::new();
        let mut i = 0;
        
        while i < data.len() {
            let byte = data[i];
            i += 1;
            
            if byte & 0x80 != 0 {
                // Indexed header field
                let index = byte & 0x7F;
                // For now, just skip indexed headers
                continue;
            } else if byte & 0x40 != 0 {
                // Literal header field with incremental indexing
                let name_len = data[i] as usize;
                i += 1;
                let name = std::str::from_utf8(&data[i..i + name_len])
                    .map_err(|e| Error::parsing(format!("Invalid header name: {}", e)))?;
                i += name_len;
                
                let value_len = data[i] as usize;
                i += 1;
                let value = std::str::from_utf8(&data[i..i + value_len])
                    .map_err(|e| Error::parsing(format!("Invalid header value: {}", e)))?;
                i += value_len;
                
                headers.set(name, value);
            } else {
                // Literal header field without indexing
                let name_len = data[i] as usize;
                i += 1;
                let name = std::str::from_utf8(&data[i..i + name_len])
                    .map_err(|e| Error::parsing(format!("Invalid header name: {}", e)))?;
                i += name_len;
                
                let value_len = data[i] as usize;
                i += 1;
                let value = std::str::from_utf8(&data[i..i + value_len])
                    .map_err(|e| Error::parsing(format!("Invalid header value: {}", e)))?;
                i += value_len;
                
                headers.set(name, value);
            }
        }
        
        Ok(headers)
    }
}
