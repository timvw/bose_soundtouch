use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, tungstenite::client::IntoClientRequest};
use log::{info, error, warn};
use anyhow::{Result, Context};
use tokio::sync::broadcast;
use quick_xml::de::from_str;
use url::Url;
use crate::types::*;

pub struct SoundTouchWebSocket {
    host: String,
    event_tx: broadcast::Sender<SoundTouchEvent>,
}

// ... copy rest of websocket.rs implementation from soundtouch_api ... 