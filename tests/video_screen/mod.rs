use crate::common::{server_setup, FromFile};
use bravia_api::Bravia;
use wiremock::{
    matchers::{method, path, BodyExactMatcher},
    Mock, ResponseTemplate,
};

const ENDPOINT_PATH: &str = "/sony/videoScreen";
const JSON_BASE_PATH: &str = "sample_payloads/video_screen";
const AUTH: Option<&str> = Some("TEST");

#[tokio::test]
async fn test_set_scene_settings() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_scene_settings.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_scene_settings.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setSceneSetting POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia
        .video_screen()
        .set_scene_settings("auto".to_string())
        .await
        .unwrap();

    // Nothing to assert, this API returns ()
}
