//! Provides an API to get the list of supported services on the server.

use crate::{
    error::{Error, Result},
    Bravia, RequestBodyBuilder, RequestBuilder,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::cmp::Ordering;

const ENDPOINT: &str = "guide";

/// Detail of supported versions of the API.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versions {
    /// Version of the API.
    pub version: String,
    /// Transport for the API,
    /// if there are any exception from that of belonging service.
    pub protocols: Option<Vec<String>>,
    /// Authentication level of the API.
    pub auth_level: Option<String>,
}

impl Ord for Versions {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl PartialOrd for Versions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Supported APIs.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Api {
    /// Name of the API.
    pub name: String,
    pub versions: Vec<Versions>,
}

impl Ord for Api {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_max = self.versions.iter().max();
        let other_max = other.versions.iter().max();
        self_max.cmp(&other_max)
    }
}

impl PartialOrd for Api {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Supported Notification APIs.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Notifications {
    /// Name of the API.
    pub name: String,
    pub versions: Vec<Versions>,
}

impl Ord for Notifications {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_max = self.versions.iter().max();
        let other_max = other.versions.iter().max();
        self_max.cmp(&other_max)
    }
}

impl PartialOrd for Notifications {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Data related to the service.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceData {
    /// Supported APIs.
    pub apis: Vec<Api>,
    pub notifications: Option<Vec<Notifications>>,
    /// Name of the service.
    pub service: String,
    /// Supported transports.
    pub protocols: Vec<String>,
}

/// Provides access to guide service APIs.
pub struct GuideService<'a>(&'a Bravia);

impl<'a> GuideService<'a> {
    pub fn new(bravia: &'a Bravia) -> Self {
        Self(bravia)
    }

    /// This API provides the supported services and their information.
    /// This API is used in the initialization sequence to dynamically
    /// fetch the service compatibility of the server.
    ///
    /// # Arguments
    /// * `services` - Services to fetch API information.\
    /// None or empty vectors are treated as all services.
    ///
    /// # Authentication Level
    /// None
    ///
    /// # Examples
    /// ```no_run
    /// # use bravia_api::{error::{Error, Result}, Bravia};
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let bravia = Bravia::new("ADDRESS", Some("PASSWORD")).await?;
    /// let service = bravia.guide().get_supported_api_info(Some(vec!["avContent".to_string()])).await?;
    /// let api_name = "getCurrentExternalInputsStatus";
    /// let api = service.get(0).ok_or(Error::MissingValue("getCurrentExternalInputsStatus service data"))?
    ///     .apis.iter().find(|&x| x.name == api_name);
    /// if let Some(api) = api {
    ///     if let Some(max) = api.versions.iter().max() {
    ///         println!("Max {} version: {}", api_name, max.version);
    ///     }
    /// }
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get_supported_api_info(
        &self,
        services: Option<Vec<String>>,
    ) -> Result<Vec<ServiceData>> {
        let mut params = Map::new();
        if let Some(services) = services {
            params.insert(String::from("services"), Value::from(services));
        }

        let body = RequestBodyBuilder::default()
            .id(5)
            .method("getSupportedApiInfo")
            .params(Value::from(params))
            .build()?;

        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .make(self.0)
            .await?;

        let parsed: Vec<ServiceData> = serde_json::from_value(req)?;
        if parsed.is_empty() {
            Err(Error::MissingValue("apis"))
        } else {
            Ok(parsed)
        }
    }
}
