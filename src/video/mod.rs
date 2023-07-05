//! APIs that are related to video functions.

use crate::{error::Result, Bravia, RequestBodyBuilder, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

const ENDPOINT: &str = "video";

fn bool_true() -> bool {
    true
}

fn candidate_f64_default() -> f64 {
    -1.0
}

/// Candidates of specified settings.\
/// Only if there is no candidate, `None` is set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    /// Value of candidate.\
    /// `""` can be set only when settings value is integer and
    /// the candidate can be represented by `max`, `min`, and `step`.
    #[serde(default)]
    pub value: String,
    /// Max value of specified settings.\
    /// Only if value of target is not indicated by numerical number, -1 is set.
    #[serde(default = "candidate_f64_default")]
    pub max: f64,
    /// Min value of specified settings.\
    /// Only if value of target is not indicated by numerical number, -1 is set.
    #[serde(default = "candidate_f64_default")]
    pub min: f64,
    /// Step value of specified settings.\
    /// Only if value of target is not indicated by numerical number, -1 is set.
    #[serde(default = "candidate_f64_default")]
    pub step: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PictureQualitySettingsResponse {
    /// Target name
    pub target: String,
    /// Current value of target
    pub current_value: String,
    /// This target is currently available or not
    #[serde(default = "bool_true")]
    pub is_available: bool,
    pub candidate: Option<Vec<Candidate>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PictureQualitySettingsRequest {
    /// Target name (UI setting target).\
    /// Please use [get_picture_quality_settings](VideoService::get_picture_quality_settings) to acquire the available targets.
    // From my tests I can't get it to work with None values.
    pub target: Option<String>,
    /// * `value` - The value to set for target name.
    pub value: Option<String>,
}

impl PictureQualitySettingsRequest {
    pub fn new(target: Option<String>, value: Option<String>) -> Self {
        Self { target, value }
    }
}

/// Provides access to video service APIs.
pub struct VideoService<'a>(&'a Bravia);

impl<'a> VideoService<'a> {
    pub fn new(bravia: &'a Bravia) -> Self {
        Self(bravia)
    }

    /// Provides current settings and supported settings related to picture quality configuration items.
    ///
    /// # Arguments
    /// * `target` - Target name
    ///     * `color` - Adjust the color saturation level.
    ///     * `brightness` - Adjust the luminance level of the screen.
    ///     * `contrast` - Adjust the picture white level.
    ///     * `sharpness` - Adjust the picture detail.
    ///     * `pictureMode` - Set picture mode.
    ///     * `lightSensor` - Optimize brightness according to ambient light.
    ///     * `colorSpace` - Change the color reproduction range.
    ///     * `colorTemperature` - Adjust the color temperature.
    ///     * `autoPictureMode` - Automatically selects the picture mode based on the viewing content.
    ///     * `hdrMode` - Picture that is suitable for a High Dynamic Range signal.
    ///     * `autoLocalDimming` - Optimizes contrast by adjusting brightness in individual sections of the screen.
    ///     * `xtendedDynamicRange` - Adjust peak luminance for the brightness whites and blackest blacks.
    ///     * `None` - Settings of all targets.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_picture_quality_settings(
        &self,
        target: Option<String>,
    ) -> Result<Vec<PictureQualitySettingsResponse>> {
        let mut params = Map::new();
        if let Some(target) = target {
            params.insert(String::from("target"), Value::from(target));
        }

        let body = RequestBodyBuilder::default()
            .id(52)
            .method("getPictureQualitySettings")
            .params(Value::from(params))
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides a function to change settings related to picture quality setting items.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_picture_quality_settings(
        &self,
        settings: Vec<PictureQualitySettingsRequest>,
    ) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("settings"), serde_json::to_value(settings)?);

        let body = RequestBodyBuilder::default()
            .id(12)
            .method("setPictureQualitySettings")
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
