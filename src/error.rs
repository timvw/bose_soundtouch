//! Error types for the SoundTouch API

use thiserror::Error;

/// Errors that can occur when using the SoundTouch API
#[derive(Error, Debug)]
pub enum BoseError {
    // HTTP API errors
    #[error("Invalid Preset")]
    InvalidPreset(String),

    #[error("Failed to (de)serialize from XML")]
    XmlError(#[from] quick_xml::DeError),

    #[error("Http client issue")]
    HttpClientError(#[from] reqwest::Error),

    // WebSocket errors
    #[error("Failed to connect to WebSocket: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("WebSocket closed")]
    WebSocketClosed,
}

pub type Result<T> = std::result::Result<T, BoseError>; 