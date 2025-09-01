#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{
        HttpMethod, HttpVersion, HttpStatus, HttpHeaders, HttpRequest, HttpResponse,
        HttpConnection, HttpConnectionPool, HttpClient, Http2Settings, Http2Frame,
        Http2FrameType, HpackCodec
    };
    use std::time::Duration;

    #[test]
    fn test_http_method_conversion() {
        assert_eq!(HttpMethod::GET.as_str(), "GET");
        assert_eq!(HttpMethod::POST.as_str(), "POST");
        assert_eq!(HttpMethod::PUT.as_str(), "PUT");
        assert_eq!(HttpMethod::DELETE.as_str(), "DELETE");
        assert_eq!(HttpMethod::HEAD.as_str(), "HEAD");
        assert_eq!(HttpMethod::OPTIONS.as_str(), "OPTIONS");
        assert_eq!(HttpMethod::PATCH.as_str(), "PATCH");
        assert_eq!(HttpMethod::TRACE.as_str(), "TRACE");
        assert_eq!(HttpMethod::CONNECT.as_str(), "CONNECT");
    }

    #[test]
    fn test_http_method_parsing() {
        assert_eq!(HttpMethod::from_str("GET"), Some(HttpMethod::GET));
        assert_eq!(HttpMethod::from_str("POST"), Some(HttpMethod::POST));
        assert_eq!(HttpMethod::from_str("PUT"), Some(HttpMethod::PUT));
        assert_eq!(HttpMethod::from_str("DELETE"), Some(HttpMethod::DELETE));
        assert_eq!(HttpMethod::from_str("HEAD"), Some(HttpMethod::HEAD));
        assert_eq!(HttpMethod::from_str("OPTIONS"), Some(HttpMethod::OPTIONS));
        assert_eq!(HttpMethod::from_str("PATCH"), Some(HttpMethod::PATCH));
        assert_eq!(HttpMethod::from_str("TRACE"), Some(HttpMethod::TRACE));
        assert_eq!(HttpMethod::from_str("CONNECT"), Some(HttpMethod::CONNECT));
        assert_eq!(HttpMethod::from_str("INVALID"), None);
    }

    #[test]
    fn test_http_version_conversion() {
        assert_eq!(HttpVersion::Http1_0.as_str(), "HTTP/1.0");
        assert_eq!(HttpVersion::Http1_1.as_str(), "HTTP/1.1");
        assert_eq!(HttpVersion::Http2_0.as_str(), "HTTP/2.0");
    }

    #[test]
    fn test_http_version_parsing() {
        assert_eq!(HttpVersion::from_str("HTTP/1.0"), Some(HttpVersion::Http1_0));
        assert_eq!(HttpVersion::from_str("HTTP/1.1"), Some(HttpVersion::Http1_1));
        assert_eq!(HttpVersion::from_str("HTTP/2.0"), Some(HttpVersion::Http2_0));
        assert_eq!(HttpVersion::from_str("INVALID"), None);
    }

    #[test]
    fn test_http_status_codes() {
        assert_eq!(HttpStatus::Ok as u16, 200);
        assert_eq!(HttpStatus::Created as u16, 201);
        assert_eq!(HttpStatus::Accepted as u16, 202);
        assert_eq!(HttpStatus::NoContent as u16, 204);
        assert_eq!(HttpStatus::MovedPermanently as u16, 301);
        assert_eq!(HttpStatus::Found as u16, 302);
        assert_eq!(HttpStatus::NotModified as u16, 304);
        assert_eq!(HttpStatus::BadRequest as u16, 400);
        assert_eq!(HttpStatus::Unauthorized as u16, 401);
        assert_eq!(HttpStatus::Forbidden as u16, 403);
        assert_eq!(HttpStatus::NotFound as u16, 404);
        assert_eq!(HttpStatus::InternalServerError as u16, 500);
    }

    #[test]
    fn test_http_status_text() {
        assert_eq!(HttpStatus::Ok.status_text(), "OK");
        assert_eq!(HttpStatus::Created.status_text(), "Created");
        assert_eq!(HttpStatus::Accepted.status_text(), "Accepted");
        assert_eq!(HttpStatus::NoContent.status_text(), "No Content");
        assert_eq!(HttpStatus::MovedPermanently.status_text(), "Moved Permanently");
        assert_eq!(HttpStatus::Found.status_text(), "Found");
        assert_eq!(HttpStatus::NotModified.status_text(), "Not Modified");
        assert_eq!(HttpStatus::BadRequest.status_text(), "Bad Request");
        assert_eq!(HttpStatus::Unauthorized.status_text(), "Unauthorized");
        assert_eq!(HttpStatus::Forbidden.status_text(), "Forbidden");
        assert_eq!(HttpStatus::NotFound.status_text(), "Not Found");
        assert_eq!(HttpStatus::InternalServerError.status_text(), "Internal Server Error");
    }

    #[test]
    fn test_http_status_categories() {
        assert!(HttpStatus::Continue.is_informational());
        assert!(HttpStatus::SwitchingProtocols.is_informational());
        
        assert!(HttpStatus::Ok.is_success());
        assert!(HttpStatus::Created.is_success());
        assert!(HttpStatus::Accepted.is_success());
        assert!(HttpStatus::NoContent.is_success());
        
        assert!(HttpStatus::MovedPermanently.is_redirection());
        assert!(HttpStatus::Found.is_redirection());
        assert!(HttpStatus::NotModified.is_redirection());
        
        assert!(HttpStatus::BadRequest.is_client_error());
        assert!(HttpStatus::Unauthorized.is_client_error());
        assert!(HttpStatus::Forbidden.is_client_error());
        assert!(HttpStatus::NotFound.is_client_error());
        
        assert!(HttpStatus::InternalServerError.is_server_error());
        assert!(HttpStatus::NotImplemented.is_server_error());
        assert!(HttpStatus::BadGateway.is_server_error());
    }

    #[test]
    fn test_http_headers_creation() {
        let headers = HttpHeaders::new();
        assert_eq!(headers.iter().count(), 0);
    }

    #[test]
    fn test_http_headers_set_get() {
        let mut headers = HttpHeaders::new();
        
        headers.set("Content-Type", "text/html");
        headers.set("Content-Length", "1234");
        headers.set("User-Agent", "Matte-Browser/1.0");
        
        assert_eq!(headers.get("content-type"), Some(&"text/html".to_string()));
        assert_eq!(headers.get("content-length"), Some(&"1234".to_string()));
        assert_eq!(headers.get("user-agent"), Some(&"Matte-Browser/1.0".to_string()));
        
        // Test case insensitivity
        assert_eq!(headers.get("Content-Type"), Some(&"text/html".to_string()));
        assert_eq!(headers.get("CONTENT-TYPE"), Some(&"text/html".to_string()));
    }

    #[test]
    fn test_http_headers_contains() {
        let mut headers = HttpHeaders::new();
        
        headers.set("Content-Type", "text/html");
        
        assert!(headers.contains("content-type"));
        assert!(headers.contains("Content-Type"));
        assert!(!headers.contains("content-length"));
    }

    #[test]
    fn test_http_headers_remove() {
        let mut headers = HttpHeaders::new();
        
        headers.set("Content-Type", "text/html");
        assert!(headers.contains("content-type"));
        
        let removed = headers.remove("content-type");
        assert_eq!(removed, Some("text/html".to_string()));
        assert!(!headers.contains("content-type"));
    }

    #[test]
    fn test_http_headers_content_length() {
        let mut headers = HttpHeaders::new();
        
        headers.set("Content-Length", "1234");
        assert_eq!(headers.content_length(), Some(1234));
        
        headers.set("Content-Length", "invalid");
        assert_eq!(headers.content_length(), None);
    }

    #[test]
    fn test_http_headers_transfer_encoding() {
        let mut headers = HttpHeaders::new();
        
        headers.set("Transfer-Encoding", "chunked");
        assert_eq!(headers.transfer_encoding(), Some(&"chunked".to_string()));
        assert!(headers.is_chunked());
        
        headers.set("Transfer-Encoding", "gzip");
        assert!(!headers.is_chunked());
    }

    #[test]
    fn test_http_headers_connection() {
        let mut headers = HttpHeaders::new();
        
        headers.set("Connection", "keep-alive");
        assert_eq!(headers.connection(), Some(&"keep-alive".to_string()));
        assert!(headers.is_keep_alive());
        
        headers.set("Connection", "close");
        assert!(!headers.is_keep_alive());
    }

    #[test]
    fn test_http_request_creation() {
        let request = HttpRequest::new(HttpMethod::GET, "/test".to_string());
        
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.uri, "/test");
        assert_eq!(request.version, HttpVersion::Http1_1);
        assert!(request.body.is_none());
        assert!(request.timeout.is_none());
        assert!(request.follow_redirects);
        assert_eq!(request.max_redirects, 5);
    }

    #[test]
    fn test_http_request_builder() {
        let request = HttpRequest::new(HttpMethod::POST, "/api/data".to_string())
            .method(HttpMethod::PUT)
            .uri("/api/update".to_string())
            .version(HttpVersion::Http2_0)
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer token")
            .body(b"{\"key\":\"value\"}".to_vec())
            .timeout(Duration::from_secs(30))
            .follow_redirects(false)
            .max_redirects(3);
        
        assert_eq!(request.method, HttpMethod::PUT);
        assert_eq!(request.uri, "/api/update");
        assert_eq!(request.version, HttpVersion::Http2_0);
        assert_eq!(request.headers.get("content-type"), Some(&"application/json".to_string()));
        assert_eq!(request.headers.get("authorization"), Some(&"Bearer token".to_string()));
        assert_eq!(request.body, Some(b"{\"key\":\"value\"}".to_vec()));
        assert_eq!(request.timeout, Some(Duration::from_secs(30)));
        assert!(!request.follow_redirects);
        assert_eq!(request.max_redirects, 3);
    }

    #[test]
    fn test_http_request_serialization() {
        let mut request = HttpRequest::new(HttpMethod::POST, "/api/data".to_string());
        request.headers.set("Content-Type", "application/json");
        request.headers.set("Content-Length", "17");
        request.body = Some(b"{\"key\":\"value\"}".to_vec());
        
        let serialized = request.serialize().unwrap();
        let request_str = String::from_utf8(serialized).unwrap();
        
        assert!(request_str.starts_with("POST /api/data HTTP/1.1\r\n"));
        assert!(request_str.contains("Content-Type: application/json\r\n"));
        assert!(request_str.contains("Content-Length: 17\r\n"));
        assert!(request_str.contains("\r\n\r\n"));
        assert!(request_str.ends_with("{\"key\":\"value\"}"));
    }

    #[test]
    fn test_http_response_creation() {
        let response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::Ok);
        
        assert_eq!(response.version, HttpVersion::Http1_1);
        assert_eq!(response.status, HttpStatus::Ok);
        assert_eq!(response.status_text, "OK");
        assert!(response.body.is_empty());
        assert_eq!(response.content_length, None);
        assert!(!response.chunked);
    }

    #[test]
    fn test_http_response_builder() {
        let response = HttpResponse::new(HttpVersion::Http2_0, HttpStatus::Created)
            .status(HttpStatus::Accepted)
            .header("Content-Type", "text/html")
            .header("Content-Length", "1234")
            .body(b"<html>Hello</html>".to_vec());
        
        assert_eq!(response.version, HttpVersion::Http2_0);
        assert_eq!(response.status, HttpStatus::Accepted);
        assert_eq!(response.status_text, "Accepted");
        assert_eq!(response.headers.get("content-type"), Some(&"text/html".to_string()));
        assert_eq!(response.headers.get("content-length"), Some(&"1234".to_string()));
        assert_eq!(response.body, b"<html>Hello</html>");
        assert_eq!(response.content_length, Some(17));
    }

    #[test]
    fn test_http_response_status_checks() {
        let success_response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::Ok);
        let redirect_response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::MovedPermanently);
        let client_error_response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::NotFound);
        let server_error_response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::InternalServerError);
        
        assert!(success_response.is_success());
        assert!(redirect_response.is_redirection());
        assert!(client_error_response.is_client_error());
        assert!(server_error_response.is_server_error());
    }

    #[test]
    fn test_http_response_redirect_location() {
        let mut response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::MovedPermanently);
        response.headers.set("Location", "/new-location");
        
        assert_eq!(response.redirect_location(), Some(&"/new-location".to_string()));
        
        let non_redirect_response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::Ok);
        assert_eq!(non_redirect_response.redirect_location(), None);
    }

    #[test]
    fn test_http_response_parsing() {
        let response_data = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, World!";
        
        let response = HttpResponse::parse(response_data).unwrap();
        
        assert_eq!(response.version, HttpVersion::Http1_1);
        assert_eq!(response.status, HttpStatus::Ok);
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.headers.get("content-type"), Some(&"text/html".to_string()));
        assert_eq!(response.headers.get("content-length"), Some(&"13".to_string()));
        assert_eq!(response.body, b"Hello, World!");
        assert_eq!(response.content_length, Some(13));
    }

    #[test]
    fn test_http_response_parsing_invalid() {
        // Missing status line
        let invalid_data = b"Content-Type: text/html\r\n\r\n";
        assert!(HttpResponse::parse(invalid_data).is_err());
        
        // Invalid status code
        let invalid_status = b"HTTP/1.1 999 Invalid\r\n\r\n";
        assert!(HttpResponse::parse(invalid_status).is_err());
        
        // Invalid HTTP version
        let invalid_version = b"INVALID/1.1 200 OK\r\n\r\n";
        assert!(HttpResponse::parse(invalid_version).is_err());
    }

    #[test]
    fn test_http_connection_pool_creation() {
        let pool = HttpConnectionPool::new(
            Duration::from_secs(30),
            10,
            100
        );
        
        let stats = pool.get_stats();
        assert_eq!(stats.total_connections, 0);
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.idle_connections, 0);
        assert_eq!(stats.reuse_count, 0);
    }

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new();
        
        assert_eq!(client.default_timeout, Duration::from_secs(30));
        assert_eq!(client.user_agent, "Matte-Browser/1.0");
        assert!(client.follow_redirects);
        assert_eq!(client.max_redirects, 5);
        
        let stats = client.get_connection_pool_stats();
        assert_eq!(stats.total_connections, 0);
    }

    #[test]
    fn test_http_client_configuration() {
        let mut client = HttpClient::new();
        
        client.set_default_timeout(Duration::from_secs(60));
        client.set_user_agent("Custom-Agent/2.0".to_string());
        client.set_default_header("X-Custom", "value");
        client.set_follow_redirects(false);
        client.set_max_redirects(10);
        
        assert_eq!(client.default_timeout, Duration::from_secs(60));
        assert_eq!(client.user_agent, "Custom-Agent/2.0");
        assert!(!client.follow_redirects);
        assert_eq!(client.max_redirects, 10);
    }

    #[test]
    fn test_uri_parsing() {
        let client = HttpClient::new();
        
        // HTTP URLs
        assert_eq!(client.parse_uri("http://example.com").unwrap(), ("example.com".to_string(), 80));
        assert_eq!(client.parse_uri("http://example.com:8080").unwrap(), ("example.com".to_string(), 8080));
        
        // HTTPS URLs
        assert_eq!(client.parse_uri("https://example.com").unwrap(), ("example.com".to_string(), 443));
        assert_eq!(client.parse_uri("https://example.com:8443").unwrap(), ("example.com".to_string(), 8443));
        
        // URLs without scheme
        assert_eq!(client.parse_uri("example.com").unwrap(), ("example.com".to_string(), 80));
        assert_eq!(client.parse_uri("example.com:8080").unwrap(), ("example.com".to_string(), 8080));
    }

    #[test]
    fn test_uri_parsing_invalid() {
        let client = HttpClient::new();
        
        // Invalid port
        assert!(client.parse_uri("http://example.com:invalid").is_err());
        assert!(client.parse_uri("https://example.com:99999").is_err());
    }

    #[test]
    fn test_http2_settings_default() {
        let settings = Http2Settings::default();
        
        assert_eq!(settings.header_table_size, 4096);
        assert!(settings.enable_push);
        assert_eq!(settings.max_concurrent_streams, 100);
        assert_eq!(settings.initial_window_size, 65535);
        assert_eq!(settings.max_frame_size, 16384);
        assert_eq!(settings.max_header_list_size, 262144);
    }

    #[test]
    fn test_http2_frame_creation() {
        let payload = b"test payload".to_vec();
        let frame = Http2Frame::new(Http2FrameType::Data, 1, payload.clone());
        
        assert_eq!(frame.length, 12);
        assert_eq!(frame.frame_type, Http2FrameType::Data);
        assert_eq!(frame.flags, 0);
        assert_eq!(frame.stream_id, 1);
        assert_eq!(frame.payload, payload);
    }

    #[test]
    fn test_http2_frame_serialization() {
        let payload = b"test".to_vec();
        let frame = Http2Frame::new(Http2FrameType::Headers, 1, payload);
        
        let serialized = frame.serialize();
        
        // Frame header should be 9 bytes
        assert_eq!(serialized.len(), 13); // 9 bytes header + 4 bytes payload
        
        // Check frame length (first 3 bytes, big-endian)
        assert_eq!(serialized[0], 0);
        assert_eq!(serialized[1], 0);
        assert_eq!(serialized[2], 4);
        
        // Check frame type
        assert_eq!(serialized[3], 1); // Headers frame type
        
        // Check flags
        assert_eq!(serialized[4], 0);
        
        // Check stream ID (last 4 bytes of header, big-endian)
        assert_eq!(serialized[5], 0);
        assert_eq!(serialized[6], 0);
        assert_eq!(serialized[7], 0);
        assert_eq!(serialized[8], 1);
        
        // Check payload
        assert_eq!(&serialized[9..], b"test");
    }

    #[test]
    fn test_http2_frame_parsing() {
        let frame_data = [
            0x00, 0x00, 0x04, // Length: 4
            0x01, // Frame type: Headers
            0x00, // Flags: 0
            0x00, 0x00, 0x00, 0x01, // Stream ID: 1
            0x74, 0x65, 0x73, 0x74, // Payload: "test"
        ];
        
        let frame = Http2Frame::parse(&frame_data).unwrap();
        
        assert_eq!(frame.length, 4);
        assert_eq!(frame.frame_type, Http2FrameType::Headers);
        assert_eq!(frame.flags, 0);
        assert_eq!(frame.stream_id, 1);
        assert_eq!(frame.payload, b"test");
    }

    #[test]
    fn test_http2_frame_parsing_invalid() {
        // Frame too short
        let short_data = [0x00, 0x00, 0x01];
        assert!(Http2Frame::parse(&short_data).is_err());
        
        // Unknown frame type
        let unknown_type = [
            0x00, 0x00, 0x00, // Length: 0
            0xFF, // Unknown frame type
            0x00, // Flags: 0
            0x00, 0x00, 0x00, 0x01, // Stream ID: 1
        ];
        assert!(Http2Frame::parse(&unknown_type).is_err());
    }

    #[test]
    fn test_hpack_codec_creation() {
        let codec = HpackCodec::new(4096);
        
        assert_eq!(codec.max_table_size, 4096);
        assert_eq!(codec.current_table_size, 0);
        assert!(codec.dynamic_table.is_empty());
    }

    #[test]
    fn test_hpack_codec_encoding() {
        let mut codec = HpackCodec::new(4096);
        let mut headers = HttpHeaders::new();
        
        headers.set("Content-Type", "text/html");
        headers.set("Content-Length", "1234");
        
        let encoded = codec.encode(&headers).unwrap();
        
        // Should contain the header data
        assert!(!encoded.is_empty());
        assert!(encoded.len() > 0);
    }

    #[test]
    fn test_hpack_codec_decoding() {
        let mut codec = HpackCodec::new(4096);
        
        // Simple literal encoding
        let encoded = [
            0x40, // Literal header field with incremental indexing
            0x0C, // Name length: 12
            0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D, 0x54, 0x79, 0x70, 0x65, // "Content-Type"
            0x09, // Value length: 9
            0x74, 0x65, 0x78, 0x74, 0x2F, 0x68, 0x74, 0x6D, 0x6C, // "text/html"
        ];
        
        let headers = codec.decode(&encoded).unwrap();
        
        assert_eq!(headers.get("content-type"), Some(&"text/html".to_string()));
    }

    #[test]
    fn test_hpack_codec_decode_invalid() {
        let mut codec = HpackCodec::new(4096);
        
        // Invalid encoding (too short)
        let invalid_data = [0x40, 0x0C];
        assert!(codec.decode(&invalid_data).is_err());
    }

    #[tokio::test]
    async fn test_http_connection_pool_get_connection() {
        let pool = HttpConnectionPool::new(
            Duration::from_secs(30),
            10,
            100
        );
        
        // This will fail in test environment since we can't actually connect
        // but we can test the error handling
        let result = pool.get_connection("localhost", 80).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_http_connection_pool_return_connection() {
        let pool = HttpConnectionPool::new(
            Duration::from_secs(30),
            10,
            100
        );
        
        // Create a mock connection (this would normally come from get_connection)
        let mock_stream = std::net::TcpStream::connect("127.0.0.1:0").unwrap();
        let connection = HttpConnection::new(
            mock_stream,
            "127.0.0.1:80".parse().unwrap(),
            Duration::from_secs(30)
        );
        
        pool.return_connection(connection);
        
        let stats = pool.get_stats();
        assert_eq!(stats.idle_connections, 1);
    }

    #[tokio::test]
    async fn test_http_connection_pool_cleanup() {
        let pool = HttpConnectionPool::new(
            Duration::from_millis(100), // Short timeout for testing
            10,
            100
        );
        
        // Create and return a connection
        let mock_stream = std::net::TcpStream::connect("127.0.0.1:0").unwrap();
        let connection = HttpConnection::new(
            mock_stream,
            "127.0.0.1:80".parse().unwrap(),
            Duration::from_millis(100)
        );
        
        pool.return_connection(connection);
        
        // Wait for connection to expire
        tokio::time::sleep(Duration::from_millis(200)).await;
        
        // Clean up expired connections
        pool.cleanup_expired();
        
        let stats = pool.get_stats();
        assert_eq!(stats.idle_connections, 0);
    }

    #[test]
    fn test_http_connection_expiration() {
        let mock_stream = std::net::TcpStream::connect("127.0.0.1:0").unwrap();
        let connection = HttpConnection::new(
            mock_stream,
            "127.0.0.1:80".parse().unwrap(),
            Duration::from_secs(30)
        );
        
        // Connection should not be expired immediately
        assert!(!connection.is_expired(Duration::from_secs(30)));
        
        // Connection should be expired after a long time
        assert!(connection.is_expired(Duration::from_millis(1)));
    }

    #[test]
    fn test_http_connection_id_management() {
        let mock_stream = std::net::TcpStream::connect("127.0.0.1:0").unwrap();
        let mut connection = HttpConnection::new(
            mock_stream,
            "127.0.0.1:80".parse().unwrap(),
            Duration::from_secs(30)
        );
        
        assert_eq!(connection.id(), 0);
        
        connection.set_id(12345);
        assert_eq!(connection.id(), 12345);
    }

    #[test]
    fn test_comprehensive_http_workflow() {
        // Test complete HTTP request/response workflow
        let mut request = HttpRequest::new(HttpMethod::POST, "/api/data".to_string());
        request.headers.set("Content-Type", "application/json");
        request.headers.set("Authorization", "Bearer token123");
        request.body = Some(b"{\"message\":\"hello\"}".to_vec());
        
        // Serialize request
        let request_data = request.serialize().unwrap();
        assert!(!request_data.is_empty());
        
        // Create response
        let mut response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::Ok);
        response.headers.set("Content-Type", "application/json");
        response.headers.set("Content-Length", "25");
        response.body = b"{\"status\":\"success\"}".to_vec();
        
        // Verify response
        assert!(response.is_success());
        assert_eq!(response.content_length, Some(25));
        assert_eq!(response.headers.get("content-type"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_http_redirect_handling() {
        let mut redirect_response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::MovedPermanently);
        redirect_response.headers.set("Location", "/new-location");
        
        assert!(redirect_response.is_redirection());
        assert_eq!(redirect_response.redirect_location(), Some(&"/new-location".to_string()));
        
        let normal_response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::Ok);
        assert!(!normal_response.is_redirection());
        assert_eq!(normal_response.redirect_location(), None);
    }

    #[test]
    fn test_http_chunked_encoding() {
        let mut response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::Ok);
        response.headers.set("Transfer-Encoding", "chunked");
        
        assert!(response.headers.is_chunked());
        assert!(response.chunked);
        
        let normal_response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::Ok);
        response.headers.set("Transfer-Encoding", "gzip");
        
        assert!(!normal_response.headers.is_chunked());
    }

    #[test]
    fn test_http_keep_alive() {
        let mut response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::Ok);
        response.headers.set("Connection", "keep-alive");
        
        assert!(response.headers.is_keep_alive());
        
        let close_response = HttpResponse::new(HttpVersion::Http1_1, HttpStatus::Ok);
        close_response.headers.set("Connection", "close");
        
        assert!(!close_response.headers.is_keep_alive());
    }

    #[test]
    fn test_http2_frame_types() {
        let frame_types = vec![
            Http2FrameType::Data,
            Http2FrameType::Headers,
            Http2FrameType::Priority,
            Http2FrameType::RstStream,
            Http2FrameType::Settings,
            Http2FrameType::PushPromise,
            Http2FrameType::Ping,
            Http2FrameType::Goaway,
            Http2FrameType::WindowUpdate,
            Http2FrameType::Continuation,
        ];
        
        for frame_type in frame_types {
            let payload = b"test".to_vec();
            let frame = Http2Frame::new(frame_type, 1, payload);
            assert_eq!(frame.frame_type, frame_type);
        }
    }

    #[test]
    fn test_http2_stream_states() {
        let states = vec![
            Http2StreamState::Idle,
            Http2StreamState::ReservedLocal,
            Http2StreamState::ReservedRemote,
            Http2StreamState::Open,
            Http2StreamState::HalfClosedLocal,
            Http2StreamState::HalfClosedRemote,
            Http2StreamState::Closed,
        ];
        
        for state in states {
            let stream = Http2Stream {
                id: 1,
                state,
                priority: 0,
                window_size: 65535,
                headers: HttpHeaders::new(),
                data: Vec::new(),
            };
            assert_eq!(stream.state, state);
        }
    }
}
