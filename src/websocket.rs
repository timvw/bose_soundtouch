use futures_util::StreamExt;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        protocol::Message,
        client::IntoClientRequest,
    },
};
use log::{info, error, warn};
use tokio::sync::broadcast;
use quick_xml::de::from_str;
use url::Url;
use crate::{types::*, error::{Result, BoseError}};

pub struct SoundTouchWebSocket {
    host: String,
    event_tx: broadcast::Sender<SoundTouchEvent>,
}

impl SoundTouchWebSocket {
    pub fn new(host: String) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self { host, event_tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SoundTouchEvent> {
        self.event_tx.subscribe()
    }

    pub async fn connect_and_listen(&self) -> Result<()> {
        let url_str = format!("ws://{}:8080", self.host);
        let url = Url::parse(&url_str).map_err(BoseError::UrlParseError)?;

        info!("Connecting to {}", url);

        let mut request = url.into_client_request()
            .map_err(|e| BoseError::ProtocolError(e.to_string()))?;
            
        request.headers_mut().insert(
            "Sec-WebSocket-Protocol",
            "gabbo".parse()
                .map_err(|e| BoseError::ProtocolError(format!("Failed to parse protocol header: {}", e)))?
        );

        let (ws_stream, response) = connect_async(request)
            .await
            .map_err(BoseError::ConnectionError)?;

        if response.headers().get("Sec-WebSocket-Protocol").map(|h| h.as_bytes()) != Some(b"gabbo") {
            return Err(BoseError::ProtocolError("Server did not accept gabbo protocol".to_string()));
        }

        info!("WebSocket connection established with gabbo protocol");

        let (_, mut read) = ws_stream.split();
        let event_tx = self.event_tx.clone();

        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Some(event) = self.parse_event(&text) {
                        if let Err(e) = event_tx.send(event) {
                            error!("Failed to send event: {}", e);
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed by server");
                    if let Err(e) = event_tx.send(SoundTouchEvent::Disconnected) {
                        error!("Failed to send disconnect event: {}", e);
                    }
                    break;
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn parse_event(&self, text: &str) -> Option<SoundTouchEvent> {
        // First try to parse device info
        if text.contains("SoundTouchSdkInfo") {
            return match from_str::<DeviceInfo>(text) {
                Ok(info) => Some(SoundTouchEvent::DeviceInfo(info)),
                Err(e) => {
                    warn!("Failed to parse DeviceInfo: {}", e);
                    None
                }
            };
        }

        // Then try user activity
        if text.contains("userActivityUpdate") {
            return match from_str::<UserActivity>(text) {
                Ok(activity) => Some(SoundTouchEvent::UserActivity(activity)),
                Err(e) => {
                    warn!("Failed to parse UserActivity: {}", e);
                    None
                }
            };
        }

        // Try to parse updates wrapper
        if text.contains("<updates") {
            match from_str::<Updates>(text) {
                Ok(updates) => {
                    // Convert the update to an event
                    if let Some(volume) = updates.volume_updated {
                        Some(SoundTouchEvent::VolumeUpdated(volume))
                    } else if let Some(now_playing) = updates.now_playing_updated {
                        Some(SoundTouchEvent::NowPlayingUpdated(now_playing))
                    } else if let Some(recents) = updates.recents_updated {
                        Some(SoundTouchEvent::RecentsUpdated(recents))
                    } else if let Some(connection) = updates.connection_state_updated {
                        Some(SoundTouchEvent::ConnectionStateUpdated(connection))
                    } else {
                        warn!("Unknown update type in: {}", text);
                        None
                    }
                }
                Err(e) => {
                    warn!("Failed to parse Updates: {} from text: {}", e, text);
                    None
                }
            }
        } else {
            warn!("Unhandled message type: {}", text);
            None
        }
    }

    pub async fn connect_and_listen_with_retry(&self) -> Result<()> {
        let mut retry_delay = tokio::time::Duration::from_secs(1);
        const MAX_DELAY: tokio::time::Duration = tokio::time::Duration::from_secs(60);

        loop {
            match self.connect_and_listen().await {
                Ok(()) => {
                    // Clean disconnect, reset retry delay
                    retry_delay = tokio::time::Duration::from_secs(1);
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    tokio::time::sleep(retry_delay).await;
                    retry_delay = std::cmp::min(retry_delay * 2, MAX_DELAY);
                }
            }
        }
    }
} 