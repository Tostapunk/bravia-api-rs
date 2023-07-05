use crate::common::{server_setup, FromFile};
use bravia_api::{
    audio::{SoundSettings, SpeakerSettings},
    Bravia,
};
use wiremock::{
    matchers::{method, path, BodyExactMatcher},
    Mock, ResponseTemplate,
};

const ENDPOINT_PATH: &str = "/sony/audio";
const JSON_BASE_PATH: &str = "sample_payloads/audio";
const AUTH: Option<&str> = Some("TEST");

#[tokio::test]
async fn test_get_sound_settings() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_sound_settings.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_sound_settings.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getSoundSettings POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let sound_settings = bravia
        .audio()
        .get_sound_settings(Some("outputTerminal".into()))
        .await
        .unwrap();

    // Assert
    assert_eq!("audioSystem", sound_settings.get(0).unwrap().value);
}

#[tokio::test]
async fn test_get_speaker_settings() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_speaker_settings.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_speaker_settings.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getSpeakerSettings POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let speaker_settings = bravia
        .audio()
        .get_speaker_settings(Some("tvPosition".into()))
        .await
        .unwrap();

    // Assert
    assert_eq!("tableTop", speaker_settings.get(0).unwrap().value);
}

#[tokio::test]
async fn test_get_volume_information() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_volume_information.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_volume_information.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getVolumeInformation POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let volume_information = bravia.audio().get_volume_information().await.unwrap();

    // Assert
    assert_eq!(25, volume_information.get(0).unwrap().volume);
}

#[tokio::test]
async fn test_set_audio_mute() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_audio_mute.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_audio_mute.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setAudioMute POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia.audio().set_audio_mute(true).await.unwrap();

    // Nothing to assert
}

#[tokio::test]
async fn test_set_audio_volume() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_audio_volume_V1_2.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_audio_volume_V1_2.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setAudioVolume POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia
        .audio()
        .set_audio_volume(
            Some("speaker".to_string()),
            "5".to_string(),
            Some("on".to_string()),
            Some("1.2"),
        )
        .await
        .unwrap();

    // Nothing to assert
}

#[tokio::test]
async fn test_set_sound_settings() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_sound_settings.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_sound_settings.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setSoundSettings POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let settings = SoundSettings::new("outputTerminal".into(), "speaker".into());
    bravia
        .audio()
        .set_sound_settings(vec![settings])
        .await
        .unwrap();

    // Nothing to assert
}

#[tokio::test]
async fn test_set_speaker_settings() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_speaker_settings.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_speaker_settings.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setSpeakerSettings POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let settings = SpeakerSettings::new("tvPosition".into(), "wallMount".into());
    bravia
        .audio()
        .set_speaker_settings(vec![settings])
        .await
        .unwrap();

    // Nothing to assert
}
