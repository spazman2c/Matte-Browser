//! Common utility functions and helpers.

use crate::error::{Error, Result};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Generate a unique identifier
pub fn generate_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Generate a UUID-like string
pub fn generate_uuid() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let id = generate_id();
    
    (timestamp, id).hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Parse a URL string into components
pub fn parse_url(url_str: &str) -> Result<HashMap<String, String>> {
    let url = url::Url::parse(url_str)
        .map_err(|e| Error::ParseError(format!("Invalid URL: {}", e)))?;
    
    let mut components = HashMap::new();
    components.insert("scheme".to_string(), url.scheme().to_string());
    
    if let Some(host) = url.host_str() {
        components.insert("host".to_string(), host.to_string());
    }
    
    if let Some(port) = url.port() {
        components.insert("port".to_string(), port.to_string());
    }
    
    components.insert("path".to_string(), url.path().to_string());
    
    if let Some(query) = url.query() {
        components.insert("query".to_string(), query.to_string());
    }
    
    if let Some(fragment) = url.fragment() {
        components.insert("fragment".to_string(), fragment.to_string());
    }
    
    Ok(components)
}

/// Validate a URL string
pub fn is_valid_url(url_str: &str) -> bool {
    url::Url::parse(url_str).is_ok()
}

/// Extract domain from URL
pub fn extract_domain(url_str: &str) -> Result<String> {
    let url = url::Url::parse(url_str)
        .map_err(|e| Error::ParseError(format!("Invalid URL: {}", e)))?;
    
    url.host_str()
        .map(|host| host.to_string())
        .ok_or_else(|| Error::ParseError("No host found in URL".to_string()))
}

/// Extract path from URL
pub fn extract_path(url_str: &str) -> Result<String> {
    let url = url::Url::parse(url_str)
        .map_err(|e| Error::ParseError(format!("Invalid URL: {}", e)))?;
    
    Ok(url.path().to_string())
}

/// Format bytes into human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Format duration into human-readable string
pub fn format_duration(duration: Duration) -> String {
    if duration.as_secs() < 60 {
        format!("{:.1}s", duration.as_secs_f64())
    } else if duration.as_secs() < 3600 {
        let minutes = duration.as_secs() / 60;
        let seconds = duration.as_secs() % 60;
        format!("{}m {}s", minutes, seconds)
    } else {
        let hours = duration.as_secs() / 3600;
        let minutes = (duration.as_secs() % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }
}

/// Get current timestamp
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Get current timestamp in milliseconds
pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Performance timer
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
    
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
    
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiter
pub struct RateLimiter {
    max_requests: u32,
    window_duration: Duration,
    requests: Vec<Instant>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            max_requests,
            window_duration,
            requests: Vec::new(),
        }
    }
    
    pub fn allow(&mut self) -> bool {
        let now = Instant::now();
        
        // Remove expired requests
        self.requests.retain(|&request_time| {
            now.duration_since(request_time) < self.window_duration
        });
        
        // Check if we can allow another request
        if self.requests.len() < self.max_requests as usize {
            self.requests.push(now);
            true
        } else {
            false
        }
    }
    
    pub fn remaining(&self) -> u32 {
        let now = Instant::now();
        let active_requests = self.requests.iter()
            .filter(|&&request_time| {
                now.duration_since(request_time) < self.window_duration
            })
            .count();
        
        self.max_requests.saturating_sub(active_requests as u32)
    }
}

/// Retry mechanism
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

/// Retry a function with exponential backoff
pub async fn retry<F, Fut, T, E>(
    config: RetryConfig,
    mut f: F,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = std::result::Result<T, E>>,
    E: std::fmt::Display,
{
    let mut delay = config.initial_delay;
    let mut last_error = None;
    
    for attempt in 1..=config.max_attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e.to_string());
                
                if attempt < config.max_attempts {
                    tokio::time::sleep(delay).await;
                    delay = std::cmp::min(
                        Duration::from_secs_f64(delay.as_secs_f64() * config.backoff_multiplier),
                        config.max_delay,
                    );
                }
            }
        }
    }
    
    Err(Error::Unknown(format!(
        "Failed after {} attempts. Last error: {}",
        config.max_attempts,
        last_error.unwrap_or_else(|| "Unknown error".to_string())
    )))
}

/// Cache with TTL (Time To Live)
pub struct Cache<K, V> {
    data: HashMap<K, (V, Instant)>,
    ttl: Duration,
}

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(ttl: Duration) -> Self {
        Self {
            data: HashMap::new(),
            ttl,
        }
    }
    
    pub fn insert(&mut self, key: K, value: V) {
        self.cleanup();
        self.data.insert(key, (value, Instant::now()));
    }
    
    pub fn get(&mut self, key: &K) -> Option<V> {
        self.cleanup();
        
        if let Some((value, timestamp)) = self.data.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value.clone());
            } else {
                self.data.remove(key);
            }
        }
        
        None
    }
    
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key).map(|(value, _)| value)
    }
    
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    pub fn len(&mut self) -> usize {
        self.cleanup();
        self.data.len()
    }
    
    pub fn is_empty(&mut self) -> bool {
        self.cleanup();
        self.data.is_empty()
    }
    
    fn cleanup(&mut self) {
        let now = Instant::now();
        self.data.retain(|_, (_, timestamp)| {
            now.duration_since(*timestamp) < self.ttl
        });
    }
}

/// String utilities
pub mod string {
    
    /// Truncate string to specified length
    pub fn truncate(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }
    
    /// Convert string to title case
    pub fn to_title_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars.map(|c| c.to_lowercase().next().unwrap_or(c))).collect(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    /// Check if string is a valid email
    pub fn is_valid_email(email: &str) -> bool {
        use regex::Regex;
        lazy_static::lazy_static! {
            static ref EMAIL_REGEX: Regex = Regex::new(
                r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
            ).unwrap();
        }
        EMAIL_REGEX.is_match(email)
    }
    
    /// Check if string is a valid URL
    pub fn is_valid_url(url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert!(id2 > id1);
    }

    #[test]
    fn test_generate_uuid() {
        let uuid1 = generate_uuid();
        let uuid2 = generate_uuid();
        assert_ne!(uuid1, uuid2);
        // The hash-based UUID should be 16 characters (64-bit hash = 16 hex chars)
        assert_eq!(uuid1.len(), 16);
        assert_eq!(uuid2.len(), 16);
        // Verify it's a valid hex string
        assert!(uuid1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(uuid2.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_parse_url() {
        let components = parse_url("https://example.com:8080/path?param=value#fragment").unwrap();
        assert_eq!(components.get("scheme").unwrap(), "https");
        assert_eq!(components.get("host").unwrap(), "example.com");
        assert_eq!(components.get("port").unwrap(), "8080");
        assert_eq!(components.get("path").unwrap(), "/path");
        assert_eq!(components.get("query").unwrap(), "param=value");
        assert_eq!(components.get("fragment").unwrap(), "fragment");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30.0s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m");
    }

    #[test]
    fn test_timer() {
        let mut timer = Timer::new();
        std::thread::sleep(Duration::from_millis(10));
        assert!(timer.elapsed_ms() >= 10);
        
        timer.reset();
        assert!(timer.elapsed_ms() < 10);
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2, Duration::from_secs(1));
        
        assert!(limiter.allow());
        assert!(limiter.allow());
        assert!(!limiter.allow());
        assert_eq!(limiter.remaining(), 0);
    }

    #[test]
    fn test_cache() {
        let mut cache = Cache::new(Duration::from_millis(100));
        
        cache.insert("key1", "value1");
        assert_eq!(cache.get(&"key1"), Some("value1"));
        
        std::thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get(&"key1"), None);
    }

    #[test]
    fn test_string_utils() {
        assert_eq!(string::truncate("Hello World", 8), "Hello...");
        assert_eq!(string::to_title_case("hello world"), "Hello World");
        assert!(string::is_valid_email("test@example.com"));
        assert!(!string::is_valid_email("invalid-email"));
        assert!(string::is_valid_url("https://example.com"));
        assert!(!string::is_valid_url("not-a-url"));
    }
}
