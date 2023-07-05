//! APIs related to the control of input and output of the device
//! for AV contents and to the manipulation of AV contents themselves on the device.
//!
//! # Getting content information
//! To get information on the content that a device has,
//! the client needs to know the source of the URI.
//! A client can get the URI of the source by using
//! [getSchemeList](AvContentService::get_scheme_list) and [getSourceList](AvContentService::get_source_list).\
//! \
//! At first, the client gets scheme information with the [getSchemeList](AvContentService::get_scheme_list).
//! Afterward, the client sets this scheme to the `scheme` parameter
//! of [getSourceList](AvContentService::get_source_list) and calls this API to get the URI of the source.\
//! \
//! The client sets the URI of the source to the `uri` parameter of [getContentList](AvContentService::get_content_list)
//! and calls this API to get the content information or browse the content.

use crate::{error::Result, Bravia, RequestBodyBuilder, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

const ENDPOINT: &str = "avContent";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Content {
    /// URI to identify the content.
    pub uri: String,
    /// Title of this content to be recognized by the user.\
    /// The default value is `None`, this means that there is no title information.
    pub title: Option<String>,
    /// Index of the list.\
    /// This starts with `stIdx` that is indicated in the request.
    /// When this value is -1, this indicates that the content itself is specified by the URI in the request parameter.
    #[serde(default)]
    pub index: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExternalInputStatus {
    /// Icon type to give a hint to the application which icon to show for the user.\
    /// The type is indicated by a `meta` URI format and implies that the developers of the client side should prepare some actual images with respect to the meta URI. The following meta URIs are defined.
    /// * `meta:composite` - Composite input
    /// * `meta:svideo` - S-Video input
    /// * `meta:composite_componentd` - Composite and D-Component combined input
    /// * `meta:component` - Component input (Y and Pb/Cb and Pr/Cr connectors)
    /// * `meta:componentd` - D-Component input
    /// * `meta:scart` - SCART input
    /// * `meta:hdmi` - HDMI input
    /// * `meta:dsub15` - D-subminiature 15pin input
    /// * `meta:tuner` - Tuner device is connected.
    /// * `meta:tape` - Tape player device is connected.
    /// * `meta:disc` - Disk player device is connected.
    /// * `meta:complex` - Complex device is connected.
    /// * `meta:avamp` - AV amp device is connected.
    /// * `meta:hometheater` - Home theater device is connected.
    /// * `meta:game` - Game player is connected.
    /// * `meta:camcoder` - Video camera is connected.
    /// * `meta:digitalcamera` - Still camera is connected.
    /// * `meta:pc` - Personal computer is connected.
    /// * `meta:tv` - TV-type CEC device is connected.
    /// * `meta:audiosystem` - Audio system-type CEC device is connected.
    /// * `meta:recordingdevice` - Recording-type CEC device is connected.
    /// * `meta:playbackdevice` - Playback-type CEC device is connected.
    /// * `meta:tunerdevice` - Tuner-type CEC device is connected.
    /// * `meta:wifidisplay` - WiFi Display input
    pub icon: String,
    /// Input connection status.
    pub connection: bool,
    /// Label name of the input set by the user.
    pub label: String,
    /// Name of input.
    pub title: String,
    /// URI to identify the content.
    pub uri: String,
    /// Input signal status.\
    /// Not supported on version 1.0.\
    /// Values:
    /// * `true` - signal is detected.
    /// * `false` - signal is not detected.
    /// * `None` - unknown (Default on version 1.0)
    pub status: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayingContentInfo {
    /// Source of the content.
    pub source: String,
    /// Title of this content to be recognized by the user.
    ///
    /// # Note
    /// Use the [getCurrentExternalInputStatus](AvContentService::get_current_external_input_status)
    ///  method to get the label name that a user sets via the UI setting.
    pub title: String,
    /// URI to identify the content.
    pub uri: String,
}

/// Provides access to av_content service APIs.
pub struct AvContentService<'a>(&'a Bravia);

impl<'a> AvContentService<'a> {
    pub fn new(bravia: &'a Bravia) -> Self {
        Self(bravia)
    }

    /// Provides the count of contents in the source.
    /// With version 1.1 it's possibile to specify a `target` value.
    ///
    /// # Arguments
    /// * `source` - Source name composed of the URI with a scheme and path.
    /// * `target` - Not available with API version 1.0
    /// * `version` - API version.
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
    /// let hdmi_count = bravia.av_content().get_content_count("extInput:hdmi".to_string(), None, None, Some("1.1")).await?;
    /// println!("hdmi: {}", hdmi_count);
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get_content_count(
        &self,
        source: String,
        content_type: Option<String>,
        target: Option<String>,
        version: Option<&str>,
    ) -> Result<usize> {
        let mut params = Map::new();
        params.insert(String::from("source"), Value::from(source));
        if let Some(t) = content_type {
            params.insert(String::from("type"), Value::from(t));
        }
        if let Some(version) = version {
            if version == "1.1" {
                if let Some(t) = target {
                    params.insert(String::from("target"), Value::from(t));
                }
            }
        }

        let body = RequestBodyBuilder::default()
            .id(11)
            .method("getContentCount")
            .version(version)
            .params(Value::from(params))
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .has_result()
            .get("count".into())
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides the list of contents under the URI.\
    /// If the number of contents is too large to be retrieved in a single request,
    /// the `st_idx` and `cnt` parameters should be used to retrieve partial lists.
    /// To get the complete list, multiple requests are to be made by adjusting
    /// the `st_idx` and `cnt` parameters.\
    /// There is a maximum limit on the number of contents that can be retrieved
    /// in a single request. This limit is device specific.
    /// The `cnt` parameter also has the same maximum limit.
    ///
    /// # Arguments
    /// * `uri` - URI to identify the content. `None` means all contents are supported by the device.
    /// * `st_idx` - Start index to get list items. The default value is 0.
    /// * `cnt` - Count of the maximum number of items that can be listed, starting from `stIdx`. The
    /// default value is `50`.
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
    /// let hdmi_list = bravia.av_content().get_content_list(Some("extInput:hdmi".to_string()), None, None).await?;
    /// for element in hdmi_list {
    ///     println!("{} - {}", element.index, element.title.unwrap());
    /// }
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get_content_list(
        &self,
        uri: Option<String>,
        st_idx: Option<u32>,
        cnt: Option<u32>,
    ) -> Result<Vec<Content>> {
        let mut params = Map::new();
        if let Some(uri) = uri {
            params.insert(String::from("uri"), Value::from(uri));
        };
        if let Some(st_idx) = st_idx {
            params.insert(String::from("stIdx"), Value::from(st_idx));
        }
        if let Some(cnt) = cnt {
            params.insert(String::from("cnt"), Value::from(cnt));
        }

        let body = RequestBodyBuilder::default()
            .id(88)
            .method("getContentList")
            .version(Some("1.5"))
            .params(Value::from(params))
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

    /// Provides information on the current status of all external input sources of the device.
    ///
    /// # Arguments
    /// * `version` - API version.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_current_external_input_status(
        &self,
        version: Option<&str>,
    ) -> Result<Vec<ExternalInputStatus>> {
        let body = RequestBodyBuilder::default()
            .id(105)
            .method("getCurrentExternalInputsStatus")
            .version(version)
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// This API provides the list of schemes that the device can handle.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_scheme_list(&self) -> Result<Vec<String>> {
        let body = RequestBodyBuilder::default()
            .id(1)
            .method("getSchemeList")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .make(self.0)
            .await?;

        let vec: Vec<HashMap<String, String>> = serde_json::from_value(req)?;
        let mut result = Vec::new();
        for map in vec {
            result.push(map.into_values().collect());
        }
        Ok(result)
    }

    /// Provides the list of sources in the scheme.
    ///
    /// # Arguments
    /// * `scheme` - Scheme name.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_source_list(&self, scheme: String) -> Result<Vec<String>> {
        let mut params = Map::new();
        params.insert(String::from("scheme"), Value::from(scheme));

        let body = RequestBodyBuilder::default()
            .id(1)
            .method("getSourceList")
            .params(Value::from(params))
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .make(self.0)
            .await?;

        let vec: Vec<HashMap<String, String>> = serde_json::from_value(req)?;
        let mut result = Vec::new();
        for map in vec {
            result.push(map.into_values().collect());
        }
        Ok(result)
    }

    /// Provides information of the currently playing content or the currently selected input.
    ///
    /// # Authentication Level
    /// Private
    pub async fn get_playing_content_info(&self) -> Result<PlayingContentInfo> {
        let body = RequestBodyBuilder::default()
            .id(103)
            .method("getPlayingContentInfo")
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

    /// Provides the function to play content.
    /// With this API, content specified in the request parameter is shown to the user.
    ///
    /// # Arguments
    /// `uri` - URI obtained from [getContentList](AvContentService::get_content_list) API.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_play_content(&self, uri: String) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("uri"), Value::from(uri));

        let body = RequestBodyBuilder::default()
            .id(101)
            .method("setPlayContent")
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
}
