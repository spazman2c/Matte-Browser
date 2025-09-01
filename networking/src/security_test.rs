#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::{
        ContentType, MixedContentType, MixedContentPolicy, MixedContentViolation,
        CorbPolicy, CorbViolation, CorsPolicy, CorsRequest, CorsResponse,
        CoopPolicy, CoopValue, CoepPolicy, CoepValue,
        SecurityContext, SecurityManager, GlobalSecurityPolicies, SecurityUtils
    };
    use std::collections::HashMap;
    use std::time::Duration;

    #[test]
    fn test_content_type_classification() {
        // Test blockable content types
        assert!(ContentType::Script.is_blockable());
        assert!(ContentType::Style.is_blockable());
        assert!(ContentType::Object.is_blockable());
        assert!(ContentType::Embed.is_blockable());
        assert!(ContentType::Plugin.is_blockable());
        
        // Test optionally blockable content types
        assert!(ContentType::Image.is_optionally_blockable());
        assert!(ContentType::Audio.is_optionally_blockable());
        assert!(ContentType::Video.is_optionally_blockable());
        assert!(ContentType::Font.is_optionally_blockable());
        
        // Test allowable content types
        assert!(ContentType::Frame.is_allowable());
        assert!(ContentType::Iframe.is_allowable());
        assert!(ContentType::XmlHttpRequest.is_allowable());
        assert!(ContentType::Fetch.is_allowable());
        assert!(ContentType::WebSocket.is_allowable());
        assert!(ContentType::Other.is_allowable());
    }

    #[test]
    fn test_content_type_mixed_content_type() {
        assert_eq!(ContentType::Script.mixed_content_type(), MixedContentType::Blockable);
        assert_eq!(ContentType::Style.mixed_content_type(), MixedContentType::Blockable);
        assert_eq!(ContentType::Image.mixed_content_type(), MixedContentType::OptionallyBlockable);
        assert_eq!(ContentType::Audio.mixed_content_type(), MixedContentType::OptionallyBlockable);
        assert_eq!(ContentType::Frame.mixed_content_type(), MixedContentType::Allowable);
        assert_eq!(ContentType::Other.mixed_content_type(), MixedContentType::Allowable);
    }

    #[test]
    fn test_content_type_mime_types() {
        assert_eq!(ContentType::Script.mime_type(), "application/javascript");
        assert_eq!(ContentType::Style.mime_type(), "text/css");
        assert_eq!(ContentType::Image.mime_type(), "image/*");
        assert_eq!(ContentType::Audio.mime_type(), "audio/*");
        assert_eq!(ContentType::Video.mime_type(), "video/*");
        assert_eq!(ContentType::Font.mime_type(), "font/*");
        assert_eq!(ContentType::Object.mime_type(), "application/*");
        assert_eq!(ContentType::Frame.mime_type(), "text/html");
        assert_eq!(ContentType::XmlHttpRequest.mime_type(), "application/json");
        assert_eq!(ContentType::Other.mime_type(), "*/*");
    }

    #[test]
    fn test_mixed_content_policy_default() {
        let policy = MixedContentPolicy::default();
        
        assert!(policy.block_mixed_content);
        assert!(policy.upgrade_insecure_requests);
        assert!(!policy.report_only);
        
        // Check blocked content types
        assert!(policy.blocked_content_types.contains(&ContentType::Script));
        assert!(policy.blocked_content_types.contains(&ContentType::Style));
        assert!(policy.blocked_content_types.contains(&ContentType::Object));
        assert!(policy.blocked_content_types.contains(&ContentType::Embed));
        assert!(policy.blocked_content_types.contains(&ContentType::Plugin));
        
        // Check upgraded content types
        assert!(policy.upgraded_content_types.contains(&ContentType::Image));
        assert!(policy.upgraded_content_types.contains(&ContentType::Audio));
        assert!(policy.upgraded_content_types.contains(&ContentType::Video));
        assert!(policy.upgraded_content_types.contains(&ContentType::Font));
    }

    #[test]
    fn test_mixed_content_policy_blocking() {
        let policy = MixedContentPolicy::default();
        
        // Test mixed content blocking (insecure source, secure target)
        assert!(policy.should_block(ContentType::Script, "http://example.com", "https://api.example.com"));
        assert!(policy.should_block(ContentType::Style, "http://example.com", "https://cdn.example.com"));
        assert!(!policy.should_block(ContentType::Image, "http://example.com", "https://cdn.example.com")); // Optionally blockable
        
        // Test no mixed content (both secure)
        assert!(!policy.should_block(ContentType::Script, "https://example.com", "https://api.example.com"));
        
        // Test no mixed content (both insecure)
        assert!(!policy.should_block(ContentType::Script, "http://example.com", "http://api.example.com"));
        
        // Test mixed content (secure source, insecure target) - should not block
        assert!(!policy.should_block(ContentType::Script, "https://example.com", "http://api.example.com"));
    }

    #[test]
    fn test_mixed_content_policy_upgrading() {
        let policy = MixedContentPolicy::default();
        
        // Test upgrading (secure source, insecure target)
        assert!(policy.should_upgrade(ContentType::Image, "https://example.com", "http://cdn.example.com"));
        assert!(policy.should_upgrade(ContentType::Audio, "https://example.com", "http://media.example.com"));
        assert!(!policy.should_upgrade(ContentType::Script, "https://example.com", "http://api.example.com")); // Not in upgraded list
        
        // Test no upgrading (both secure)
        assert!(!policy.should_upgrade(ContentType::Image, "https://example.com", "https://cdn.example.com"));
        
        // Test no upgrading (both insecure)
        assert!(!policy.should_upgrade(ContentType::Image, "http://example.com", "http://cdn.example.com"));
        
        // Test no upgrading (insecure source, secure target)
        assert!(!policy.should_upgrade(ContentType::Image, "http://example.com", "https://cdn.example.com"));
    }

    #[test]
    fn test_corb_policy_default() {
        let policy = CorbPolicy::default();
        
        assert!(policy.enabled);
        assert!(!policy.report_only);
        assert!(policy.blocked_mime_types.contains(&"text/html".to_string()));
        assert!(policy.blocked_mime_types.contains(&"text/xml".to_string()));
        assert!(policy.blocked_mime_types.contains(&"text/plain".to_string()));
        assert!(policy.blocked_mime_types.contains(&"application/json".to_string()));
        assert!(policy.blocked_mime_types.contains(&"application/xml".to_string()));
        
        assert!(policy.blocked_content_types.contains(&ContentType::Script));
        assert!(policy.blocked_content_types.contains(&ContentType::Style));
        assert!(policy.blocked_content_types.contains(&ContentType::Object));
        assert!(policy.blocked_content_types.contains(&ContentType::Embed));
        assert!(policy.blocked_content_types.contains(&ContentType::Plugin));
    }

    #[test]
    fn test_corb_policy_blocking() {
        let policy = CorbPolicy::default();
        
        // Test blocked MIME types
        assert!(policy.should_block("https://example.com", "https://api.example.com", "text/html", ContentType::Script));
        assert!(policy.should_block("https://example.com", "https://api.example.com", "application/json", ContentType::Script));
        
        // Test blocked content types
        assert!(policy.should_block("https://example.com", "https://api.example.com", "text/plain", ContentType::Script));
        assert!(policy.should_block("https://example.com", "https://api.example.com", "text/plain", ContentType::Style));
        
        // Test allowed content types
        assert!(!policy.should_block("https://example.com", "https://api.example.com", "text/plain", ContentType::Image));
        assert!(!policy.should_block("https://example.com", "https://api.example.com", "text/plain", ContentType::Audio));
    }

    #[test]
    fn test_cors_policy_default() {
        let policy = CorsPolicy::default();
        
        assert!(policy.allowed_origins.contains(&"*".to_string()));
        assert!(policy.allowed_methods.contains(&"GET".to_string()));
        assert!(policy.allowed_methods.contains(&"POST".to_string()));
        assert!(policy.allowed_methods.contains(&"PUT".to_string()));
        assert!(policy.allowed_methods.contains(&"DELETE".to_string()));
        assert!(policy.allowed_methods.contains(&"HEAD".to_string()));
        assert!(policy.allowed_methods.contains(&"OPTIONS".to_string()));
        
        assert!(policy.allowed_headers.contains(&"Content-Type".to_string()));
        assert!(policy.allowed_headers.contains(&"Authorization".to_string()));
        assert!(policy.allowed_headers.contains(&"X-Requested-With".to_string()));
        
        assert_eq!(policy.max_age, Some(Duration::from_secs(86400)));
        assert!(!policy.allow_credentials);
        assert!(policy.handle_preflight);
    }

    #[test]
    fn test_cors_policy_origin_checking() {
        let policy = CorsPolicy::default();
        
        // Test wildcard origin
        assert!(policy.is_origin_allowed("https://example.com"));
        assert!(policy.is_origin_allowed("https://api.example.com"));
        assert!(policy.is_origin_allowed("http://localhost:3000"));
        
        // Test specific origins
        let mut specific_policy = CorsPolicy::default();
        specific_policy.allowed_origins = vec!["https://example.com".to_string()];
        
        assert!(specific_policy.is_origin_allowed("https://example.com"));
        assert!(!specific_policy.is_origin_allowed("https://api.example.com"));
        assert!(!specific_policy.is_origin_allowed("http://localhost:3000"));
    }

    #[test]
    fn test_cors_policy_method_checking() {
        let policy = CorsPolicy::default();
        
        assert!(policy.is_method_allowed("GET"));
        assert!(policy.is_method_allowed("POST"));
        assert!(policy.is_method_allowed("PUT"));
        assert!(policy.is_method_allowed("DELETE"));
        assert!(policy.is_method_allowed("HEAD"));
        assert!(policy.is_method_allowed("OPTIONS"));
        assert!(!policy.is_method_allowed("PATCH"));
        assert!(!policy.is_method_allowed("TRACE"));
    }

    #[test]
    fn test_cors_policy_header_checking() {
        let policy = CorsPolicy::default();
        
        assert!(policy.is_header_allowed("Content-Type"));
        assert!(policy.is_header_allowed("Authorization"));
        assert!(policy.is_header_allowed("X-Requested-With"));
        assert!(!policy.is_header_allowed("X-Custom-Header"));
        assert!(!policy.is_header_allowed("X-API-Key"));
    }

    #[test]
    fn test_cors_request_processing() {
        let policy = CorsPolicy::default();
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), "Bearer token".to_string());
        
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "GET".to_string(),
            headers,
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let response = policy.process_request(&request);
        
        assert!(response.allowed);
        assert!(response.error.is_none());
        assert!(response.cors_headers.contains_key("Access-Control-Allow-Origin"));
        assert!(response.cors_headers.contains_key("Access-Control-Allow-Methods"));
        assert!(response.cors_headers.contains_key("Access-Control-Allow-Headers"));
        assert_eq!(response.cors_headers.get("Access-Control-Allow-Origin"), Some(&"https://example.com".to_string()));
    }

    #[test]
    fn test_cors_request_processing_disallowed_origin() {
        let mut policy = CorsPolicy::default();
        policy.allowed_origins = vec!["https://allowed.com".to_string()];
        
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let response = policy.process_request(&request);
        
        assert!(!response.allowed);
        assert!(response.error.is_some());
        assert_eq!(response.error, Some("Origin not allowed".to_string()));
    }

    #[test]
    fn test_cors_request_processing_disallowed_method() {
        let policy = CorsPolicy::default();
        
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "PATCH".to_string(),
            headers: HashMap::new(),
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let response = policy.process_request(&request);
        
        assert!(!response.allowed);
        assert!(response.error.is_some());
        assert_eq!(response.error, Some("Method not allowed".to_string()));
    }

    #[test]
    fn test_coop_policy_default() {
        let policy = CoopPolicy::default();
        
        assert_eq!(policy.value, CoopValue::UnsafeNone);
        assert!(!policy.report_only);
        assert!(policy.report_uri.is_none());
    }

    #[test]
    fn test_coop_policy_header_values() {
        let mut policy = CoopPolicy::default();
        
        assert_eq!(policy.header_value(), "unsafe-none");
        
        policy.value = CoopValue::SameOrigin;
        assert_eq!(policy.header_value(), "same-origin");
        
        policy.value = CoopValue::SameOriginAllowPopups;
        assert_eq!(policy.header_value(), "same-origin-allow-popups");
    }

    #[test]
    fn test_coop_policy_popup_allowing() {
        let mut policy = CoopPolicy::default();
        
        // UnsafeNone allows all popups
        assert!(policy.is_popup_allowed("https://example.com", "https://api.example.com"));
        assert!(policy.is_popup_allowed("https://example.com", "https://example.com"));
        
        // SameOrigin only allows same origin
        policy.value = CoopValue::SameOrigin;
        assert!(!policy.is_popup_allowed("https://example.com", "https://api.example.com"));
        assert!(policy.is_popup_allowed("https://example.com", "https://example.com"));
        
        // SameOriginAllowPopups allows all but isolates them
        policy.value = CoopValue::SameOriginAllowPopups;
        assert!(policy.is_popup_allowed("https://example.com", "https://api.example.com"));
        assert!(policy.is_popup_allowed("https://example.com", "https://example.com"));
    }

    #[test]
    fn test_coep_policy_default() {
        let policy = CoepPolicy::default();
        
        assert_eq!(policy.value, CoepValue::UnsafeNone);
        assert!(!policy.report_only);
        assert!(policy.report_uri.is_none());
    }

    #[test]
    fn test_coep_policy_header_values() {
        let mut policy = CoepPolicy::default();
        
        assert_eq!(policy.header_value(), "unsafe-none");
        
        policy.value = CoepValue::RequireCorp;
        assert_eq!(policy.header_value(), "require-corp");
    }

    #[test]
    fn test_coep_policy_resource_allowing() {
        let mut policy = CoepPolicy::default();
        
        // UnsafeNone allows all resources
        assert!(policy.is_resource_allowed("https://example.com", "https://api.example.com", false));
        assert!(policy.is_resource_allowed("https://example.com", "https://example.com", false));
        
        // RequireCorp requires same origin or CORP header
        policy.value = CoepValue::RequireCorp;
        assert!(!policy.is_resource_allowed("https://example.com", "https://api.example.com", false));
        assert!(policy.is_resource_allowed("https://example.com", "https://example.com", false));
        assert!(policy.is_resource_allowed("https://example.com", "https://api.example.com", true)); // Has CORP header
    }

    #[test]
    fn test_security_context_creation() {
        let context = SecurityContext::new("https://example.com".to_string(), "https://example.com/page".to_string());
        
        assert_eq!(context.origin, "https://example.com");
        assert_eq!(context.url, "https://example.com/page");
        assert!(context.is_secure);
        
        let insecure_context = SecurityContext::new("http://example.com".to_string(), "http://example.com/page".to_string());
        assert!(!insecure_context.is_secure);
    }

    #[test]
    fn test_security_context_mixed_content_checking() {
        let context = SecurityContext::new("https://example.com".to_string(), "https://example.com/page".to_string());
        
        // Test mixed content violation
        let violation = context.check_mixed_content(ContentType::Script, "http://api.example.com/script.js");
        assert!(violation.is_some());
        
        let violation = violation.unwrap();
        assert_eq!(violation.content_type, ContentType::Script);
        assert_eq!(violation.source_url, "https://example.com/page");
        assert_eq!(violation.target_url, "http://api.example.com/script.js");
        assert_eq!(violation.violation_type, MixedContentType::Blockable);
        
        // Test no mixed content
        let violation = context.check_mixed_content(ContentType::Script, "https://api.example.com/script.js");
        assert!(violation.is_none());
    }

    #[test]
    fn test_security_context_corb_checking() {
        let context = SecurityContext::new("https://example.com".to_string(), "https://example.com/page".to_string());
        
        // Test CORB violation
        let violation = context.check_corb("https://example.com", "https://api.example.com/data", "text/html", ContentType::Script);
        assert!(violation.is_some());
        
        let violation = violation.unwrap();
        assert_eq!(violation.content_type, ContentType::Script);
        assert_eq!(violation.url, "https://api.example.com/data");
        assert_eq!(violation.origin, "https://example.com");
        assert_eq!(violation.mime_type, "text/html");
        
        // Test no CORB violation
        let violation = context.check_corb("https://example.com", "https://api.example.com/data", "text/plain", ContentType::Image);
        assert!(violation.is_none());
    }

    #[test]
    fn test_security_context_cors_processing() {
        let context = SecurityContext::new("https://example.com".to_string(), "https://example.com/page".to_string());
        
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let response = context.process_cors_request(&request);
        
        assert!(response.allowed);
        assert!(response.error.is_none());
        assert!(response.cors_headers.contains_key("Access-Control-Allow-Origin"));
    }

    #[test]
    fn test_security_manager_creation() {
        let manager = SecurityManager::new();
        
        // Manager should be created successfully
        assert!(true); // Just verify no panic
    }

    #[test]
    fn test_security_manager_context_management() {
        let manager = SecurityManager::new();
        
        // Create context
        let context_id = manager.create_context("https://example.com".to_string(), "https://example.com/page".to_string());
        assert!(!context_id.is_empty());
        
        // Get context
        let context = manager.get_context(&context_id);
        assert!(context.is_some());
        
        let context = context.unwrap();
        assert_eq!(context.origin, "https://example.com");
        assert_eq!(context.url, "https://example.com/page");
        assert!(context.is_secure);
        
        // Get non-existent context
        let context = manager.get_context("non-existent");
        assert!(context.is_none());
    }

    #[test]
    fn test_security_manager_mixed_content_checking() {
        let manager = SecurityManager::new();
        
        let context_id = manager.create_context("https://example.com".to_string(), "https://example.com/page".to_string());
        
        // Test mixed content blocking
        let blocked = manager.check_mixed_content(&context_id, ContentType::Script, "http://api.example.com/script.js").unwrap();
        assert!(blocked);
        
        // Test no mixed content
        let blocked = manager.check_mixed_content(&context_id, ContentType::Script, "https://api.example.com/script.js").unwrap();
        assert!(!blocked);
        
        // Test with non-existent context
        let result = manager.check_mixed_content("non-existent", ContentType::Script, "http://api.example.com/script.js");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_security_manager_corb_checking() {
        let manager = SecurityManager::new();
        
        let context_id = manager.create_context("https://example.com".to_string(), "https://example.com/page".to_string());
        
        // Test CORB blocking
        let blocked = manager.check_corb(&context_id, "https://example.com", "https://api.example.com/data", "text/html", ContentType::Script).unwrap();
        assert!(blocked);
        
        // Test no CORB blocking
        let blocked = manager.check_corb(&context_id, "https://example.com", "https://api.example.com/data", "text/plain", ContentType::Image).unwrap();
        assert!(!blocked);
        
        // Test with non-existent context
        let result = manager.check_corb("non-existent", "https://example.com", "https://api.example.com/data", "text/html", ContentType::Script);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_security_manager_cors_processing() {
        let manager = SecurityManager::new();
        
        let context_id = manager.create_context("https://example.com".to_string(), "https://example.com/page".to_string());
        
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let response = manager.process_cors_request(&context_id, &request).unwrap();
        
        assert!(response.allowed);
        assert!(response.error.is_none());
        assert!(response.cors_headers.contains_key("Access-Control-Allow-Origin"));
        
        // Test with non-existent context
        let result = manager.process_cors_request("non-existent", &request);
        assert!(result.is_err());
    }

    #[test]
    fn test_security_manager_violation_tracking() {
        let manager = SecurityManager::new();
        
        let context_id = manager.create_context("https://example.com".to_string(), "https://example.com/page".to_string());
        
        // Generate some violations
        manager.check_mixed_content(&context_id, ContentType::Script, "http://api.example.com/script.js").unwrap();
        manager.check_corb(&context_id, "https://example.com", "https://api.example.com/data", "text/html", ContentType::Script).unwrap();
        
        // Check violations
        let mixed_content_violations = manager.get_mixed_content_violations();
        assert_eq!(mixed_content_violations.len(), 1);
        
        let corb_violations = manager.get_corb_violations();
        assert_eq!(corb_violations.len(), 1);
        
        // Clear violations
        manager.clear_violations();
        
        let mixed_content_violations = manager.get_mixed_content_violations();
        assert_eq!(mixed_content_violations.len(), 0);
        
        let corb_violations = manager.get_corb_violations();
        assert_eq!(corb_violations.len(), 0);
    }

    #[test]
    fn test_security_utils_url_checking() {
        // Test secure URLs
        assert!(SecurityUtils::is_secure_url("https://example.com"));
        assert!(SecurityUtils::is_secure_url("wss://example.com"));
        assert!(!SecurityUtils::is_secure_url("http://example.com"));
        assert!(!SecurityUtils::is_secure_url("ws://example.com"));
        
        // Test local URLs
        assert!(SecurityUtils::is_local_url("file:///path/to/file"));
        assert!(SecurityUtils::is_local_url("data:text/plain,Hello"));
        assert!(SecurityUtils::is_local_url("blob:https://example.com/uuid"));
        assert!(!SecurityUtils::is_local_url("https://example.com"));
    }

    #[test]
    fn test_security_utils_origin_extraction() {
        // Test origin extraction
        let origin = SecurityUtils::extract_origin("https://example.com/page").unwrap();
        assert_eq!(origin, "https://example.com");
        
        let origin = SecurityUtils::extract_origin("https://example.com:8080/page").unwrap();
        assert_eq!(origin, "https://example.com:8080");
        
        let origin = SecurityUtils::extract_origin("http://localhost:3000/api").unwrap();
        assert_eq!(origin, "http://localhost:3000");
        
        // Test invalid URL
        let result = SecurityUtils::extract_origin("invalid-url");
        assert!(result.is_err());
    }

    #[test]
    fn test_security_utils_origin_comparison() {
        // Test same origin
        assert!(SecurityUtils::is_same_origin("https://example.com", "https://example.com"));
        assert!(SecurityUtils::is_same_origin("https://example.com:443", "https://example.com"));
        assert!(!SecurityUtils::is_same_origin("https://example.com", "https://api.example.com"));
        assert!(!SecurityUtils::is_same_origin("http://example.com", "https://example.com"));
        
        // Test cross origin
        assert!(SecurityUtils::is_cross_origin("https://example.com", "https://api.example.com"));
        assert!(SecurityUtils::is_cross_origin("http://example.com", "https://example.com"));
        assert!(!SecurityUtils::is_cross_origin("https://example.com", "https://example.com"));
    }

    #[test]
    fn test_security_utils_preflight_validation() {
        // Test valid preflight request
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://example.com".to_string());
        headers.insert("Access-Control-Request-Method".to_string(), "POST".to_string());
        
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "OPTIONS".to_string(),
            headers,
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let is_preflight = SecurityUtils::validate_preflight_request(&request).unwrap();
        assert!(is_preflight);
        
        // Test invalid preflight request (wrong method)
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "GET".to_string(),
            headers,
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let is_preflight = SecurityUtils::validate_preflight_request(&request).unwrap();
        assert!(!is_preflight);
        
        // Test invalid preflight request (missing headers)
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "OPTIONS".to_string(),
            headers: HashMap::new(),
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let is_preflight = SecurityUtils::validate_preflight_request(&request).unwrap();
        assert!(!is_preflight);
    }

    #[test]
    fn test_security_utils_preflight_response_generation() {
        let policy = CorsPolicy::default();
        
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "OPTIONS".to_string(),
            headers: HashMap::new(),
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let response = SecurityUtils::generate_preflight_response(&policy, &request);
        
        assert!(response.allowed);
        assert!(response.error.is_none());
        assert!(response.cors_headers.contains_key("Access-Control-Allow-Origin"));
        assert!(response.cors_headers.contains_key("Access-Control-Allow-Methods"));
        assert!(response.cors_headers.contains_key("Access-Control-Allow-Headers"));
        assert!(response.cors_headers.contains_key("Access-Control-Max-Age"));
        assert_eq!(response.cors_headers.get("Access-Control-Allow-Origin"), Some(&"https://example.com".to_string()));
    }

    #[test]
    fn test_global_security_policies() {
        let policies = GlobalSecurityPolicies {
            default_mixed_content_policy: MixedContentPolicy::default(),
            default_corb_policy: CorbPolicy::default(),
            default_cors_policy: CorsPolicy::default(),
            default_coop_policy: CoopPolicy::default(),
            default_coep_policy: CoepPolicy::default(),
        };
        
        // Verify all policies are created
        assert!(policies.default_mixed_content_policy.block_mixed_content);
        assert!(policies.default_corb_policy.enabled);
        assert!(policies.default_cors_policy.allowed_origins.contains(&"*".to_string()));
        assert_eq!(policies.default_coop_policy.value, CoopValue::UnsafeNone);
        assert_eq!(policies.default_coep_policy.value, CoepValue::UnsafeNone);
    }

    #[test]
    fn test_comprehensive_security_workflow() {
        let manager = SecurityManager::new();
        
        // Create security context
        let context_id = manager.create_context("https://example.com".to_string(), "https://example.com/page".to_string());
        
        // Test mixed content detection
        let blocked = manager.check_mixed_content(&context_id, ContentType::Script, "http://api.example.com/script.js").unwrap();
        assert!(blocked);
        
        // Test CORB detection
        let blocked = manager.check_corb(&context_id, "https://example.com", "https://api.example.com/data", "text/html", ContentType::Script).unwrap();
        assert!(blocked);
        
        // Test CORS processing
        let request = CorsRequest {
            origin: "https://example.com".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            url: "https://api.example.com/data".to_string(),
            credentials: false,
        };
        
        let response = manager.process_cors_request(&context_id, &request).unwrap();
        assert!(response.allowed);
        
        // Test violation tracking
        let mixed_content_violations = manager.get_mixed_content_violations();
        assert_eq!(mixed_content_violations.len(), 1);
        
        let corb_violations = manager.get_corb_violations();
        assert_eq!(corb_violations.len(), 1);
        
        // Test policy updates
        let policies = manager.get_global_policies();
        assert!(policies.default_mixed_content_policy.block_mixed_content);
    }

    #[test]
    fn test_security_policy_customization() {
        // Test custom mixed content policy
        let mut mixed_content_policy = MixedContentPolicy::default();
        mixed_content_policy.block_mixed_content = false;
        mixed_content_policy.report_only = true;
        
        assert!(!mixed_content_policy.block_mixed_content);
        assert!(mixed_content_policy.report_only);
        
        // Test custom CORB policy
        let mut corb_policy = CorbPolicy::default();
        corb_policy.enabled = false;
        corb_policy.report_only = true;
        
        assert!(!corb_policy.enabled);
        assert!(corb_policy.report_only);
        
        // Test custom CORS policy
        let mut cors_policy = CorsPolicy::default();
        cors_policy.allowed_origins = vec!["https://example.com".to_string()];
        cors_policy.allow_credentials = true;
        
        assert_eq!(cors_policy.allowed_origins.len(), 1);
        assert!(cors_policy.allow_credentials);
        
        // Test custom COOP policy
        let mut coop_policy = CoopPolicy::default();
        coop_policy.value = CoopValue::SameOrigin;
        coop_policy.report_uri = Some("https://reports.example.com".to_string());
        
        assert_eq!(coop_policy.value, CoopValue::SameOrigin);
        assert_eq!(coop_policy.report_uri, Some("https://reports.example.com".to_string()));
        
        // Test custom COEP policy
        let mut coep_policy = CoepPolicy::default();
        coep_policy.value = CoepValue::RequireCorp;
        coep_policy.report_uri = Some("https://reports.example.com".to_string());
        
        assert_eq!(coep_policy.value, CoepValue::RequireCorp);
        assert_eq!(coep_policy.report_uri, Some("https://reports.example.com".to_string()));
    }
}
