use crate::common::{server_setup, FromFile};
use bravia_api::{video::PictureQualitySettingsRequest, Bravia};
use wiremock::{
    matchers::{method, path, BodyExactMatcher},
    Mock, ResponseTemplate,
};

const ENDPOINT_PATH: &str = "/sony/video";
const JSON_BASE_PATH: &str = "sample_payloads/video";
const AUTH: Option<&str> = Some("TEST");

#[tokio::test]
async fn test_get_picture_quality_settings() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_picture_quality_settings.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_picture_quality_settings.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getPictureQualitySettings POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let quality_settings = bravia
        .video()
        .get_picture_quality_settings(Some("color".to_string()))
        .await
        .unwrap();

    // Assert
    let quality_settings = quality_settings.get(0).unwrap();
    assert_eq!("color", quality_settings.target.as_str());
    assert_eq!("2", quality_settings.current_value.as_str());
}

#[tokio::test]
async fn test_set_picture_quality_settings() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_picture_quality_settings.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_picture_quality_settings.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setPictureQualitySettings POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let settings =
        PictureQualitySettingsRequest::new(Some("color".to_string()), Some("2".to_string()));
    bravia
        .video()
        .set_picture_quality_settings(vec![settings])
        .await
        .unwrap();

    // Nothing to assert, this API returns ()
}
