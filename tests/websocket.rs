#[cfg(feature = "websocket")]
mod tests {
    use bose_soundtouch::*;

    const SAMPLE_NOW_PLAYING: &str = include_str!("samples/now_playing.xml");
    const SAMPLE_VOLUME: &str = include_str!("samples/volume.xml");
    const SAMPLE_CONNECTION: &str = include_str!("samples/connection.xml");
    const SAMPLE_UNKNOWN_SOURCE: &str = r#"<updates deviceID="000C8AB02519">
        <nowPlayingUpdated>
            <nowPlaying deviceID="000C8AB02519" source="NEW_SOURCE" sourceAccount="">
                <ContentItem source="NEW_SOURCE" type="stationurl" location="" sourceAccount="" isPresetable="true">
                    <itemName>Unknown Source</itemName>
                </ContentItem>
                <track>Test Track</track>
                <playStatus>PLAY_STATE</playStatus>
            </nowPlaying>
        </nowPlayingUpdated>
    </updates>"#;

    #[test]
    fn test_parse_now_playing() {
        let mut client = BoseClient::new_from_string("test");
        let _rx = client.subscribe();
        let event = client
            .parse_event(SAMPLE_NOW_PLAYING)
            .expect("Failed to parse now playing event");
        match event {
            SoundTouchEvent::NowPlayingUpdated(update) => {
                assert_eq!(update.now_playing.source, Source::Tunein);
                assert_eq!(
                    update.now_playing.artist.as_deref(),
                    Some("Teddy Swims - Bad Dreams")
                );
                assert_eq!(update.now_playing.track.as_deref(), Some("Qmusic België"));
            }
            _ => panic!("Expected NowPlayingUpdated event"),
        }
    }

    #[test]
    fn test_parse_volume() {
        let mut client = BoseClient::new_from_string("test");
        let _rx = client.subscribe();
        let event = client
            .parse_event(SAMPLE_VOLUME)
            .expect("Failed to parse volume event");
        match event {
            SoundTouchEvent::VolumeUpdated(update) => {
                assert_eq!(update.volume.target_volume, 5);
                assert_eq!(update.volume.actual_volume, 5);
                assert!(!update.volume.mute_enabled);
            }
            _ => panic!("Expected VolumeUpdated event"),
        }
    }

    #[test]
    fn test_parse_connection() {
        let mut client = BoseClient::new_from_string("test");
        let _rx = client.subscribe();
        let event = client
            .parse_event(SAMPLE_CONNECTION)
            .expect("Failed to parse connection event");
        match event {
            SoundTouchEvent::ConnectionStateUpdated(state) => {
                assert_eq!(state.state, ConnectionStateType::NetworkWifiConnected);
                assert!(state.up);
                assert_eq!(state.signal, SignalStrength::GoodSignal);
            }
            _ => panic!("Expected ConnectionStateUpdated event"),
        }
    }

    #[cfg(feature = "unknown-variants")]
    #[test]
    fn test_parse_unknown_values() {
        let mut client = BoseClient::new_from_string("test");
        let _rx = client.subscribe();

        let event = client
            .parse_event(SAMPLE_UNKNOWN_SOURCE)
            .expect("Failed to parse unknown source");
        match event {
            SoundTouchEvent::NowPlayingUpdated(update) => {
                assert!(matches!(update.now_playing.source, Source::Unknown));
                assert!(matches!(
                    update.now_playing.content_item.source,
                    Source::Unknown
                ));
            }
            _ => panic!("Expected NowPlayingUpdated event"),
        }
    }

    #[cfg(not(feature = "unknown-variants"))]
    #[test]
    fn test_reject_unknown_values() {
        let mut client = BoseClient::new_from_string("test");
        let _rx = client.subscribe();

        let result = client.parse_event(SAMPLE_UNKNOWN_SOURCE);
        assert!(
            result.is_err(),
            "Should fail to parse unknown source when unknown-variants feature is disabled"
        );
    }
}
