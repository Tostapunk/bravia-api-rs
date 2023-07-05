use crate::common::{server_setup, FromFile};
use bravia_api::{av_content::ExternalInputStatus, Bravia};
use wiremock::{
    matchers::{method, path, BodyExactMatcher},
    Mock, ResponseTemplate,
};

const ENDPOINT_PATH: &str = "/sony/avContent";
const JSON_BASE_PATH: &str = "sample_payloads/av_content";
const AUTH: Option<&str> = Some("TEST");

#[tokio::test]
async fn test_content_count() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_content_count_V1_0.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_content_count_V1_0.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getContentCount POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let hdmi_count = bravia
        .av_content()
        .get_content_count("extInput:hdmi".to_string(), None, None, None)
        .await
        .unwrap();

    // Assert
    assert_eq!(4, hdmi_count);
}

#[tokio::test]
async fn test_get_content_list() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_content_list.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_content_list.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getContentList POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let hdmi_list = bravia
        .av_content()
        .get_content_list(Some("extInput:hdmi".to_string()), Some(0), Some(50))
        .await
        .unwrap();

    // Assert
    assert_eq!(
        "HDMI 3/ARC",
        hdmi_list.get(2).unwrap().title.as_ref().unwrap()
    );
    assert_ne! {3, hdmi_list.get(2).unwrap().index};
}

#[tokio::test]
async fn test_get_current_external_input_status() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_current_external_input_status_V1_0.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_current_external_input_status_V1_0.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getCurrentExternalInputsStatus POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let external_inputs_status = bravia
        .av_content()
        .get_current_external_input_status(None)
        .await
        .unwrap();

    // Assert
    let hdmi = ExternalInputStatus {
        icon: "meta:hdmi".to_string(),
        connection: true,
        label: "".to_string(),
        title: "HDMI1".to_string(),
        uri: "extInput:hdmi?port=1".to_string(),
        status: None,
    };

    assert_eq!(&hdmi, external_inputs_status.get(2).unwrap());
}

#[tokio::test]
async fn test_get_scheme_list() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_scheme_list.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_scheme_list.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getSchemeList POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let scheme_list = bravia.av_content().get_scheme_list().await.unwrap();

    // Assert
    assert_eq!(vec!["extInput", "fav"], scheme_list);
}

#[tokio::test]
async fn test_get_source_list() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_source_list.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_source_list.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getSourceList POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let source_list = bravia
        .av_content()
        .get_source_list("extInput".to_string())
        .await
        .unwrap();

    // Assert
    assert_eq!(
        vec![
            "extInput:hdmi",
            "extInput:component",
            "extInput:cec",
            "extInput:widi"
        ],
        source_list
    );
}

#[tokio::test]
async fn test_get_playing_content_info() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_playing_content_info.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_playing_content_info.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getPlayingContentInfo POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let playing_content = bravia
        .av_content()
        .get_playing_content_info()
        .await
        .unwrap();

    // Assert
    assert_eq!("HDMI 2", playing_content.title);
}

#[tokio::test]
async fn test_set_play_content() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_play_content.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_play_content.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setPlayContent POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia
        .av_content()
        .set_play_content("extInput:hdmi?port=2".to_string())
        .await
        .unwrap();

    // Nothing to assert, this API returns ()
}
