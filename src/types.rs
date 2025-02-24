use serde::{Deserialize, Serialize};

// HTTP API types
#[derive(Serialize, Deserialize, Debug)]
pub struct BoseClient {
    pub hostname: String,
}

// WebSocket types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub server_version: String,
    pub server_build: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    pub device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    #[serde(rename = "targetvolume")]
    pub target_volume: u8,
    #[serde(rename = "actualvolume")]
    pub actual_volume: u8,
    #[serde(rename = "muteenabled")]
    pub mute_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeUpdate {
    pub volume: Volume,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@type", default)]
    pub item_type: Option<String>,
    #[serde(rename = "@location", default)]
    pub location: Option<String>,
    #[serde(rename = "@sourceAccount", default)]
    pub source_account: Option<String>,
    #[serde(rename = "@isPresetable", default)]
    pub is_presetable: bool,
    #[serde(rename = "itemName")]
    pub item_name: Option<String>,
    #[serde(rename = "containerArt")]
    pub container_art: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowPlaying {
    #[serde(rename = "@deviceID")]
    pub device_id: String,
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@sourceAccount", default)]
    pub source_account: Option<String>,
    #[serde(rename = "ContentItem")]
    pub content_item: ContentItem,
    pub track: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    #[serde(rename = "stationName")]
    pub station_name: Option<String>,
    pub art: Option<String>,
    #[serde(rename = "@artImageStatus")]
    pub art_status: Option<String>,
    #[serde(rename = "playStatus")]
    pub play_status: String,
    #[serde(rename = "streamType")]
    pub stream_type: Option<String>,
    #[serde(rename = "favoriteEnabled")]
    pub favorite_enabled: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowPlayingUpdate {
    #[serde(rename = "nowPlaying")]
    pub now_playing: NowPlaying,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub id: u8,
    #[serde(rename = "ContentItem")]
    pub content_item: ContentItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recent {
    pub device_id: String,
    pub utc_time: u64,
    pub id: String,
    #[serde(rename = "contentItem")]
    pub content_item: ContentItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recents {
    pub recent: Vec<Recent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentsUpdate {
    pub recents: Recents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    #[serde(rename = "@state")]
    pub state: String,
    #[serde(rename = "@up")]
    pub up: bool,
    #[serde(rename = "@signal")]
    pub signal: String,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum SoundTouchEvent {
    DeviceInfo(DeviceInfo),
    UserActivity(UserActivity),
    VolumeUpdated(VolumeUpdate),
    NowPlayingUpdated(NowPlayingUpdate),
    PresetSelected(Preset),
    RecentsUpdated(RecentsUpdate),
    ConnectionStateUpdated(ConnectionState),
    Disconnected,
}

#[derive(Serialize, Deserialize)]
pub struct Updates {
    #[serde(rename = "@deviceID")]
    pub device_id: String,
    #[serde(rename = "volumeUpdated")]
    pub volume_updated: Option<VolumeUpdate>,
    #[serde(rename = "nowPlayingUpdated")]
    pub now_playing_updated: Option<NowPlayingUpdate>,
    #[serde(rename = "recentsUpdated")]
    pub recents_updated: Option<RecentsUpdate>,
    #[serde(rename = "connectionStateUpdated")]
    pub connection_state_updated: Option<ConnectionState>,
}

impl std::fmt::Debug for Updates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Updates {{")?;
        writeln!(f, "  device_id: {}", self.device_id)?;
        if let Some(ref v) = self.volume_updated {
            writeln!(f, "  volume_updated: {:?}", v)?;
        }
        if let Some(ref n) = self.now_playing_updated {
            writeln!(f, "  now_playing_updated: {:?}", n)?;
        }
        if let Some(ref r) = self.recents_updated {
            writeln!(f, "  recents_updated: {:?}", r)?;
        }
        if let Some(ref c) = self.connection_state_updated {
            writeln!(f, "  connection_state_updated: {:?}", c)?;
        }
        write!(f, "}}")
    }
}

// ... rest of the types from soundtouch_api/src/types.rs ... 