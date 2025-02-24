use bose_soundtouch::*;

const SAMPLE_NOW_PLAYING: &str = include_str!("samples/now_playing.xml");
const SAMPLE_VOLUME: &str = include_str!("samples/volume.xml");
const SAMPLE_CONNECTION: &str = include_str!("samples/connection.xml");

#[test]
fn test_parse_now_playing() {
    let mut client = BoseClient::new("test");
    let _rx = client.subscribe();
    let event = client.parse_event(SAMPLE_NOW_PLAYING).expect("Failed to parse now playing event");
    match event {
        SoundTouchEvent::NowPlayingUpdated(update) => {
            assert_eq!(update.now_playing.source, Source::Tunein);
            assert_eq!(update.now_playing.artist.as_deref(), Some("Teddy Swims - Bad Dreams"));
            assert_eq!(update.now_playing.track.as_deref(), Some("Qmusic België"));
        }
        _ => panic!("Expected NowPlayingUpdated event"),
    }
}

#[test]
fn test_parse_volume() {
    let mut client = BoseClient::new("test");
    let _rx = client.subscribe();
    let event = client.parse_event(SAMPLE_VOLUME).expect("Failed to parse volume event");
    match event {
        SoundTouchEvent::VolumeUpdated(update) => {
            assert_eq!(update.volume.target_volume, 5);
            assert_eq!(update.volume.actual_volume, 5);
            assert_eq!(update.volume.mute_enabled, false);
        }
        _ => panic!("Expected VolumeUpdated event"),
    }
}

#[test]
fn test_parse_connection() {
    let mut client = BoseClient::new("test");
    let _rx = client.subscribe();
    let event = client.parse_event(SAMPLE_CONNECTION).expect("Failed to parse connection event");
    match event {
        SoundTouchEvent::ConnectionStateUpdated(state) => {
            assert_eq!(state.state, "NETWORK_WIFI_CONNECTED");
            assert_eq!(state.up, true);
            assert_eq!(state.signal, "GOOD_SIGNAL");
        }
        _ => panic!("Expected ConnectionStateUpdated event"),
    }
} 