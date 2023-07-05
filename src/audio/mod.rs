//! APIs that are related to audio functions like volume, sound effects and so on.

use crate::{error::Result, Bravia, RequestBodyBuilder, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

const ENDPOINT: &str = "audio";

/// Target name.
/// * `outputTerminal` - Selecting speakers or terminals to output sound.
pub type SoundTarget = String;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SoundSettings {
    pub target: SoundTarget,
    /// Current value of target.\
    /// If `target` is `outputTerminal`:
    /// * `speaker` - Audio is output from the speaker.
    /// * `speaker_hdmi` - Audio is output from the speaker and HDMI.
    /// * `hdmi` - Audio is output from HDMI.
    /// * `audioSystem` - Audio is output from HDMI or digital audio output.
    #[serde(alias = "currentValue")]
    pub value: String,
}

impl SoundSettings {
    pub fn new(target: SoundTarget, value: String) -> Self {
        Self { target, value }
    }
}

/// Target name. (UI setting target)
/// * `tvPosition` - Sets the sound according to the display position.
/// * `subwooferLevel` - Sets the level of the Subwoofer speaker.
///     Note that the range and step values vary depending on the device.
/// * `subwooferFreq` - Adjusts the cut off frequency of the Wireless Subwoofer.
///     All frequencies below the cut off frequency are output to the Wireless Subwoofer instead of the display speakers.
/// * `subwooferPhase` - Sets the phase polarity of the subwoofer.
/// * `subwooferPower` - Sets the power control method of the Wireless Subwoofer.
pub type SpeakerTarget = String;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SpeakerSettings {
    pub target: SpeakerTarget,
    /// Target value.
    /// * If `target` is `tvPosition`
    ///     * `tableTop` - Provides the best sound quality when you place the display on a TV stand.
    ///     * `wallMount` - Provides the best sound quality when you hang the display on a wall.
    /// * If `target` is `subwooferLevel`
    ///     * `0` - Minimum value.
    ///     * `24` - Maximum value.
    /// * If `target` is `subwooferFreq`
    ///     * `0` - Minimum value.
    ///     * `30` - Maximum value.
    /// * If `target` is `subwooferPhase`
    ///     * `normal` - normal.
    ///     * `reverse` - reverse.
    /// * If `target` is `subwooferPower`
    ///     * `on` - on.
    ///     * `off` - off.
    #[serde(alias = "currentValue")]
    pub value: String,
}

impl SpeakerSettings {
    pub fn new(target: SpeakerTarget, value: String) -> Self {
        Self { target, value }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInformation {
    /// Output target of the sound.
    /// The following values are defined:
    /// * `speaker` - outputs sound to the speaker(s)
    /// * `headphone` - outputs sound to the headphones
    pub target: String,
    /// Current volume.
    pub volume: usize,
    /// Current mute status.
    pub mute: bool,
    /// Max volume level.
    pub max_volume: usize,
    /// Min volume level.
    pub min_volume: usize,
}

/// Provides access to audio service APIs.
pub struct AudioService<'a>(&'a Bravia);

impl<'a> AudioService<'a> {
    pub fn new(bravia: &'a Bravia) -> Self {
        Self(bravia)
    }

    /// Provides the current settings and supported settings related to the sound configuration items.
    ///
    /// # Arguments
    /// `target`
    /// * `None` - This indicates the settings of all targets.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_sound_settings(
        &self,
        target: Option<SoundTarget>,
    ) -> Result<Vec<SoundSettings>> {
        let mut params = Map::new();
        params.insert(String::from("target"), Value::from(target));

        let body = RequestBodyBuilder::default()
            .id(73)
            .method("getSoundSettings")
            .version(Some("1.1"))
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

    /// Provides current settings and supported settings related to speaker configuration items.
    ///
    /// # Arguments
    /// `target`
    /// * `None` - This indicates the settings of all targets.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_speaker_settings(
        &self,
        target: Option<SpeakerTarget>,
    ) -> Result<Vec<SpeakerSettings>> {
        let mut params = Map::new();
        params.insert(String::from("target"), Value::from(target));

        let body = RequestBodyBuilder::default()
            .id(67)
            .method("getSpeakerSettings")
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

    /// Provides information about the sound volume (and mute status) of the device.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_volume_information(&self) -> Result<Vec<VolumeInformation>> {
        let body = RequestBodyBuilder::default()
            .id(33)
            .method("getVolumeInformation")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides the function to change the audio mute status.
    ///
    /// # Arguments
    /// * `status` - Mute status to set.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_audio_mute(&self, status: bool) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("status"), Value::from(status));

        let body = RequestBodyBuilder::default()
            .id(601)
            .method("setAudioMute")
            .params(Value::from(params))
            .build()?;
        RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .has_result()
            .make(self.0)
            .await?;
        Ok(())
    }

    /// Provides the function to change the audio volume level.
    ///
    /// # Arguments
    /// * `target` -  Output target of the sound. The following values are defined:
    ///     * `None` - outputs sound to all output equipment of the device.
    ///     If the mute information of all outputs is the same, this value is set.
    ///     * `speaker` - outputs sound to the speaker(s).
    ///     * `headphone` - outputs sound to the headphones.
    /// * `volume` - Volume level to set. The following formats are applied:
    ///     * `N` - N is a numeric string (ex. "25"). The volume is set to level N.
    ///     * `+N` - N is a numeric string (ex. "+14"). The volume is increased by an increment of N.
    ///     * `-N` - N is a numeric string (ex. "-10"). The volume is reduced by an increment of N.
    /// * `ui` - If the UI (volume bar, etc.) should be displayed, set this `on`.
    ///     * Supported with API version 1.2
    ///     <!-- end of the list -->
    ///     Values:
    ///     * `on` - UI is displayed.
    ///     * `off` - UI is not displayed.
    ///     * `None` - Not specified. (depends on the server)
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
    /// let volume_info = bravia.audio().get_volume_information().await?;
    /// for element in volume_info.iter() {
    ///     if element.target == "speaker" && element.volume > 10 {
    ///         bravia.audio()
    ///             .set_audio_volume(
    ///                 Some("speaker".to_string()),
    ///                 "10".to_string(),
    ///                 Some("on".to_string()),
    ///                 None
    ///             )
    ///             .await?
    ///     }
    /// }
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn set_audio_volume(
        &self,
        target: Option<String>,
        volume: String,
        ui: Option<String>,
        version: Option<&str>,
    ) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("target"), Value::from(target));
        params.insert(String::from("volume"), Value::from(volume));
        if let Some(version) = version {
            if version == "1.2" && ui.is_some() {
                params.insert(String::from("ui"), Value::from(ui));
            }
        }

        let body = RequestBodyBuilder::default()
            .id(98)
            .method("setAudioVolume")
            .version(version)
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

    /// Provides the function to change the settings related to sound setting items.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_sound_settings(&self, settings: Vec<SoundSettings>) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("settings"), serde_json::to_value(settings)?);

        let body = RequestBodyBuilder::default()
            .id(5)
            .method("setSoundSettings")
            .version(Some("1.1"))
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

    /// Provides the function to change the settings related to speaker setting items.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_speaker_settings(&self, settings: Vec<SpeakerSettings>) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("settings"), serde_json::to_value(settings)?);

        let body = RequestBodyBuilder::default()
            .id(62)
            .method("setSpeakerSettings")
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
