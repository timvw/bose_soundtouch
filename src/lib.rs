use thiserror::Error;
//use quick_xml::DeError;
use reqwest::{Client, IntoUrl};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;

pub struct BoseClient {
    hostname: String,
}

#[derive(Error, Debug)]
pub enum BoseClientError {
    #[error("Invalid Preset")]
    InvalidPreset(String),
    #[error("Failed to (de)serialize from XML")]
    XmlError(#[from] quick_xml::DeError),
    #[error("Http client issue")]
    HttpClientError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, BoseClientError>;

impl BoseClient {
    pub fn new(hostname: &str) -> BoseClient {
        BoseClient {
            hostname: String::from(hostname),
        }
    }

    pub async fn play(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Play).await
    }

    pub async fn pause(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Pause).await
    }

    pub async fn power(&self) -> Result<()> {
        self.press_and_release_key(&KeyValue::Power).await
    }

    pub async fn press_and_release_key(&self, key_value: &KeyValue) -> Result<()> {
        let url = format!("http://{}:8090/key", &self.hostname);
        let _ = post_xml(&url, &PostKey::press(key_value)).await?;
        let _ = post_xml(&url, &PostKey::release(key_value)).await?;
        Ok(())
    }

    pub async fn get_status(&self) -> Result<NowPlaying> {
        let url = format!("http://{}:8090/now_playing", &self.hostname);
        get_xml(url).await
    }

    pub async fn get_volume(&self) -> Result<Volume> {
        let url = format!("http://{}:8090/volume", &self.hostname);
        get_xml(url).await
    }

    pub async fn set_volume(&self, value: i32) -> Result<()> {
        let url = format!("http://{}:8090/volume", &self.hostname);
        let _ = post_xml(&url, &PostVolume::new(value)).await?;
        Ok(())
    }

    pub async fn get_presets(&self) -> Result<Presets> {
        let url = format!("http://{}:8090/presets", &self.hostname);
        get_xml(url).await
    }

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
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum KeyValue {
    Play,
    Pause,
    //STOP,
    //PREV_TRACK,
    //NEXT_TRACK,
    //THUMBS_UP,
    //THUMBS_DOWN,
    //BOOKMARK,
    Power,
    //MUTE,
    //VOLUME_UP,
    //VOLUME_DOWN,
    #[serde(rename(serialize = "PRESET_1"))]
    Preset1,
    #[serde(rename(serialize = "PRESET_2"))]
    Preset2,
    #[serde(rename(serialize = "PRESET_3"))]
    Preset3,
    #[serde(rename(serialize = "PRESET_4"))]
    Preset4,
    #[serde(rename(serialize = "PRESET_5"))]
    Preset5,
    #[serde(rename(serialize = "PRESET_6"))]
    Preset6,
    //AUX_INPUT,
    //SHUFFLE_OFF,
    //SHUFFLE_ON,
    //REPEAT_OFF,
    //REPEAT_ONE,
    //REPEAT_ALL,
    //PLAY_PAUSE,
    //ADD_FAVORITE,
    //REMOVE_FAVORITE,
    //INVALID_KEY,
}

impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
enum KeyState {
    PRESS,
    RELEASE,
}

impl fmt::Display for KeyState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "nowPlaying"))]
#[allow(dead_code)]
pub struct NowPlaying {
    #[serde(rename = "@deviceID")]
    pub device_id: String,
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@sourceAccount")]
    pub source_account: Option<String>,
    #[serde(rename = "ContentItem")]
    pub content_item: NowPlayingContentItem,
    pub track: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    #[serde(rename = "stationName")]
    pub station_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NowPlayingContentItem {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@type")]
    pub content_type: Option<String>,
    #[serde(rename = "@location")]
    pub location: Option<String>,
    #[serde(rename = "@isPresetable")]
    pub is_presetable: bool,
    #[serde(rename = "itemName")]
    pub name: Option<String>,
    #[serde(rename = "containerArt")]
    pub container_art: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "volume"))]
#[allow(dead_code)]
pub struct Volume {
    #[serde(rename = "targetvolume")]
    pub target: i32,
    #[serde(rename = "actualvolume")]
    pub actual: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename(serialize = "volume"))]
struct PostVolume {
    #[serde(rename = "$value")]
    value: i32,
}

impl PostVolume {
    pub fn new(value: i32) -> PostVolume {
        PostVolume { value: value }
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
            state: KeyState::PRESS,
            sender: "Gabbo".to_string(),
            value: value.clone(),
        }
    }

    pub fn release(value: &KeyValue) -> PostKey {
        PostKey {
            state: KeyState::RELEASE,
            sender: "Gabbo".to_string(),
            value: value.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename(deserialize = "nowPlaying"))]
#[allow(dead_code)]
pub struct Presets {
    #[serde(rename = "$value", default)]
    pub items: Vec<Preset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preset {
    #[serde(rename = "@id")]
    pub id: i32,
    #[serde(rename = "@createdOn")]
    pub created_on: i32,
    #[serde(rename = "@updatedOn")]
    pub updated_on: i32,
    #[serde(rename = "$value")]
    pub content_item: PresetContentItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PresetContentItem {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@type")]
    pub preset_type: String,
    #[serde(rename = "@location")]
    pub location: String,
    #[serde(rename = "@sourceAccount")]
    pub source_account: String,
    #[serde(rename = "@isPresetable")]
    pub is_presetable: bool,
    #[serde(rename = "itemName")]
    pub name: String,
    #[serde(rename = "containerArt")]
    pub container_art: String,
    //#[serde(rename = "$value")]
    //<itemName>RGR</itemName><containerArt>http://cdn-profiles.tunein.com/s214611/images/logoq.jpg?t=152269</containerArt>
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
    quick_xml::se::to_string(value).map_err(|e| BoseClientError::XmlError(e))
}

async fn post_xml<U: IntoUrl + Debug + Clone, T: ?Sized + Serialize + Debug>(
    url: U,
    data: &T,
) -> Result<()> {
    let client = Client::new();
    let body = serialize_xml(data)?;
    let _ = client
        .post(url.clone())
        .body(body.clone())
        .send()
        .await
        .map_err(|e| BoseClientError::HttpClientError(e))?;
    Ok(())
}

async fn get_xml<U: IntoUrl + Debug + Clone, T: DeserializeOwned>(url: U) -> Result<T> {
    let client = Client::new();
    let response = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| BoseClientError::HttpClientError(e))?;
    let body = response.text().await?;
    //println!("received body: {}", &body);
    let value: T = quick_xml::de::from_str(&body).map_err(|e| BoseClientError::XmlError(e))?;
    Ok(value)
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
