use crate::common::{server_setup, FromFile};
use bravia_api::Bravia;
use wiremock::{
    matchers::{method, path, BodyExactMatcher},
    Mock, ResponseTemplate,
};

const ENDPOINT_PATH: &str = "/sony/encryption";
const JSON_BASE_PATH: &str = "sample_payloads/encryption";
const AUTH: Option<&str> = Some("TEST");

#[tokio::test]
async fn test_get_public_key() {
    // Arrange
    let mock_server = server_setup(JSON_BASE_PATH).await;
    let expected_body = BodyExactMatcher::from_json_file(&format!(
        "{}/requests/get_public_key.json",
        JSON_BASE_PATH
    ));
    let template = ResponseTemplate::from_json_file(&format!(
        "{}/responses/get_public_key.json",
        JSON_BASE_PATH
    ));
    Mock::given(method("POST"))
        .and(path(ENDPOINT_PATH))
        .and(expected_body)
        .respond_with(template)
        .named("getPublicKey POST")
        .mount(&mock_server)
        .await;
    let bravia = Bravia::new(&mock_server.uri(), AUTH).await.unwrap();

    // Act
    let public_key = bravia.encryption().get_public_key().await.unwrap();

    // Assert
    let sample_key = "AAAAB3NzaC1yc2EAAAABIwAAAQEA3p6TmGYDRtnnmzckD5leg7lHIUY9cuV6vFvacew1uZ7Bmx2MF9a7SqmtiLDkLS3P9y9eobRjuWriSfgmqDPRFRU2mdwAmRm2aIvYa6WkzvnrfUhGR+SCT/Z62j7V9ps6Mt5HB8mFQj3494p4StTPVS1nFqvEUazEx13EJnJyHsdYqsV6UJV169e43oLSSccb3lr8BzeMUnGEfY+NKlAxDpEycr5jJYyTkLfrbX0lyAPs+vLwLRYhm+h2qJYAZUwknus4vD7aki4G69S+gnENClglh/e9ut9Q5BrtxiBQCEikn9V9rlnVkbp1eEUf89XFiHRWMVrRAINtJyQFvvoPOQ==";
    assert_eq!(sample_key, public_key);
}
