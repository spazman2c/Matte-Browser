pub mod error;
pub mod http;
pub mod tls;

pub use error::{Error, Result};
pub use http::{
    HttpMethod, HttpVersion, HttpStatus, HttpHeaders, HttpRequest, HttpResponse,
    HttpConnection, HttpConnectionPool, ConnectionPoolStats, HttpClient,
    Http2Settings, Http2FrameType, Http2Frame, Http2Stream, Http2StreamState,
    Http2Connection, HpackCodec,
};
pub use tls::{
    TlsVersion, TlsCipherSuite, TlsSignatureAlgorithm, TlsCertificate, CertificateValidationResult,
    OcspResponse, OcspResponseStatus, OcspCertStatus, CertificatePinning, HstsConfig, HstsEntry,
    TlsConfig, TlsSession, TlsConnectionState, TlsConnection, TlsClient, TlsServer, OcspResponder,
};

#[cfg(test)]
mod http_test;
#[cfg(test)]
mod tls_test;
