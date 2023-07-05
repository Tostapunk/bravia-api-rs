//! APIs that launch the application itself and the accompanying manipulations related to specific applications.

use crate::{error::Result, Bravia, RequestBodyBuilder, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

const ENDPOINT: &str = "appControl";

/// Application info.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Application {
    /// Application name.
    pub title: String,
    /// Application URI.
    pub uri: String,
    /// URL that indicates the application's icon location.
    /// If this member is skipped, the default is "", which means that there is no icon for the application.
    #[serde(default)]
    pub icon: String,
}

/// Application status.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApplicationStatus {
    /// Application name.\
    /// The following values are defined:
    /// * `textInput` - software keyboard
    /// * `cursorDisplay` - application using a cursor
    /// * `webBrowse` - web browser
    pub name: String,
    /// Application status.\
    /// The following values are defined:
    /// * `off` - application is inactive
    /// * `on` - application is active
    pub status: String,
}

// WebApp status.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WebAppStatus {
    /// WebAppRuntime application status.
    pub active: bool,
    /// The URL of the current webpage to open on WebApp.
    #[serde(default)]
    pub url: String,
}

/// Provides access to app_control service APIs.
pub struct AppControlService<'a>(&'a Bravia);

impl<'a> AppControlService<'a> {
    pub fn new(bravia: &'a Bravia) -> Self {
        Self(bravia)
    }

    /// Provides the list of applications that can be launched by [setActiveApp](Self::set_active_app).
    ///
    /// # Authentication Level
    /// Private
    ///
    /// # Examples
    /// ```no_run
    /// # use bravia_api::{Bravia, error::Result};
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let bravia = Bravia::new("ADDRESS", Some("PASSWORD")).await?;
    /// let app_list = bravia.app_control().get_application_list().await?;
    /// for app in &app_list {
    ///     println!("{}", app.title);
    /// }
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get_application_list(&self) -> Result<Vec<Application>> {
        let body = RequestBodyBuilder::default()
            .id(60)
            .method("getApplicationList")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .has_result()
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides the status of the application itself or the accompanying status related to a specific application.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_application_status_list(&self) -> Result<Vec<ApplicationStatus>> {
        let body = RequestBodyBuilder::default()
            .id(55)
            .method("getApplicationStatusList")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .has_result()
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Returns the current text input on the field of the software keyboard, if `enc_key` is set this must be encrypted.
    /// This version (1.1) supports encrypted text data transmission.
    ///
    /// # Arguments
    /// * `enc_key` - Encryption key encrypted by the public key.\
    /// The default value is `None` , which means the data is not encrypted.
    ///
    /// # Authentication Level
    /// Private
    pub async fn get_text_form(&self, enc_key: Option<String>) -> Result<String> {
        let mut params = Map::new();
        if let Some(key) = enc_key {
            params.insert(String::from("encKey"), Value::from(key));
        };

        let body = RequestBodyBuilder::default()
            .id(60)
            .method("getTextForm")
            .version(Some("1.1"))
            .params(Value::from(params))
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .has_result()
            .get("text".into())
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides functions to retrieve the status of WebAppRuntime and to retrieve the URL of the current webpage to open on WebApp.
    ///
    /// # Authentication Level
    /// Private
    pub async fn get_web_app_status(&self) -> Result<WebAppStatus> {
        let body = RequestBodyBuilder::default()
            .id(1)
            .method("getWebAppStatus")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .has_result()
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides functions to launch an application.
    ///
    /// # Arguments
    /// * `uri` - URI of target application.
    ///     * `localapp://webappruntime?url=target_url` - launch target_url
    ///     * `localapp://webappruntime?manifest=manifest_url` - launch an application in manifest_url
    ///     * `localapp://webappruntime?auid=application_unique_id` - launch the application in auid=application_unique_id in the USB storage
    ///
    /// # Authentication Level
    /// Generic
    ///
    /// # Examples
    /// ```no_run
    /// # use bravia_api::{Bravia, error::Result};
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let bravia = Bravia::new("ADDRESS", Some("PASSWORD")).await?;
    /// bravia.app_control().set_active_app("localapp://webappruntime?url=http%3A%2F%2Fexample.com%2F".to_string()).await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn set_active_app(&self, uri: String) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("uri"), Value::from(uri));

        let body = RequestBodyBuilder::default()
            .id(601)
            .method("setActiveApp")
            .params(Value::from(params))
            .build()?;
        RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .make(self.0)
            .await?;
        Ok(())
    }

    /// Provides the function to input text on the field of the software keyboard.
    ///
    /// # Arguments
    /// * `text` - Text data encoded by UTF8. If `enc_key` is set, this must be encrypted.
    /// * `enc_key` - Encryption key encrypted by the public key.\
    /// The default value is `None`, which means the data is not encrypted.
    ///     * Not supported with API version 1.0
    /// * `version` - API version.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_text_form(
        &self,
        text: String,
        enc_key: Option<String>,
        version: Option<&str>,
    ) -> Result<()> {
        let params = if let Some(version) = version {
            if version == "1.1" {
                let mut map = Map::new();
                if let Some(enc_key) = enc_key {
                    map.insert(String::from("encKey"), Value::from(enc_key));
                }
                map.insert(String::from("text"), Value::from(text));
                Value::from(map)
            } else {
                Value::from(text)
            }
        } else {
            Value::from(text)
        };

        let body = RequestBodyBuilder::default()
            .id(601)
            .method("setTextForm")
            .version(version)
            .params(params)
            .build()?;
        RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .make(self.0)
            .await?;
        Ok(())
    }

    /// Provides the function to terminate all applications.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn terminate_apps(&self) -> Result<()> {
        let body = RequestBodyBuilder::default()
            .id(55)
            .method("terminateApps")
            .build()?;
        RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .make(self.0)
            .await?;
        Ok(())
    }
}
