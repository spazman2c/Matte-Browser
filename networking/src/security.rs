use crate::error::{Error, Result};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::sync::Arc;
use parking_lot::RwLock;
use url::Url;

/// Content type for mixed content detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentType {
    Script,
    Style,
    Image,
    Audio,
    Video,
    Font,
    Object,
    Embed,
    Plugin,
    Frame,
    Iframe,
    XmlHttpRequest,
    Fetch,
    WebSocket,
    Other,
}

/// Mixed content type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixedContentType {
    Blockable,
    OptionallyBlockable,
    Allowable,
}

/// Mixed content policy
#[derive(Debug, Clone)]
pub struct MixedContentPolicy {
    /// Whether to block mixed content
    pub block_mixed_content: bool,
    /// Whether to upgrade insecure requests
    pub upgrade_insecure_requests: bool,
    /// Content types to block
    pub blocked_content_types: Vec<ContentType>,
    /// Content types to upgrade
    pub upgraded_content_types: Vec<ContentType>,
    /// Report-only mode
    pub report_only: bool,
}

/// Mixed content violation
#[derive(Debug, Clone)]
pub struct MixedContentViolation {
    /// Violation type
    pub violation_type: MixedContentType,
    /// Content type
    pub content_type: ContentType,
    /// Source URL
    pub source_url: String,
    /// Target URL
    pub target_url: String,
    /// Violation timestamp
    pub timestamp: SystemTime,
    /// Violation details
    pub details: String,
}

/// CORB (Cross-Origin Read Blocking) policy
#[derive(Debug, Clone)]
pub struct CorbPolicy {
    /// Whether CORB is enabled
    pub enabled: bool,
    /// Blocked MIME types
    pub blocked_mime_types: Vec<String>,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Report-only mode
    pub report_only: bool,
    /// Blocked content types
    pub blocked_content_types: Vec<ContentType>,
}

/// CORB violation
#[derive(Debug, Clone)]
pub struct CorbViolation {
    /// Requested URL
    pub url: String,
    /// Origin
    pub origin: String,
    /// MIME type
    pub mime_type: String,
    /// Content type
    pub content_type: ContentType,
    /// Violation timestamp
    pub timestamp: SystemTime,
    /// Violation details
    pub details: String,
}

/// CORS (Cross-Origin Resource Sharing) policy
#[derive(Debug, Clone)]
pub struct CorsPolicy {
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Exposed headers
    pub exposed_headers: Vec<String>,
    /// Max age
    pub max_age: Option<Duration>,
    /// Allow credentials
    pub allow_credentials: bool,
    /// Preflight handling
    pub handle_preflight: bool,
}

/// CORS request
#[derive(Debug, Clone)]
pub struct CorsRequest {
    /// Request origin
    pub origin: String,
    /// Request method
    pub method: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request URL
    pub url: String,
    /// Whether request includes credentials
    pub credentials: bool,
}

/// CORS response
#[derive(Debug, Clone)]
pub struct CorsResponse {
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Whether request is allowed
    pub allowed: bool,
    /// CORS headers to add
    pub cors_headers: HashMap<String, String>,
    /// Error message if not allowed
    pub error: Option<String>,
}

/// COOP (Cross-Origin Opener Policy) policy
#[derive(Debug, Clone)]
pub struct CoopPolicy {
    /// COOP value
    pub value: CoopValue,
    /// Report-only mode
    pub report_only: bool,
    /// Report URI
    pub report_uri: Option<String>,
}

/// COOP value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoopValue {
    UnsafeNone,
    SameOrigin,
    SameOriginAllowPopups,
}

/// COEP (Cross-Origin Embedder Policy) policy
#[derive(Debug, Clone)]
pub struct CoepPolicy {
    /// COEP value
    pub value: CoepValue,
    /// Report-only mode
    pub report_only: bool,
    /// Report URI
    pub report_uri: Option<String>,
}

/// COEP value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoepValue {
    UnsafeNone,
    RequireCorp,
}

/// Security context
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Document origin
    pub origin: String,
    /// Document URL
    pub url: String,
    /// Whether document is secure
    pub is_secure: bool,
    /// Mixed content policy
    pub mixed_content_policy: MixedContentPolicy,
    /// CORB policy
    pub corb_policy: CorbPolicy,
    /// CORS policy
    pub cors_policy: CorsPolicy,
    /// COOP policy
    pub coop_policy: CoopPolicy,
    /// COEP policy
    pub coep_policy: CoepPolicy,
}

/// Security manager
pub struct SecurityManager {
    /// Security contexts
    contexts: Arc<RwLock<HashMap<String, SecurityContext>>>,
    /// Mixed content violations
    mixed_content_violations: Arc<RwLock<Vec<MixedContentViolation>>>,
    /// CORB violations
    corb_violations: Arc<RwLock<Vec<CorbViolation>>>,
    /// Global security policies
    global_policies: Arc<RwLock<GlobalSecurityPolicies>>,
}

/// Global security policies
#[derive(Debug, Clone)]
pub struct GlobalSecurityPolicies {
    /// Default mixed content policy
    pub default_mixed_content_policy: MixedContentPolicy,
    /// Default CORB policy
    pub default_corb_policy: CorbPolicy,
    /// Default CORS policy
    pub default_cors_policy: CorsPolicy,
    /// Default COOP policy
    pub default_coop_policy: CoopPolicy,
    /// Default COEP policy
    pub default_coep_policy: CoepPolicy,
}

impl ContentType {
    /// Check if content type is blockable
    pub fn is_blockable(&self) -> bool {
        matches!(self, ContentType::Script | ContentType::Style | ContentType::Object | ContentType::Embed | ContentType::Plugin)
    }

    /// Check if content type is optionally blockable
    pub fn is_optionally_blockable(&self) -> bool {
        matches!(self, ContentType::Image | ContentType::Audio | ContentType::Video | ContentType::Font)
    }

    /// Check if content type is allowable
    pub fn is_allowable(&self) -> bool {
        matches!(self, ContentType::Frame | ContentType::Iframe | ContentType::XmlHttpRequest | ContentType::Fetch | ContentType::WebSocket | ContentType::Other)
    }

    /// Get mixed content type
    pub fn mixed_content_type(&self) -> MixedContentType {
        if self.is_blockable() {
            MixedContentType::Blockable
        } else if self.is_optionally_blockable() {
            MixedContentType::OptionallyBlockable
        } else {
            MixedContentType::Allowable
        }
    }

    /// Get MIME type for content type
    pub fn mime_type(&self) -> &'static str {
        match self {
            ContentType::Script => "application/javascript",
            ContentType::Style => "text/css",
            ContentType::Image => "image/*",
            ContentType::Audio => "audio/*",
            ContentType::Video => "video/*",
            ContentType::Font => "font/*",
            ContentType::Object => "application/*",
            ContentType::Embed => "application/*",
            ContentType::Plugin => "application/*",
            ContentType::Frame => "text/html",
            ContentType::Iframe => "text/html",
            ContentType::XmlHttpRequest => "application/json",
            ContentType::Fetch => "application/json",
            ContentType::WebSocket => "application/websocket",
            ContentType::Other => "*/*",
        }
    }
}

impl MixedContentPolicy {
    /// Create default mixed content policy
    pub fn default() -> Self {
        Self {
            block_mixed_content: true,
            upgrade_insecure_requests: true,
            blocked_content_types: vec![
                ContentType::Script,
                ContentType::Style,
                ContentType::Object,
                ContentType::Embed,
                ContentType::Plugin,
            ],
            upgraded_content_types: vec![
                ContentType::Image,
                ContentType::Audio,
                ContentType::Video,
                ContentType::Font,
            ],
            report_only: false,
        }
    }

    /// Check if content should be blocked
    pub fn should_block(&self, content_type: ContentType, source_url: &str, target_url: &str) -> bool {
        if !self.block_mixed_content {
            return false;
        }

        let source_secure = source_url.starts_with("https://");
        let target_secure = target_url.starts_with("https://");

        // If both are secure or both are insecure, no mixed content
        if source_secure == target_secure {
            return false;
        }

        // If source is insecure and target is secure, this is mixed content
        if !source_secure && target_secure {
            return self.blocked_content_types.contains(&content_type);
        }

        false
    }

    /// Check if content should be upgraded
    pub fn should_upgrade(&self, content_type: ContentType, source_url: &str, target_url: &str) -> bool {
        if !self.upgrade_insecure_requests {
            return false;
        }

        let source_secure = source_url.starts_with("https://");
        let target_secure = target_url.starts_with("https://");

        // If source is secure and target is insecure, try to upgrade
        if source_secure && !target_secure {
            return self.upgraded_content_types.contains(&content_type);
        }

        false
    }
}

impl CorbPolicy {
    /// Create default CORB policy
    pub fn default() -> Self {
        Self {
            enabled: true,
            blocked_mime_types: vec![
                "text/html".to_string(),
                "text/xml".to_string(),
                "text/plain".to_string(),
                "application/json".to_string(),
                "application/xml".to_string(),
            ],
            allowed_origins: Vec::new(),
            report_only: false,
            blocked_content_types: vec![
                ContentType::Script,
                ContentType::Style,
                ContentType::Object,
                ContentType::Embed,
                ContentType::Plugin,
            ],
        }
    }

    /// Check if request should be blocked
    pub fn should_block(&self, origin: &str, url: &str, mime_type: &str, content_type: ContentType) -> bool {
        if !self.enabled {
            return false;
        }

        // Check if origin is allowed
        if !self.allowed_origins.is_empty() && !self.allowed_origins.contains(&origin.to_string()) {
            return true;
        }

        // Check if MIME type is blocked
        if self.blocked_mime_types.contains(&mime_type.to_string()) {
            return true;
        }

        // Check if content type is blocked
        if self.blocked_content_types.contains(&content_type) {
            return true;
        }

        false
    }
}

impl CorsPolicy {
    /// Create default CORS policy
    pub fn default() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "HEAD".to_string(),
                "OPTIONS".to_string(),
            ],
            allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "X-Requested-With".to_string(),
            ],
            exposed_headers: Vec::new(),
            max_age: Some(Duration::from_secs(86400)), // 24 hours
            allow_credentials: false,
            handle_preflight: true,
        }
    }

    /// Check if origin is allowed
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        self.allowed_origins.contains(&"*".to_string()) || self.allowed_origins.contains(&origin.to_string())
    }

    /// Check if method is allowed
    pub fn is_method_allowed(&self, method: &str) -> bool {
        self.allowed_methods.contains(&method.to_string())
    }

    /// Check if header is allowed
    pub fn is_header_allowed(&self, header: &str) -> bool {
        self.allowed_headers.contains(&header.to_string())
    }

    /// Process CORS request
    pub fn process_request(&self, request: &CorsRequest) -> CorsResponse {
        let mut response = CorsResponse {
            headers: HashMap::new(),
            allowed: true,
            cors_headers: HashMap::new(),
            error: None,
        };

        // Check origin
        if !self.is_origin_allowed(&request.origin) {
            response.allowed = false;
            response.error = Some("Origin not allowed".to_string());
            return response;
        }

        // Check method
        if !self.is_method_allowed(&request.method) {
            response.allowed = false;
            response.error = Some("Method not allowed".to_string());
            return response;
        }

        // Check headers
        for header in request.headers.keys() {
            if !self.is_header_allowed(header) {
                response.allowed = false;
                response.error = Some(format!("Header not allowed: {}", header));
                return response;
            }
        }

        // Add CORS headers
        response.cors_headers.insert("Access-Control-Allow-Origin".to_string(), request.origin.clone());
        response.cors_headers.insert("Access-Control-Allow-Methods".to_string(), self.allowed_methods.join(", "));
        response.cors_headers.insert("Access-Control-Allow-Headers".to_string(), self.allowed_headers.join(", "));

        if let Some(max_age) = self.max_age {
            response.cors_headers.insert("Access-Control-Max-Age".to_string(), max_age.as_secs().to_string());
        }

        if self.allow_credentials {
            response.cors_headers.insert("Access-Control-Allow-Credentials".to_string(), "true".to_string());
        }

        if !self.exposed_headers.is_empty() {
            response.cors_headers.insert("Access-Control-Expose-Headers".to_string(), self.exposed_headers.join(", "));
        }

        response
    }
}

impl CoopPolicy {
    /// Create default COOP policy
    pub fn default() -> Self {
        Self {
            value: CoopValue::UnsafeNone,
            report_only: false,
            report_uri: None,
        }
    }

    /// Get COOP header value
    pub fn header_value(&self) -> String {
        match self.value {
            CoopValue::UnsafeNone => "unsafe-none".to_string(),
            CoopValue::SameOrigin => "same-origin".to_string(),
            CoopValue::SameOriginAllowPopups => "same-origin-allow-popups".to_string(),
        }
    }

    /// Check if popup is allowed
    pub fn is_popup_allowed(&self, opener_origin: &str, popup_origin: &str) -> bool {
        match self.value {
            CoopValue::UnsafeNone => true,
            CoopValue::SameOrigin => opener_origin == popup_origin,
            CoopValue::SameOriginAllowPopups => true, // Popups are allowed but isolated
        }
    }
}

impl CoepPolicy {
    /// Create default COEP policy
    pub fn default() -> Self {
        Self {
            value: CoepValue::UnsafeNone,
            report_only: false,
            report_uri: None,
        }
    }

    /// Get COEP header value
    pub fn header_value(&self) -> String {
        match self.value {
            CoepValue::UnsafeNone => "unsafe-none".to_string(),
            CoepValue::RequireCorp => "require-corp".to_string(),
        }
    }

    /// Check if resource is allowed
    pub fn is_resource_allowed(&self, resource_origin: &str, document_origin: &str, has_corp_header: bool) -> bool {
        match self.value {
            CoepValue::UnsafeNone => true,
            CoepValue::RequireCorp => {
                resource_origin == document_origin || has_corp_header
            }
        }
    }
}

impl SecurityContext {
    /// Create new security context
    pub fn new(origin: String, url: String) -> Self {
        let is_secure = url.starts_with("https://");
        
        Self {
            origin,
            url,
            is_secure,
            mixed_content_policy: MixedContentPolicy::default(),
            corb_policy: CorbPolicy::default(),
            cors_policy: CorsPolicy::default(),
            coop_policy: CoopPolicy::default(),
            coep_policy: CoepPolicy::default(),
        }
    }

    /// Check for mixed content
    pub fn check_mixed_content(&self, content_type: ContentType, target_url: &str) -> Option<MixedContentViolation> {
        if self.mixed_content_policy.should_block(content_type, &self.url, target_url) {
            Some(MixedContentViolation {
                violation_type: content_type.mixed_content_type(),
                content_type,
                source_url: self.url.clone(),
                target_url: target_url.to_string(),
                timestamp: SystemTime::now(),
                details: format!("Mixed content blocked: {} from {} to {}", 
                    content_type.mime_type(), self.url, target_url),
            })
        } else {
            None
        }
    }

    /// Check for CORB violation
    pub fn check_corb(&self, origin: &str, url: &str, mime_type: &str, content_type: ContentType) -> Option<CorbViolation> {
        if self.corb_policy.should_block(origin, url, mime_type, content_type) {
            Some(CorbViolation {
                url: url.to_string(),
                origin: origin.to_string(),
                mime_type: mime_type.to_string(),
                content_type,
                timestamp: SystemTime::now(),
                details: format!("CORB violation: {} from {} with MIME type {}", 
                    content_type.mime_type(), origin, mime_type),
            })
        } else {
            None
        }
    }

    /// Process CORS request
    pub fn process_cors_request(&self, request: &CorsRequest) -> CorsResponse {
        self.cors_policy.process_request(request)
    }
}

impl SecurityManager {
    /// Create new security manager
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(RwLock::new(HashMap::new())),
            mixed_content_violations: Arc::new(RwLock::new(Vec::new())),
            corb_violations: Arc::new(RwLock::new(Vec::new())),
            global_policies: Arc::new(RwLock::new(GlobalSecurityPolicies {
                default_mixed_content_policy: MixedContentPolicy::default(),
                default_corb_policy: CorbPolicy::default(),
                default_cors_policy: CorsPolicy::default(),
                default_coop_policy: CoopPolicy::default(),
                default_coep_policy: CoepPolicy::default(),
            })),
        }
    }

    /// Create security context
    pub fn create_context(&self, origin: String, url: String) -> String {
        let context_id = format!("{}:{}", origin, url);
        let context = SecurityContext::new(origin, url);
        
        self.contexts.write().insert(context_id.clone(), context);
        context_id
    }

    /// Get security context
    pub fn get_context(&self, context_id: &str) -> Option<SecurityContext> {
        self.contexts.read().get(context_id).cloned()
    }

    /// Check mixed content
    pub fn check_mixed_content(&self, context_id: &str, content_type: ContentType, target_url: &str) -> Result<bool> {
        if let Some(context) = self.get_context(context_id) {
            if let Some(violation) = context.check_mixed_content(content_type, target_url) {
                if !context.mixed_content_policy.report_only {
                    self.mixed_content_violations.write().push(violation);
                    return Ok(true); // Blocked
                }
            }
        }
        Ok(false) // Not blocked
    }

    /// Check CORB
    pub fn check_corb(&self, context_id: &str, origin: &str, url: &str, mime_type: &str, content_type: ContentType) -> Result<bool> {
        if let Some(context) = self.get_context(context_id) {
            if let Some(violation) = context.check_corb(origin, url, mime_type, content_type) {
                if !context.corb_policy.report_only {
                    self.corb_violations.write().push(violation);
                    return Ok(true); // Blocked
                }
            }
        }
        Ok(false) // Not blocked
    }

    /// Process CORS request
    pub fn process_cors_request(&self, context_id: &str, request: &CorsRequest) -> Result<CorsResponse> {
        if let Some(context) = self.get_context(context_id) {
            Ok(context.process_cors_request(request))
        } else {
            Err(Error::security("Security context not found".to_string()))
        }
    }

    /// Get mixed content violations
    pub fn get_mixed_content_violations(&self) -> Vec<MixedContentViolation> {
        self.mixed_content_violations.read().clone()
    }

    /// Get CORB violations
    pub fn get_corb_violations(&self) -> Vec<CorbViolation> {
        self.corb_violations.read().clone()
    }

    /// Clear violations
    pub fn clear_violations(&self) {
        self.mixed_content_violations.write().clear();
        self.corb_violations.write().clear();
    }

    /// Update global policies
    pub fn update_global_policies(&self, policies: GlobalSecurityPolicies) {
        *self.global_policies.write() = policies;
    }

    /// Get global policies
    pub fn get_global_policies(&self) -> GlobalSecurityPolicies {
        self.global_policies.read().clone()
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Security utilities
pub struct SecurityUtils;

impl SecurityUtils {
    /// Check if URL is secure
    pub fn is_secure_url(url: &str) -> bool {
        url.starts_with("https://") || url.starts_with("wss://")
    }

    /// Check if URL is local
    pub fn is_local_url(url: &str) -> bool {
        url.starts_with("file://") || url.starts_with("data:") || url.starts_with("blob:")
    }

    /// Extract origin from URL
    pub fn extract_origin(url: &str) -> Result<String> {
        let parsed_url = Url::parse(url)
            .map_err(|e| Error::security(format!("Invalid URL: {}", e)))?;
        
        let scheme = parsed_url.scheme();
        let host = parsed_url.host_str()
            .ok_or_else(|| Error::security("No host in URL".to_string()))?;
        let port = parsed_url.port();
        
        let origin = if let Some(port) = port {
            format!("{}://{}:{}", scheme, host, port)
        } else {
            format!("{}://{}", scheme, host)
        };
        
        Ok(origin)
    }

    /// Check if origins are same
    pub fn is_same_origin(origin1: &str, origin2: &str) -> bool {
        origin1 == origin2
    }

    /// Check if origins are cross-origin
    pub fn is_cross_origin(origin1: &str, origin2: &str) -> bool {
        !Self::is_same_origin(origin1, origin2)
    }

    /// Validate CORS preflight request
    pub fn validate_preflight_request(request: &CorsRequest) -> Result<bool> {
        // Check if it's a preflight request
        if request.method != "OPTIONS" {
            return Ok(false);
        }

        // Check for required headers
        let has_origin = request.headers.contains_key("Origin");
        let has_access_control_request_method = request.headers.contains_key("Access-Control-Request-Method");

        Ok(has_origin && has_access_control_request_method)
    }

    /// Generate CORS preflight response
    pub fn generate_preflight_response(policy: &CorsPolicy, request: &CorsRequest) -> CorsResponse {
        let mut response = CorsResponse {
            headers: HashMap::new(),
            allowed: true,
            cors_headers: HashMap::new(),
            error: None,
        };

        // Add CORS headers for preflight
        response.cors_headers.insert("Access-Control-Allow-Origin".to_string(), request.origin.clone());
        response.cors_headers.insert("Access-Control-Allow-Methods".to_string(), policy.allowed_methods.join(", "));
        response.cors_headers.insert("Access-Control-Allow-Headers".to_string(), policy.allowed_headers.join(", "));

        if let Some(max_age) = policy.max_age {
            response.cors_headers.insert("Access-Control-Max-Age".to_string(), max_age.as_secs().to_string());
        }

        if policy.allow_credentials {
            response.cors_headers.insert("Access-Control-Allow-Credentials".to_string(), "true".to_string());
        }

        response
    }
}
