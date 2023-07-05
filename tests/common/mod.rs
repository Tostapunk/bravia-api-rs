use wiremock::{MockServer, Mock, matchers::{method, path, body_string_contains, BodyExactMatcher}, ResponseTemplate};
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

pub async fn server_setup(json_base_path: &str) -> MockServer {
    let file = File::open(format!("{}/supported_api_info.json", json_base_path)).unwrap();
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).unwrap();
    let template = ResponseTemplate::new(200).set_body_json(json);
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/sony/guide"))
        .and(body_string_contains("getSupportedApiInfo"))
        .respond_with(template)
        .named("getSupportedApiInfo POST")
        .mount(&mock_server)
        .await;
   mock_server 
}

pub trait FromFile {
    fn from_json_file(json_path: &str) -> Self;
}

impl FromFile for ResponseTemplate {
    fn from_json_file(json_path: &str) -> Self {
        let file = File::open(json_path).unwrap();
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader).unwrap();
        ResponseTemplate::new(200).set_body_json(json)    
    }
}

impl FromFile for BodyExactMatcher {
    fn from_json_file(json_path: &str) -> Self {
        let file = File::open(json_path).unwrap();
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader).unwrap();
        Self::json(json)
    }
}
