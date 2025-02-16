/*!
An easy to use client for the Bose SoundTouch API.

# Getting started

Add `bose_soundtouch` to your `Cargo.toml`:

```toml
[dependencies]
bose_soundtouch = { version = "1" }
tokio = { version = "1", features = ["full"] }
```

## Getting the status of your speaker

```rust
use bose_soundtouch::BoseClient;

#[tokio::main]
async fn main() {
    let client = BoseClient::new("192.168.1.143");
    let status = client.get_status().await.unwrap();
    println!("status: {:?}", status);
}
```

*/

use reqwest::{Client, IntoUrl};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;
use thiserror::Error;

/// Client for interacting with Bose SoundTouch devices
///
/// Provides methods to control playback, volume, presets, and device settings
/// through the SoundTouch HTTP API.
#[derive(Serialize, Deserialize, Debug)]
pub struct BoseClient {
    hostname: String,
}

/// Errors that can occur when interacting with the Bose SoundTouch API
#[derive(Error, Debug)]
pub enum BoseClientError {
    /// Invalid preset number was specified (valid range: 1-6)
    #[error("Invalid Preset")]
    InvalidPreset(String),
    /// Failed to serialize or deserialize XML data
    #[error("Failed to (de)serialize from XML")]
    XmlError(#[from] quick_xml::DeError),
    /// HTTP client encountered an error
    #[error("Http client issue")]
    HttpClientError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, BoseClientError>;

impl BoseClient {
    /// Creates a new BoseClient instance
    ///
    /// # Arguments
    /// * `hostname` - IP address or hostname of the SoundTouch device
    pub fn new(hostname: &str) -> BoseClient {
        BoseClient {
            hostname: String::from(hostname),
        }
    }

    /// Stops playback
    pub async fn stop(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Stop).await
    }

    /// Skips to next track
    pub async fn next_track(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::NextTrack).await
    }

    /// Returns to previous track
    pub async fn prev_track(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::PrevTrack).await
    }

    /// Toggles mute state
    pub async fn mute(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Mute).await
    }

    /// Gets information about the device
    pub async fn get_info(&self) -> Result<DeviceInfo> {
        let url = format!("http://{}:8090/info", &self.hostname);
        get_xml(url).await
    }

    /// Sets the device name
    ///
    /// # Arguments
    /// * `name` - New name for the device
    pub async fn set_name(&self, name: &str) -> Result<()> {
        let url = format!("http://{}:8090/name", &self.hostname);
        post_xml(
            &url,
            &DeviceName {
                name: name.to_string(),
            },
        )
        .await
    }

    /// Starts playback
    pub async fn play(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Play).await
    }

    /// Pauses playback
    pub async fn pause(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Pause).await
    }

    /// Toggles power state
    pub async fn power(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Power).await
    }

    /// Simulates pressing and releasing a key on the remote
    ///
    /// # Arguments
    /// * `key_value` - The key to simulate pressing
    pub async fn press_and_release_key(&self, key_value: &KeyValue) -> Result<()> {
        let url = format!("http://{}:8090/key", &self.hostname);
        post_xml(&url, &PostKey::press(key_value)).await?;
        post_xml(&url, &PostKey::release(key_value)).await?;
        Ok(())
    }

    /// Gets the current playback status
    pub async fn get_status(&self) -> Result<NowPlaying> {
        let url = format!("http://{}:8090/now_playing", &self.hostname);
        get_xml(url).await
    }

    /// Gets the current volume settings
    pub async fn get_volume(&self) -> Result<Volume> {
        let url = format!("http://{}:8090/volume", &self.hostname);
        get_xml(url).await
    }

    /// Sets the volume level
    ///
    /// # Arguments
    /// * `value` - Volume level (0-100)
    pub async fn set_volume(&self, value: i32) -> Result<()> {
        let url = format!("http://{}:8090/volume", &self.hostname);
        post_xml(&url, &PostVolume::new(value)).await?;
        Ok(())
    }

    /// Gets the list of presets
    pub async fn get_presets(&self) -> Result<Presets> {
        let url = format!("http://{}:8090/presets", &self.hostname);
        get_xml(url).await
    }

    /// Selects a preset
    ///
    /// # Arguments
    /// * `value` - Preset number (1-6)
    ///
    /// # Errors
    /// Returns `BoseClientError::InvalidPreset` if the preset number is not between 1 and 6
    pub async fn set_preset(&self, value: i32) -> Result<()> {
        match value {
            1 => self.press_and_release_key(&KeyValue::Preset1).await,
            2 => self.press_and_release_key(&KeyValue::Preset2).await,
            3 => self.press_and_release_key(&KeyValue::Preset3).await,
            4 => self.press_and_release_key(&KeyValue::Preset4).await,
            5 => self.press_and_release_key(&KeyValue::Preset5).await,
            6 => self.press_and_release_key(&KeyValue::Preset6).await,
            _ => Err(BoseClientError::InvalidPreset(format!(
                "{} is not a valid preset (1-6).",
                value
            ))),
        }
    }

    /// Gets the list of available sources
    pub async fn get_sources(&self) -> Result<Sources> {
        let url = format!("http://{}:8090/sources", &self.hostname);
        get_xml(url).await
    }

    /// Selects a source for playback
    ///
    /// # Arguments
    /// * `source` - Source type (e.g., "BLUETOOTH", "AUX")
    /// * `source_account` - Optional account associated with the source
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    ///
    /// // Select Bluetooth source
    /// client.select_source("BLUETOOTH", None).await?;
    ///
    /// // Select AUX input
    /// client.select_source("AUX", Some("AUX")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn select_source(&self, source: &str, source_account: Option<&str>) -> Result<()> {
        let url = format!("http://{}:8090/select", &self.hostname);
        let source_data = SelectSource {
            source: source.to_string(),
            source_account: source_account.map(String::from),
        };
        post_xml(&url, &source_data).await
    }

    /// Selects the Bluetooth source
    ///
    /// This is a convenience method for selecting the Bluetooth input.
    pub async fn select_bluetooth(&self) -> Result<()> {
        self.select_source("BLUETOOTH", None).await
    }

    /// Selects the AUX input source
    ///
    /// This is a convenience method for selecting the AUX input.
    /// Some devices may have multiple AUX inputs (AUX, AUX1, AUX2, etc.).
    ///
    /// # Arguments
    /// * `input` - Optional AUX input number (e.g., None for "AUX", Some("AUX1") for specific input)
    pub async fn select_aux(&self, input: Option<&str>) -> Result<()> {
        let source_account = input.unwrap_or("AUX");
        self.select_source("AUX", Some(source_account)).await
    }

    /// Checks if a specific source is available
    ///
    /// # Arguments
    /// * `source` - Source type to check (e.g., "BLUETOOTH", "AUX")
    ///
    /// # Returns
    /// `true` if the source is available and ready, `false` otherwise
    pub async fn is_source_available(&self, source: &str) -> Result<bool> {
        let sources = self.get_sources().await?;
        Ok(sources
            .items
            .iter()
            .any(|item| item.source == source && item.status == SourceStatus::Ready))
    }

    /// Gets the current zone configuration
    ///
    /// Returns information about the current multi-room zone setup, including
    /// the master device and any connected slave devices.
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    /// let zone = client.get_zone().await?;
    /// println!("Master device: {}", zone.master);
    /// for member in zone.members {
    ///     println!("Member: {} at {}", member.mac_address, member.ip_address);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_zone(&self) -> Result<Zone> {
        let url = format!("http://{}:8090/getZone", &self.hostname);
        get_xml(url).await
    }

    /// Creates or updates a multi-room zone
    ///
    /// Sets up a new zone configuration with this device as the master and the specified
    /// devices as slaves. This will override any existing zone configuration.
    ///
    /// # Arguments
    /// * `slave_devices` - Vector of tuples containing (ip_address, mac_address) for each slave device
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    /// let slaves = vec![
    ///     ("192.168.1.144".to_string(), "00:11:22:33:44:55".to_string()),
    ///     ("192.168.1.145".to_string(), "AA:BB:CC:DD:EE:FF".to_string()),
    /// ];
    /// client.set_zone(&slaves).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_zone(&self, slave_devices: &[(String, String)]) -> Result<()> {
        let url = format!("http://{}:8090/setZone", &self.hostname);

        // Get the master device info
        let info = self.get_info().await?;
        let master_mac = info.device_id;

        // Create zone members (including master)
        let mut members = vec![ZoneMember {
            ip_address: self.hostname.clone(),
            mac_address: master_mac.clone(),
        }];

        // Add slave members
        members.extend(slave_devices.iter().map(|(ip, mac)| ZoneMember {
            ip_address: ip.clone(),
            mac_address: mac.clone(),
        }));

        let zone = Zone {
            master: master_mac,
            sender_ip_address: Some(self.hostname.clone()),
            members,
        };

        post_xml(&url, &zone).await
    }

    /// Adds a slave device to the existing zone
    ///
    /// Adds a new slave device to the current multi-room zone without affecting
    /// other existing slave devices.
    ///
    /// # Arguments
    /// * `slave_ip` - IP address of the slave device
    /// * `slave_mac` - MAC address of the slave device
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    /// client.add_zone_slave(
    ///     "192.168.1.144",
    ///     "00:11:22:33:44:55"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_zone_slave(&self, slave_ip: &str, slave_mac: &str) -> Result<()> {
        let url = format!("http://{}:8090/addZoneSlave", &self.hostname);

        // Get current zone to get master info
        let current_zone = self.get_zone().await?;

        // Create new member
        let new_member = ZoneMember {
            ip_address: slave_ip.to_string(),
            mac_address: slave_mac.to_string(),
        };

        // Create zone with just the new member
        let zone = Zone {
            master: current_zone.master,
            sender_ip_address: None,
            members: vec![new_member],
        };

        post_xml(&url, &zone).await
    }

    /// Removes a slave device from the zone
    ///
    /// Removes a specific slave device from the current multi-room zone without
    /// affecting other slave devices.
    ///
    /// # Arguments
    /// * `slave_ip` - IP address of the slave device to remove
    /// * `slave_mac` - MAC address of the slave device to remove
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    /// client.remove_zone_slave(
    ///     "192.168.1.144",
    ///     "00:11:22:33:44:55"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_zone_slave(&self, slave_ip: &str, slave_mac: &str) -> Result<()> {
        let url = format!("http://{}:8090/removeZoneSlave", &self.hostname);

        // Get current zone to get master info
        let current_zone = self.get_zone().await?;

        // Create member to remove
        let member = ZoneMember {
            ip_address: slave_ip.to_string(),
            mac_address: slave_mac.to_string(),
        };

        // Create zone with just the member to remove
        let zone = Zone {
            master: current_zone.master,
            sender_ip_address: None,
            members: vec![member],
        };

        post_xml(&url, &zone).await
    }

    /// Checks if this device is part of a multi-room zone
    ///
    /// # Returns
    /// `true` if the device is either a master or slave in a zone, `false` otherwise
    pub async fn is_in_zone(&self) -> Result<bool> {
        let zone = self.get_zone().await?;
        Ok(zone.members.len() > 1)
    }

    /// Checks if this device is the master of a multi-room zone
    ///
    /// # Returns
    /// `true` if the device is the master of a zone, `false` otherwise
    pub async fn is_zone_master(&self) -> Result<bool> {
        let zone = self.get_zone().await?;
        let info = self.get_info().await?;
        Ok(zone.master == info.device_id && zone.members.len() > 1)
    }

    /// Gets the bass capabilities of the device
    ///
    /// Returns information about the supported bass range and default value.
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    /// let caps = client.get_bass_capabilities().await?;
    /// println!("Bass range: {} to {} (default: {})",
    ///     caps.min_value, caps.max_value, caps.default);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bass_capabilities(&self) -> Result<BassCapabilities> {
        let url = format!("http://{}:8090/bassCapabilities", &self.hostname);
        get_xml(url).await
    }

    /// Gets the current bass settings
    ///
    /// Returns both the target and actual bass levels.
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    /// let bass = client.get_bass().await?;
    /// println!("Current bass level: {}", bass.actual);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bass(&self) -> Result<Bass> {
        let url = format!("http://{}:8090/bass", &self.hostname);
        get_xml(url).await
    }

    /// Sets the bass level
    ///
    /// # Arguments
    /// * `value` - Bass level to set (use get_bass_capabilities to determine valid range)
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    ///
    /// // Get valid bass range
    /// let caps = client.get_bass_capabilities().await?;
    ///
    /// // Set bass to middle of range
    /// let mid_bass = (caps.max_value + caps.min_value) / 2;
    /// client.set_bass(mid_bass).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_bass(&self, value: i32) -> Result<()> {
        let url = format!("http://{}:8090/bass", &self.hostname);
        post_xml(&url, &SetBass { value }).await
    }

    /// Sets the bass level to the device's default value
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    /// client.reset_bass_to_default().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reset_bass_to_default(&self) -> Result<()> {
        let caps = self.get_bass_capabilities().await?;
        self.set_bass(caps.default).await
    }

    /// Increases the bass level by one step
    ///
    /// Will not exceed the maximum supported bass level.
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    /// client.bass_up().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn bass_up(&self) -> Result<()> {
        let caps = self.get_bass_capabilities().await?;
        let current = self.get_bass().await?;
        if current.actual < caps.max_value {
            self.set_bass(current.actual + 1).await
        } else {
            Ok(())
        }
    }

    /// Decreases the bass level by one step
    ///
    /// Will not go below the minimum supported bass level.
    ///
    /// # Example
    /// ```no_run
    /// # use bose_soundtouch::BoseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BoseClient::new("192.168.1.143");
    /// client.bass_down().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn bass_down(&self) -> Result<()> {
        let caps = self.get_bass_capabilities().await?;
        let current = self.get_bass().await?;
        if current.actual > caps.min_value {
            self.set_bass(current.actual - 1).await
        } else {
            Ok(())
        }
    }

    /// Toggles between play and pause states
    pub async fn play_pause(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::PlayPause).await
    }

    /// Gives thumbs up to current track
    pub async fn thumbs_up(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::ThumbsUp).await
    }

    /// Gives thumbs down to current track
    pub async fn thumbs_down(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::ThumbsDown).await
    }

    /// Bookmarks the current track/station
    pub async fn bookmark(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Bookmark).await
    }

    /// Adds current item to favorites
    pub async fn add_favorite(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::AddFavorite).await
    }

    /// Removes current item from favorites
    pub async fn remove_favorite(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::RemoveFavorite).await
    }
}

/// Remote control key values supported by the SoundTouch API
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum KeyValue {
    /// Play the current media
    Play,
    /// Pause the current media
    Pause,
    /// Stop playback
    Stop,
    /// Skip to previous track
    PrevTrack,
    /// Skip to next track
    NextTrack,
    /// Give thumbs up to current track
    ThumbsUp,
    /// Give thumbs down to current track
    ThumbsDown,
    /// Bookmark the current track/station
    Bookmark,
    /// Toggle power state
    Power,
    /// Toggle mute state
    Mute,
    /// Select preset 1
    #[serde(rename(serialize = "PRESET_1"))]
    Preset1,
    /// Select preset 2
    #[serde(rename(serialize = "PRESET_2"))]
    Preset2,
    /// Select preset 3
    #[serde(rename(serialize = "PRESET_3"))]
    Preset3,
    /// Select preset 4
    #[serde(rename(serialize = "PRESET_4"))]
    Preset4,
    /// Select preset 5
    #[serde(rename(serialize = "PRESET_5"))]
    Preset5,
    /// Select preset 6
    #[serde(rename(serialize = "PRESET_6"))]
    Preset6,
    /// Switch to AUX input
    AuxInput,
    /// Turn shuffle mode off
    ShuffleOff,
    /// Turn shuffle mode on
    ShuffleOn,
    /// Turn repeat mode off
    RepeatOff,
    /// Repeat current track
    RepeatOne,
    /// Repeat all tracks
    RepeatAll,
    /// Toggle between play and pause
    PlayPause,
    /// Add current item to favorites
    AddFavorite,
    /// Remove current item from favorites
    RemoveFavorite,
}

impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
enum KeyState {
    Press,
    Release,
}

impl fmt::Display for KeyState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Current playback information
#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "nowPlaying"))]
pub struct NowPlaying {
    /// Unique device identifier
    #[serde(rename = "@deviceID")]
    pub device_id: String,
    /// Current source (e.g., INTERNET_RADIO, BLUETOOTH, STANDBY)
    #[serde(rename = "@source")]
    pub source: String,
    /// Account associated with the current source
    #[serde(rename = "@sourceAccount")]
    pub source_account: Option<String>,
    /// Details about the current content
    #[serde(rename = "ContentItem")]
    pub content_item: NowPlayingContentItem,
    /// Current track name
    pub track: Option<String>,
    /// Current artist name
    pub artist: Option<String>,
    /// Current album name
    pub album: Option<String>,
    /// Current station name (for radio sources)
    #[serde(rename = "stationName")]
    pub station_name: Option<String>,
    /// Artwork URL and status
    #[serde(rename = "art")]
    pub art: Option<Art>,
    /// Current playback status
    #[serde(rename = "playStatus")]
    pub play_status: Option<PlayStatus>,
    /// Description of current content
    pub description: Option<String>,
    /// Station location (for radio sources)
    #[serde(rename = "stationLocation")]
    pub station_location: Option<String>,
}

/// Content item details for currently playing media
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NowPlayingContentItem {
    /// Source of the content (e.g., INTERNET_RADIO)
    #[serde(rename = "@source")]
    pub source: String,
    /// Type of content
    #[serde(rename = "@type")]
    pub content_type: Option<String>,
    /// Location/URL of the content
    #[serde(rename = "@location")]
    pub location: Option<String>,
    /// Whether this content can be saved as a preset
    #[serde(rename = "@isPresetable")]
    pub is_presetable: bool,
    /// Name of the content item
    #[serde(rename = "itemName")]
    pub name: Option<String>,
    /// URL of the album/station artwork
    #[serde(rename = "containerArt")]
    pub container_art: Option<String>,
}

/// Artwork information
#[derive(Debug, Deserialize)]
pub struct Art {
    /// Status of the artwork
    #[serde(rename = "@artImageStatus")]
    pub status: ArtStatus,
    /// URL of the artwork
    #[serde(rename = "$value")]
    pub url: Option<String>,
}

/// Status of artwork
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArtStatus {
    Invalid,
    ShowDefaultImage,
    Downloading,
    ImagePresent,
}

/// Playback status
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayStatus {
    PlayState,
    PauseState,
    StopState,
    BufferingState,
    InvalidPlayStatus,
}

/// Volume settings for the device
#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "volume"))]
#[allow(dead_code)]
pub struct Volume {
    /// Target volume level (0-100)
    #[serde(rename = "targetvolume")]
    pub target: i32,
    /// Current actual volume level (0-100)
    #[serde(rename = "actualvolume")]
    pub actual: i32,
    /// Whether mute is enabled
    #[serde(rename = "muteenabled")]
    pub mute_enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "volume"))]
struct PostVolume {
    #[serde(rename = "$value")]
    value: i32,
}

impl PostVolume {
    pub fn new(value: i32) -> PostVolume {
        PostVolume { value }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "key"))]
struct PostKey {
    #[serde(rename = "@state")]
    state: KeyState,
    #[serde(rename = "@sender")]
    sender: String,
    #[serde(rename = "$text")]
    value: KeyValue,
}

impl PostKey {
    pub fn press(value: &KeyValue) -> PostKey {
        PostKey {
            state: KeyState::Press,
            sender: "Gabbo".to_string(),
            value: *value,
        }
    }

    pub fn release(value: &KeyValue) -> PostKey {
        PostKey {
            state: KeyState::Release,
            sender: "Gabbo".to_string(),
            value: *value,
        }
    }
}

/// Collection of preset stations/sources
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(deserialize = "nowPlaying"))]
#[allow(dead_code)]
pub struct Presets {
    /// List of preset items
    #[serde(rename = "$value", default)]
    pub items: Vec<Preset>,
}

/// Individual preset station/source
#[derive(Debug, Serialize, Deserialize)]
pub struct Preset {
    /// Preset number (1-6)
    #[serde(rename = "@id")]
    pub id: i32,
    /// Unix timestamp when preset was created
    #[serde(rename = "@createdOn")]
    pub created_on: i32,
    /// Unix timestamp when preset was last updated
    #[serde(rename = "@updatedOn")]
    pub updated_on: i32,
    /// Content details for this preset
    #[serde(rename = "$value")]
    pub content_item: PresetContentItem,
}

/// Content details for a preset
#[derive(Debug, Serialize, Deserialize)]
pub struct PresetContentItem {
    /// Source of the preset content
    #[serde(rename = "@source")]
    pub source: String,
    /// Type of preset content
    #[serde(rename = "@type")]
    pub preset_type: String,
    /// Location/URL of the preset content
    #[serde(rename = "@location")]
    pub location: String,
    /// Account associated with this preset
    #[serde(rename = "@sourceAccount")]
    pub source_account: String,
    /// Whether this content can be saved as a preset
    #[serde(rename = "@isPresetable")]
    pub is_presetable: bool,
    /// Name of the preset content
    #[serde(rename = "itemName")]
    pub name: String,
    /// URL of the preset artwork
    #[serde(rename = "containerArt")]
    pub container_art: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PresetContentItemValue {
    //#[serde(rename = "itemName")]
    //item_name: String
}

fn serialize_xml<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    quick_xml::se::to_string(value).map_err(BoseClientError::XmlError)
}

async fn post_xml<U: IntoUrl + Debug + Clone, T: ?Sized + Serialize + Debug>(
    url: U,
    data: &T,
) -> Result<()> {
    let client = Client::new();
    let body = serialize_xml(data)?;
    client
        .post(url.clone())
        .body(body.clone())
        .send()
        .await
        .map_err(BoseClientError::HttpClientError)?;
    Ok(())
}

async fn get_xml<U: IntoUrl + Debug + Clone, T: DeserializeOwned>(url: U) -> Result<T> {
    let client = Client::new();
    let response = client
        .get(url.clone())
        .send()
        .await
        .map_err(BoseClientError::HttpClientError)?;
    let body = response.text().await?;
    tracing::debug!("Response from {}: {}", url.as_str(), body);
    let value: T = quick_xml::de::from_str(&body).map_err(BoseClientError::XmlError)?;
    Ok(value)
}

/// Information about the device
#[derive(Debug, Deserialize)]
#[serde(rename = "info")]
pub struct DeviceInfo {
    /// Device ID (MAC address)
    #[serde(rename = "@deviceID")]
    pub device_id: String,
    /// Device name
    pub name: String,
    /// Device type
    #[serde(rename = "type")]
    pub device_type: String,
    /// Marge account UUID
    #[serde(rename = "margeAccountUUID")]
    pub marge_account_uuid: String,
    /// Component version information
    pub components: Components,
    /// URL for device management
    #[serde(rename = "margeURL")]
    pub marge_url: String,
    /// Network information (multiple entries)
    #[serde(rename = "networkInfo")]
    pub network_info: Vec<NetworkInfo>,
    /// Module type
    #[serde(rename = "moduleType")]
    pub module_type: String,
    /// Device variant
    pub variant: String,
    /// Variant mode
    #[serde(rename = "variantMode")]
    pub variant_mode: String,
    /// Country code
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// Region code
    #[serde(rename = "regionCode")]
    pub region_code: String,
}

/// Component version information
#[derive(Debug, Deserialize)]
pub struct Components {
    /// List of components
    pub component: Vec<Component>,
}

/// Individual component information
#[derive(Debug, Deserialize)]
pub struct Component {
    /// Component category
    #[serde(rename = "componentCategory")]
    pub category: String,
    /// Software version (optional)
    #[serde(rename = "softwareVersion")]
    pub software_version: Option<String>,
    /// Serial number (optional)
    #[serde(rename = "serialNumber")]
    pub serial_number: Option<String>,
}

/// Network information for the device
#[derive(Debug, Deserialize)]
pub struct NetworkInfo {
    /// Network type (SCM or SMSC)
    #[serde(rename = "@type")]
    pub network_type: String,
    /// MAC address
    #[serde(rename = "macAddress")]
    pub mac_address: String,
    /// IP address
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "name"))]
struct DeviceName {
    #[serde(rename = "$text")]
    name: String,
}

/// Available sources for the SoundTouch device
#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "sources"))]
pub struct Sources {
    /// List of available sources
    #[serde(rename = "sourceItem")]
    pub items: Vec<SourceItem>,
}

/// Individual source item
#[derive(Debug, Deserialize)]
pub struct SourceItem {
    /// Source type (e.g., INTERNET_RADIO, BLUETOOTH, AUX)
    #[serde(rename = "@source")]
    pub source: String,
    /// Account associated with the source
    #[serde(rename = "@sourceAccount")]
    pub source_account: Option<String>,
    /// Status of the source (UNAVAILABLE, READY)
    #[serde(rename = "@status")]
    pub status: SourceStatus,
    /// Display name of the source
    #[serde(rename = "$value")]
    pub name: String,
}

/// Status of a source
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SourceStatus {
    /// Source is not available
    Unavailable,
    /// Source is ready to use
    Ready,
}

/// Content item for selecting a source
#[derive(Debug, Serialize)]
struct SelectSource {
    /// Source type (e.g., INTERNET_RADIO, BLUETOOTH, AUX)
    #[serde(rename = "@source")]
    source: String,
    /// Account associated with the source (if any)
    #[serde(rename = "@sourceAccount")]
    source_account: Option<String>,
}

/// Zone configuration for multi-room audio
#[derive(Debug, Deserialize, Serialize)]
pub struct Zone {
    /// MAC address of the master device
    #[serde(rename = "@master")]
    pub master: String,
    /// IP address of the sender (only used when setting zones)
    #[serde(rename = "@senderIPAddress", skip_serializing_if = "Option::is_none")]
    pub sender_ip_address: Option<String>,
    /// List of zone members (master and slaves)
    #[serde(rename = "member")]
    pub members: Vec<ZoneMember>,
}

/// Member device in a multi-room zone
#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneMember {
    /// IP address of the device
    #[serde(rename = "@ipaddress")]
    pub ip_address: String,
    /// MAC address of the device
    #[serde(rename = "$value")]
    pub mac_address: String,
}

/// Bass capabilities of the device
#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "bassCapabilities"))]
pub struct BassCapabilities {
    /// Minimum bass level supported
    #[serde(rename = "@minValue")]
    pub min_value: i32,
    /// Maximum bass level supported
    #[serde(rename = "@maxValue")]
    pub max_value: i32,
    /// Default bass level
    #[serde(rename = "@default")]
    pub default: i32,
}

/// Bass settings for the device
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(deserialize = "bass"))]
pub struct Bass {
    /// Current bass level
    #[serde(rename = "targetbass")]
    pub target: i32,
    /// Actual bass level
    #[serde(rename = "actualbass")]
    pub actual: i32,
}

/// Request to set bass level
#[derive(Debug, Serialize)]
#[serde(rename(serialize = "bass"))]
struct SetBass {
    #[serde(rename = "$value")]
    value: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_serializer() {
        assert_eq!(
            "<key state=\"press\" sender=\"Gabbo\">POWER</key>".to_string(),
            serialize_xml(&PostKey::press(&KeyValue::Power)).unwrap()
        )
    }

    #[test]
    fn test_preset_key_serializer() {
        assert_eq!(
            "<key state=\"press\" sender=\"Gabbo\">PRESET_1</key>".to_string(),
            serialize_xml(&PostKey::press(&KeyValue::Preset1)).unwrap()
        )
    }

    #[test]
    fn test_volume_serializer() {
        assert_eq!(
            "<volume>9</volume>".to_string(),
            serialize_xml(&PostVolume { value: 9 }).unwrap()
        )
    }
}
