//! Error types for the Bose SoundTouch API

use quick_xml::errors::serialize::SeError;
use thiserror::Error;

/// Errors that can occur when using the SoundTouch API
#[derive(Error, Debug)]
pub enum BoseError {
    /// Invalid preset number was specified (valid range: 1-6)
    #[error("Invalid Preset")]
    InvalidPreset(String),

    /// Failed to deserialize XML data
    #[error("Failed to deserialize from XML")]
    XmlError(#[from] quick_xml::DeError),

    /// Failed to serialize XML data
    #[error("Failed to serialize to XML")]
    SerializeError(#[from] SeError),

    /// HTTP client encountered an error
    #[error("Http client issue")]
    HttpClientError(#[from] reqwest::Error),

    /// Failed to establish WebSocket connection
    #[cfg(feature = "websocket")]
    #[error("Failed to connect to WebSocket: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),

    /// Failed to parse WebSocket URL
    #[cfg(feature = "websocket")]
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// WebSocket protocol error occurred
    #[cfg(feature = "websocket")]
    #[error("Protocol error: {0}")]
    ProtocolError(String),

    /// WebSocket connection was closed
    #[cfg(feature = "websocket")]
    #[error("WebSocket closed")]
    WebSocketClosed,
}

/// Result type for SoundTouch API operations
pub type Result<T> = std::result::Result<T, BoseError>;
