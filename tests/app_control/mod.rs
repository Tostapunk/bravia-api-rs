use crate::common::{server_setup, FromFile};
use bravia_api::{
    app_control::{Application, ApplicationStatus, WebAppStatus},
    Bravia,
};
use wiremock::{
    matchers::{headers, method, path, BodyExactMatcher},
    Mock, ResponseTemplate,
};

const ENDPOINT_PATH: &str = "/sony/appControl";
const JSON_BASE_PATH: &str = "sample_payloads/app_control";
const AUTH: Option<&str> = Some("TEST");

#[tokio::test]
async fn test_get_application_list() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_application_list.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_application_list.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(headers("X-Auth-PSK", vec!["TEST"]))
        .and(expected_body)
        .respond_with(template)
        .named("getApplicationList POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let app_list = bravia.app_control().get_application_list().await.unwrap();

    // Assert
    let yt = Application {
        title: "YouTube".to_string(),
        uri: "com.sony.dtv.com.google.android.youtube.tv.com.google.android.apps.youtube.tv.activity.ShellActivity".to_string(),
        icon: "http://43.3.149.111/DIAL/icon/com.sony.dtv.com.google.android.youtube.tv.com.google.android.apps.youtube.tv.activity.ShellActivity.png".to_string(),
    };
    assert_eq!(&yt, app_list.get(0).unwrap());
}

#[tokio::test]
async fn test_get_application_status_list() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_application_status_list.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_application_status_list.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getApplicationStatusList POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let app_status_list = bravia
        .app_control()
        .get_application_status_list()
        .await
        .unwrap();

    // Assert
    let cursor_display = ApplicationStatus {
        name: "cursorDisplay".to_string(),
        status: "off".to_string(),
    };
    assert_eq!(&cursor_display, app_status_list.get(1).unwrap());
}

#[tokio::test]
async fn test_get_text_form() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_text_form.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_text_form.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getTextForm POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let text_form = bravia.app_control().get_text_form(None).await.unwrap();

    // Assert
    assert_eq!("hello world!!", text_form);
}

#[tokio::test]
async fn test_get_web_app_status() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_web_app_status.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_web_app_status.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getWebAppStatus POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let web_app_status = bravia.app_control().get_web_app_status().await.unwrap();

    // Assert
    let example_app = WebAppStatus {
        active: true,
        url: "http://example.com/".to_string(),
    };
    assert_eq!(example_app, web_app_status);
}

#[tokio::test]
async fn test_set_active_app() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_active_app.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_active_app.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setActiveApp POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia
        .app_control()
        .set_active_app("localapp://webappruntime?url=http%3A%2F%2Fexample.com%2F".to_string())
        .await
        .unwrap();

    // Nothing to assert, this API returns ()
}

#[tokio::test]
async fn test_set_text_form() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_text_form_V1_0.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_text_form_V1_0.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setTextForm POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia
        .app_control()
        .set_text_form("hello world!!".to_string(), None, None)
        .await
        .unwrap();

    // Nothing to assert, this API returns ()
}

#[tokio::test]
async fn test_terminate_apps() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/terminate_apps.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/terminate_apps.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("terminateApps POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia.app_control().terminate_apps().await.unwrap();

    // Nothing to assert, this API returns ()
}
