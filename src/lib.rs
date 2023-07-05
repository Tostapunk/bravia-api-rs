//! Rust wrapper for Sony Bravia APIs.\
//! \
//! This project is **unofficial** and not related in any way to Sony.\
//! This documentation is mainly inspired by the official one.
//!
//! # Usage
//! ```no_run
//! # use bravia_api::{Bravia, error::Result};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let bravia = Bravia::new("ADDRESS", Some("PASSWORD")).await?;
//!
//! // Then you can access the API services and their APIs like this:
//! // bravia.service_name().api_name().await;
//!
//! // For example to use version 1.1 of the getCurrentTime API from the system service:
//! bravia.system().get_current_time(Some("1.1")).await?;
//! #    Ok(())
//! # }
//! ```

#![warn(clippy::all, clippy::unwrap_used)]
#![allow(clippy::missing_errors_doc)]

use app_control::AppControlService;
use audio::AudioService;
use av_content::AvContentService;
use derive_builder::Builder;
use encryption::EncryptionService;
use error::{Error, Result};
use guide::GuideService;
use reqwest::{header::CONTENT_TYPE, Client, StatusCode};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use system::SystemService;
use video::VideoService;
use video_screen::VideoScreenService;

pub mod app_control;
pub mod audio;
pub mod av_content;
pub mod encryption;
pub mod error;
pub mod guide;
pub mod system;
pub mod video;
pub mod video_screen;

type VersionsVec = Vec<String>;
type APIsMap = HashMap<String, VersionsVec>;
type ServicesMap = HashMap<String, APIsMap>;

#[derive(Serialize, Builder, Clone, Default)]
#[builder(build_fn(error = "derive_builder::UninitializedFieldError"))]
struct RequestBody<'a> {
    id: usize,
    method: &'a str,
    #[builder(setter(custom), default = "\"1.0\"")]
    version: &'a str,
    #[builder(setter(custom), default)]
    params: Vec<Value>,
}

impl<'a> RequestBodyBuilder<'a> {
    fn version(&mut self, value: Option<&'a str>) -> &mut Self {
        self.version = if value.is_none() { Some("1.0") } else { value };
        self
    }

    fn params(&mut self, values: Value) -> &mut Self {
        self.params = Some(vec![values]);
        self
    }
}

// Used to get a specific element from the response.
#[derive(Clone)]
enum RequestGetElementType<'a> {
    // Get the element by index.
    Index(usize),
    // Get the element by name from the result vector.
    Text(&'a str),
}

impl<'a> From<&'a str> for RequestGetElementType<'a> {
    fn from(value: &'a str) -> Self {
        Self::Text(value)
    }
}

impl<'a> From<usize> for RequestGetElementType<'a> {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}

#[derive(Builder)]
#[builder(build_fn(error = "derive_builder::UninitializedFieldError"))]
struct Request<'a> {
    endpoint: &'a str,
    body: RequestBody<'a>,
    // Indicates if the request needs authentication.
    #[builder(setter(custom), default)]
    is_protected: bool,
    // Indicates if the request should have a result.
    #[builder(setter(custom), default)]
    has_result: bool,
    #[builder(default = "RequestGetElementType::Index(0)")]
    get: RequestGetElementType<'a>,
}

impl<'a> RequestBuilder<'a> {
    fn is_protected(&mut self) -> &mut Self {
        self.is_protected = Some(true);
        self
    }

    fn has_result(&mut self) -> &mut Self {
        self.has_result = Some(true);
        self
    }

    #[allow(clippy::unwrap_used)]
    async fn make(&mut self, bravia: &Bravia) -> Result<Value> {
        let request = self.build()?;
        Ok(bravia.make_request(request).await.unwrap())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Bravia {
    base_url: String,
    auth: Option<String>,
    api_support: ServicesMap,
}

impl Bravia {
    /// # Arguments
    /// * `address` - Server address.
    /// * `auth` - Server password.\
    /// Only needed when the API authentication level is not `None`.
    pub async fn new(address: &str, auth: Option<&str>) -> Result<Self> {
        let mut ret = Bravia {
            base_url: format!("{address}/sony/"),
            auth: auth.map(str::to_string),
            api_support: HashMap::new(),
        };
        ret.create_supported_api_cache().await?;
        Ok(ret)
    }

    pub fn guide(&self) -> GuideService {
        GuideService::new(self)
    }

    pub fn app_control(&self) -> AppControlService {
        AppControlService::new(self)
    }

    pub fn audio(&self) -> AudioService {
        AudioService::new(self)
    }

    pub fn av_content(&self) -> AvContentService {
        AvContentService::new(self)
    }

    pub fn encryption(&self) -> EncryptionService {
        EncryptionService::new(self)
    }

    pub fn system(&self) -> SystemService {
        SystemService::new(self)
    }

    pub fn video(&self) -> VideoService {
        VideoService::new(self)
    }

    pub fn video_screen(&self) -> VideoScreenService {
        VideoScreenService::new(self)
    }

    // Populates the `api_support` HashMap.
    async fn create_supported_api_cache(&mut self) -> Result<()> {
        let services = self.guide().get_supported_api_info(None).await?;
        for service in services {
            let mut service_apis = HashMap::new();
            for api in service.apis {
                let api_versions: Vec<String> =
                    api.versions.iter().map(|x| x.version.to_string()).collect();
                service_apis.insert(api.name.to_string(), api_versions);
            }
            self.api_support.insert(service.service, service_apis);
        }
        Ok(())
    }

    /// Checks if the API is supported by checking the cached API level.
    fn is_api_supported(&self, service: &str, api: &str, api_level: &str) -> Result<()> {
        if let Some(service) = self.api_support.get(service) {
            if let Some(api) = service.get(api) {
                if api.iter().any(|x| x == api_level) {
                    Ok(())
                } else {
                    Err(Error::BraviaApiLevelError)
                }
            } else {
                Err(Error::BraviaApiNotFound)
            }
        } else {
            Err(Error::BraviaApiServiceNotFound)
        }
    }

    /// Makes the API request and parses the result.
    async fn make_request<'a>(&self, req: Request<'a>) -> Result<Value> {
        let url = format!("{}{}", self.base_url, req.endpoint);

        // Checks if the requested API is supported by the server
        if req.body.method != "getSupportedApiInfo" {
            self.is_api_supported(req.endpoint, req.body.method, req.body.version)?;
        };

        // If no authentication is required it uses an empty &str
        let auth: &str = if req.is_protected {
            if let Some(value) = &self.auth {
                value.as_str()
            } else {
                return Err(Error::BraviaAuthLevelError);
            }
        } else {
            ""
        };

        // Creates and sends the request
        let resp = Client::new()
            .post(url)
            .header("X-Auth-PSK", auth)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&req.body)?)
            .send()
            .await;

        match resp {
            Ok(resp) => {
                match resp.status() {
                    StatusCode::OK => {
                        let mut parsed = resp.json::<serde_json::Value>().await?;
                        if let Some(result) = parsed.get_mut("result") {
                            if req.has_result {
                                let result = match req.get {
                                    RequestGetElementType::Index(value) => result.get_mut(value),
                                    RequestGetElementType::Text(value) => result[0].get_mut(value),
                                };
                                Ok(result.ok_or(Error::MissingValue("result values"))?.take())
                            } else {
                                // Anyway this result will not be used anywhere
                                Ok(serde_json::Value::Null)
                            }
                        } else if let Some(error) = parsed.get_mut("error") {
                            let api_error = error.take();
                            let err = serde_json::from_value(api_error)?;
                            Err(Error::BraviaError(err))
                        } else {
                            Err(Error::InvalidResponse("Missing result and error fields."))
                        }
                    }
                    _ => Err(Error::BadStatus(resp.status())),
                }
            }
            Err(err) => Err(Error::NetworkError(err)),
        }
    }
}
