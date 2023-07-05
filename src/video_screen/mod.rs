//! APIs that are related to video screen functions.
//!
//! # Note
//! This API is able to set the value at API call timing,
//!  which might vary depending on the device state. (For example, depending on the "current" input source.)

use crate::{error::Result, Bravia, RequestBodyBuilder, RequestBuilder};
use serde_json::{Map, Value};

const ENDPOINT: &str = "videoScreen";

/// Provides access to video_screen service APIs.
pub struct VideoScreenService<'a>(&'a Bravia);

impl<'a> VideoScreenService<'a> {
    pub fn new(bravia: &'a Bravia) -> Self {
        Self(bravia)
    }

    /// Provides the function to change the current scene setting value.
    ///
    /// # Arguments
    /// * `value` - Scene of the input source.
    ///     * `auto` - Automatically selects the scene based on the viewing content.
    ///     * `auto24pSync` - Automatically selects `Cinema` for 24Hz signal content. Behaves as `Auto` for all other signals.
    ///     * `general` - Turns off scene select for general content.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_scene_settings(&self, value: String) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("value"), Value::from(value));

        let body = RequestBodyBuilder::default()
            .id(40)
            .method("setSceneSetting")
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
