use crate::common::{server_setup, FromFile};
use bravia_api::{
    system::{InterfaceInfo, LEDIndicatorStatus, NetworkSettings, RemoteControllerAction},
    Bravia,
};
use wiremock::{
    matchers::{method, path, BodyExactMatcher},
    Mock, ResponseTemplate,
};

const ENDPOINT_PATH: &str = "/sony/system";
const JSON_BASE_PATH: &str = "sample_payloads/system";
const AUTH: Option<&str> = Some("TEST");

#[tokio::test]
async fn test_get_current_time() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body_1_0 = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_current_time_V1_0.json",
        JSON_BASE_PATH
    ));
    let expected_body_1_1 = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_current_time_V1_1.json",
        JSON_BASE_PATH
    ));
    let template_1_0 = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_current_time_V1_0.json",
        JSON_BASE_PATH
    ));
    let template_1_1 = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_current_time_V1_1.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body_1_0)
        .respond_with(template_1_0)
        .named("getCurrentTime POST")
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body_1_1)
        .respond_with(template_1_1)
        .named("getCurrentTime POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let time_1_0 = bravia.system().get_current_time(None).await.unwrap();
    let time_1_1 = bravia.system().get_current_time(Some("1.1")).await.unwrap();

    // Assert
    assert_eq!("2018-10-03T13:03:04+0100", time_1_0.date_time);
    assert_eq!("2018-10-03T13:03:59+0100", time_1_1.date_time);
}

#[tokio::test]
async fn test_get_interface_information() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_interface_information.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_interface_information.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getInterfaceInformation POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let interface_info = bravia.system().get_interface_information().await.unwrap();

    // Assert
    let info = InterfaceInfo {
        product_category: "tv".to_string(),
        model_name: "FW-55BZ35F".to_string(),
        product_name: "BRAVIA".to_string(),
        server_name: "".to_string(),
        interface_version: "5.0.1".to_string(),
    };
    assert_eq!(info, interface_info);
}

#[tokio::test]
async fn test_get_led_indicator_status() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_led_indicator_status.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_led_indicator_status.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getLEDIndicatorStatus POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let led_status = bravia.system().get_led_indicator_status().await.unwrap();

    // Assert
    let status = LEDIndicatorStatus {
        mode: "Demo".to_string(),
        status: Some("true".to_string()),
    };
    assert_eq!(status, led_status);
}

#[tokio::test]
async fn test_get_network_settings() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_network_settings.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_network_settings.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getNetworkSettings POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let net_status = bravia
        .system()
        .get_network_settings(Some("eth0".to_string()))
        .await
        .unwrap();

    // Assert
    let eth0 = NetworkSettings {
        netif: "eth0".to_string(),
        hw_addr: "FF-FF-FF-FF-FF-FF".to_string(),
        ip_addr_v4: "0.0.0.0".to_string(),
        ip_addr_v6: "ffff::ffff:ffff:fffd%7".to_string(),
        netmask: "255.255.255.0".to_string(),
        gateway: "0.0.0.0".to_string(),
        dns: vec!["0.0.0.0".to_string(), "1.1.1.1".to_string()],
    };
    let wlan0 = NetworkSettings {
        netif: "wlan0".to_string(),
        hw_addr: "00-00-00-00-00-00".to_string(),
        ip_addr_v4: "0.0.0.1".to_string(),
        ip_addr_v6: "ffff::ffff:ffff:fffd%8".to_string(),
        netmask: "255.255.255.0".to_string(),
        gateway: "0.0.0.0".to_string(),
        dns: vec!["0.0.0.0".to_string(), "fec0:0:0:ffff::1%1".to_string()],
    };
    assert_eq!(vec![eth0, wlan0], net_status);
}

#[tokio::test]
async fn test_get_power_saving_mode() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_power_saving_mode.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_power_saving_mode.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getPowerSavingMode POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let power_saving_mode = bravia.system().get_power_saving_mode().await.unwrap();

    // Assert
    assert_eq!("high", power_saving_mode);
}

#[tokio::test]
async fn test_get_power_status() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_power_status.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_power_status.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getPowerStatus POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let power_status = bravia.system().get_power_status().await.unwrap();

    // Assert
    assert_eq!("standby", power_status);
}

#[tokio::test]
async fn test_get_remote_controller_info() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_remote_controller_info.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_remote_controller_info.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getRemoteControllerInfo POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let controller_info = bravia.system().get_remote_controller_info().await.unwrap();

    // Assert
    let power_off = RemoteControllerAction {
        name: "PowerOff".to_string(),
        value: "AAAAAQAAAAEAAAAvAw==".to_string(),
    };
    assert_eq!(&power_off, controller_info.get(0).unwrap());
}

#[tokio::test]
async fn test_get_remote_device_settings() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_remote_device_settings.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_remote_device_settings.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getRemoteDeviceSettings POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let device_settings = bravia
        .system()
        .get_remote_device_settings(Some("accessPermission".to_string()))
        .await
        .unwrap();

    // Assert
    assert_eq!("accessPermission", device_settings.get(0).unwrap().target);
}

#[tokio::test]
async fn test_get_system_information() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_system_information.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_system_information.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getSystemInformation POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let system_info = bravia.system().get_system_information().await.unwrap();

    // Assert
    assert_eq!("FW-55BZ35F", system_info.model);
}

#[tokio::test]
async fn test_get_system_supported_function() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_system_supported_function.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_system_supported_function.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getSystemSupportedFunction POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let system_fn = bravia
        .system()
        .get_system_supported_function()
        .await
        .unwrap();
    let system_fn = system_fn.get(0).unwrap();

    // Assert
    assert_eq!("WOL", system_fn.option);
    assert_eq!("00:00:00:00:00:00:00:E0", system_fn.value);
}

#[tokio::test]
async fn test_get_wol_mode() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body =
        BodyExactMatcher::from_json_file(&format!("{}/requests/get_wol_mode.json", JSON_BASE_PATH));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_wol_mode.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getWolMode POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let wol_mode = bravia.system().get_wol_mode().await.unwrap();

    // Assert
    assert!(wol_mode);
}

#[tokio::test]
async fn test_request_reboot() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/request_reboot.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/request_reboot.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("requestReboot POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia.system().request_reboot().await.unwrap();

    // Nothing to assert, this API returns ()
}

#[tokio::test]
async fn test_set_led_indicator_status() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_led_indicator_status.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_led_indicator_status.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setLEDIndicatorStatus POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let led_status = LEDIndicatorStatus::new("Demo".to_string(), Some("true".to_string()));
    bravia
        .system()
        .set_led_indicator_status(led_status)
        .await
        .unwrap();

    // Nothing to assert, this API returns ()
}

#[tokio::test]
async fn test_set_language() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body =
        BodyExactMatcher::from_json_file(&format!("{}/requests/set_language.json", JSON_BASE_PATH));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_language.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setLanguage POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia
        .system()
        .set_language("eng".to_string())
        .await
        .unwrap();

    // Nothing to assert, this API returns ()
}

#[tokio::test]
async fn test_set_power_saving_mode() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_power_saving_mode.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_power_saving_mode.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setPowerSavingMode POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia
        .system()
        .set_power_saving_mode("pictureOff".to_string())
        .await
        .unwrap();

    // Nothing to assert, this API returns ()
}

#[tokio::test]
async fn test_set_power_status() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/set_power_status.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_power_status.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setPowerStatus POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia.system().set_power_status(false).await.unwrap();

    // Nothing to assert, this API returns ()
}

#[tokio::test]
async fn test_set_wol_mode() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body =
        BodyExactMatcher::from_json_file(&format!("{}/requests/set_wol_mode.json", JSON_BASE_PATH));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/set_wol_mode.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("setWolMode POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    bravia.system().set_wol_mode(false).await.unwrap();

    // Nothing to assert, this API returns ()
}
