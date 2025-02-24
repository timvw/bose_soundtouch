//! Error types for the SoundTouch API

use thiserror::Error;

/// Errors that can occur when using the SoundTouch API
#[derive(Error, Debug)]
pub enum SoundTouchError {
    #[error("Failed to connect to WebSocket: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Failed to parse XML: {0}")]
    XmlParseError(#[from] quick_xml::DeError),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("WebSocket closed")]
    WebSocketClosed,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
} 