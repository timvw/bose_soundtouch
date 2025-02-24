use bose_soundtouch::*;

#[test]
fn test_key_serializer() {
    let key = PostKey::press(&KeyValue::Stop);
    let xml = quick_xml::se::to_string(&key).unwrap();
    assert_eq!(xml, r#"<key state="press" sender="Gabbo">STOP</key>"#);
}

#[test]
fn test_preset_key_serializer() {
    let key = PostKey::press(&KeyValue::Preset1);
    let xml = quick_xml::se::to_string(&key).unwrap();
    assert_eq!(xml, r#"<key state="press" sender="Gabbo">PRESET_1</key>"#);
}

#[test]
fn test_volume_serializer() {
    let volume = PostVolume::new(50);
    let xml = quick_xml::se::to_string(&volume).unwrap();
    assert_eq!(xml, r#"<volume>50</volume>"#);
} 