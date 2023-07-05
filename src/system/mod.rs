//! APIs that are related to basic device functions.

use crate::{error::Result, Bravia, RequestBodyBuilder, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

const ENDPOINT: &str = "system";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    /// Current time set in the device (ISO8601 format).
    pub date_time: String,
    /// Timezone offset (unit: minute, range: ±(23*60+59)).\
    /// Not available with API version 1.0.
    pub time_zone_offset_minute: Option<usize>,
    /// DST offset (unit: minute, range: ±(23*60+59)).\
    /// Not available with API version 1.0.
    pub dst_offset_minute: Option<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceInfo {
    /// Category name of the device.
    pub product_category: String,
    /// More detailed product information can be returned if [product_category](Self::product_category) is not enough.
    pub product_name: String,
    /// Model name.
    pub model_name: String,
    /// If the device can launch multiple REST API servers, return this server's name for the client to distinguish.
    pub server_name: String,
    /// Version for the client to change its behavior with regards to significant differences within product_category.
    /// This version is managed/controlled within each product_category.
    /// The composition of this parameter is `[X].[Y].[Z]`,
    /// where `[X]`, `[Y]`, and `[Z]` are strings each representing an integer and concatenated with periods "." in between.
    /// * `[X]`: This value is assigned and incremented so that the client can distinguish any significant differences
    /// between devices or groups of devices within product_category.
    /// How this value is assigned depends on each product_category.
    /// * `[Y]`: This value represents the versions of API sets supported within `[X]`.
    /// This version must be incremented if supported APIs are added or deleted.
    /// * `[Z]`: This value must be incremented if any behavior of existing APIs is changed within `[X.Y]`.
    pub interface_version: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LEDIndicatorStatus {
    /// Functional meaning of the target LED.
    /// * `Demo`
    /// * `AutoBrightnessAdjust`
    /// * `Dark`
    /// * `SimpleResponse`
    /// * `Off`
    pub mode: String,
    /// LED Indicator status.
    /// * `true` - On
    /// * `false` - Off
    /// * `None`
    ///     * input: the server decides the behavior
    ///     * output: unknown
    pub status: Option<String>,
}

impl LEDIndicatorStatus {
    pub fn new(mode: String, status: Option<String>) -> Self {
        Self { mode, status }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSettings {
    /// Network Interface.
    pub netif: String,
    /// Hardware address (MAC Address).
    pub hw_addr: String,
    /// IP Address for IPV4.
    pub ip_addr_v4: String,
    /// IP Address for IPV6.
    pub ip_addr_v6: String,
    /// Netmask.
    pub netmask: String,
    /// Gateway.
    pub gateway: String,
    /// DNS.
    pub dns: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RemoteControllerAction {
    /// Name of remote control button.
    pub name: String,
    /// IRCC code value that is supported in this device.
    pub value: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteDeviceSettings {
    /// Target name.
    /// * `accessPermission` - Sets whether to permit access from remote devices, which can access the server device from outside the door.
    pub target: String,
    /// Current value of the target.
    /// If `target` is `accessPermission`:
    /// * `on` - access is permitted
    /// * `off` - access is not permitted
    pub current_value: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SystemInformation {
    /// Device category.
    pub product: String,
    /// Language code of the device, represented by ISO-639 alpha-3.\
    /// The default value is `""`, and if the server device cannot send this parameter,
    /// an empty string is returned.
    #[serde(default)]
    pub language: String,
    /// Name of the product.\
    /// This must be unique within each product.
    pub model: String,
    /// Serial ID assigned to each device.\
    /// The default value is `""`, and if the server device cannot send this parameter,
    /// an empty string is returned.
    #[serde(default)]
    pub serial: String,
    /// Ethernet MAC address.\
    /// Default value is `""` and in case server device can not send this parameter,
    /// empty string is returned.
    #[serde(default)]
    pub mac_addr: String,
    /// Product name.\
    /// This must be unique within each category.
    pub name: String,
    /// Represents the rough age and season of the device in the market.\
    /// The parameter is composed of `[X].[Y].[Z]`,
    /// where `[X]`, `[Y]`, and `[Z]` are strings each representing an integer,
    /// concatenated with periods "." in between.
    /// This must be unique within each category.\
    /// The default value is, `""` and if the server device cannot send this parameter,
    /// an empty string is returned.
    #[serde(default)]
    pub generation: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SupportedFunction {
    /// Option name to identify the function.
    /// * `WOL` - If the server supports WOL, the MAC address is set as a value.
    pub option: String,
    /// Current value for each option.\
    /// The value varies per option and the following value is defined.
    /// * (ex) `00:00:00:00:00:00:00:E0` (The MAC Address when the option value is `WOL`.)
    pub value: String,
}

/// Provides access to system service APIs.
pub struct SystemService<'a>(&'a Bravia);

impl<'a> SystemService<'a> {
    pub fn new(bravia: &'a Bravia) -> Self {
        Self(bravia)
    }

    /// Provides the current time, parameters of timezone and DST offset information.
    ///
    /// # Arguments
    /// * `version` - API version.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_current_time(&self, version: Option<&str>) -> Result<Time> {
        let body = RequestBodyBuilder::default()
            .id(51)
            .method("getCurrentTime")
            .version(version)
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .make(self.0)
            .await?;

        if version.is_none() || version == Some("1.0") {
            let date_time: String = serde_json::from_value(req)?;
            let time = Time {
                date_time,
                time_zone_offset_minute: None,
                dst_offset_minute: None,
            };
            Ok(time)
        } else {
            Ok(serde_json::from_value(req)?)
        }
    }

    /// Provides information of the REST API interface provided by the server.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_interface_information(&self) -> Result<InterfaceInfo> {
        let body = RequestBodyBuilder::default()
            .id(33)
            .method("getInterfaceInformation")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides function to get the LED Indicator mode.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn get_led_indicator_status(&self) -> Result<LEDIndicatorStatus> {
        let body = RequestBodyBuilder::default()
            .id(45)
            .method("getLEDIndicatorStatus")
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

    /// Provides information about network settings.
    ///
    /// # Arguments
    /// * `netif` - Network interface.\
    /// The default value is `None`, this indicates all interfaces.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn get_network_settings(
        &self,
        netif: Option<String>,
    ) -> Result<Vec<NetworkSettings>> {
        let mut params = Map::new();
        if let Some(netif) = netif {
            params.insert(String::from("netif"), Value::from(netif));
        }

        let body = RequestBodyBuilder::default()
            .id(2)
            .method("getNetworkSettings")
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

    /// Provides the setting of the power saving mode to adjust the device's power consumption.
    ///
    /// # Returns
    /// Current power saving mode, the following values are defined:
    /// * `off` - Power saving mode is disabled.
    /// * `low` - Power saving mode is enabled at a low level.
    /// * `high` - Power saving mode is enabled at a high level.
    /// * `pictureOff` - Power saving mode is enabled with the panel output off.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_power_saving_mode(&self) -> Result<String> {
        let body = RequestBodyBuilder::default()
            .id(51)
            .method("getPowerSavingMode")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .get("mode".into())
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides the current power status of the device.
    ///
    /// # Returns
    /// Current power status, the following values are defined:
    /// * `standby` - Device is in the power off state.
    /// * `active` - Device is in the power on state.
    ///
    /// # Authentication Level
    /// None
    ///
    /// # Note
    /// It is possible that some devices may not respond when they are in the power off state.
    pub async fn get_power_status(&self) -> Result<String> {
        let body = RequestBodyBuilder::default()
            .id(50)
            .method("getPowerStatus")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .get("status".into())
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides the information of the device's remote controller.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_remote_controller_info(&self) -> Result<Vec<RemoteControllerAction>> {
        let body = RequestBodyBuilder::default()
            .id(54)
            .method("getRemoteControllerInfo")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .get(1.into())
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides the current settings and supported settings related to remote devices, which can access the server device from outside the door.
    ///
    /// # Arguments
    /// * `target` - Target name.
    /// * `accessPermission` - Sets whether to permit access from remote devices,
    /// which can access the server device from outside the door.
    /// * `None` - Settings of all targets.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_remote_device_settings(
        &self,
        target: Option<String>,
    ) -> Result<Vec<RemoteDeviceSettings>> {
        let mut params = Map::new();
        if let Some(target) = target {
            params.insert(String::from("target"), Value::from(target));
        }

        let body = RequestBodyBuilder::default()
            .id(44)
            .method("getRemoteDeviceSettings")
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

    /// Provides general information on the device.
    ///
    /// # Authentication Level
    /// Private
    pub async fn get_system_information(&self) -> Result<SystemInformation> {
        let body = RequestBodyBuilder::default()
            .id(33)
            .method("getSystemInformation")
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

    /// Provides the list of device capabilities within the scope of system service handling.
    ///
    /// # Authentication Level
    /// None
    pub async fn get_system_supported_function(&self) -> Result<Vec<SupportedFunction>> {
        let body = RequestBodyBuilder::default()
            .id(55)
            .method("getSystemSupportedFunction")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .has_result()
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides information on the device's WoL (Wake-on-LAN) mode settings.\
    /// The mode indicates whether the device receives the WoL packet to power on.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn get_wol_mode(&self) -> Result<bool> {
        let body = RequestBodyBuilder::default()
            .id(50)
            .method("getWolMode")
            .build()?;
        let req = RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .has_result()
            .get("enabled".into())
            .make(self.0)
            .await?;
        Ok(serde_json::from_value(req)?)
    }

    /// Provides the function to reboot the device.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn request_reboot(&self) -> Result<()> {
        let body = RequestBodyBuilder::default()
            .id(10)
            .method("requestReboot")
            .build()?;
        RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .make(self.0)
            .await?;
        Ok(())
    }

    /// Provides the function to light up a specific LED Indicator,
    /// usually equipped in the front of the device to show the current device status to the user.
    ///
    /// # Authentication Level
    /// Generic
    ///
    /// # Note
    /// When requesting to change the LED indicator status with this API,
    /// you should take care not to return it to its original status
    /// when terminating your application.
    pub async fn set_led_indicator_status(&self, led_status: LEDIndicatorStatus) -> Result<()> {
        let body = RequestBodyBuilder::default()
            .id(53)
            .method("setLEDIndicatorStatus")
            .version(Some("1.1"))
            .params(serde_json::to_value(led_status)?)
            .build()?;
        RequestBuilder::default()
            .endpoint(ENDPOINT)
            .body(body)
            .is_protected()
            .make(self.0)
            .await?;
        Ok(())
    }

    /// Provides the language setting of the device.
    ///
    /// # Arguments
    /// `lang` - Language code represented by ISO-639 alpha-3 to set in the device.
    ///
    /// # Authentication Level
    /// Generic
    ///
    /// # Notes
    /// * A special value is required for language to distinguish Chinese characters, as seen below:
    ///     * "CHS": means Simplified Chinese.
    ///     * "CHT": means Traditional Chinese.
    /// * The available values of “language” depend on region or country settings on TV.
    pub async fn set_language(&self, lang: String) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("language"), Value::from(lang));
        let body = RequestBodyBuilder::default()
            .id(55)
            .method("setLanguage")
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

    /// Provides the function to change the setting of the power saving mode
    /// and adjust the device's power consumption.
    ///
    /// # Arguments
    /// `mode` - Current power saving mode. The following values are defined:
    /// * `off` - Power saving mode is disabled.
    /// * `low` - Power saving mode is enabled at a low level.
    /// * `high` - Power saving mode is enabled at a high level.
    /// * `pictureOff` - Power saving mode is enabled with the panel output off.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_power_saving_mode(&self, mode: String) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("mode"), Value::from(mode));
        let body = RequestBodyBuilder::default()
            .id(52)
            .method("setPowerSavingMode")
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

    /// Provides the function to change the current power status of the device.
    ///
    /// # Arguments
    /// `status` - Power status.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_power_status(&self, status: bool) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("status"), Value::from(status));
        let body = RequestBodyBuilder::default()
            .id(55)
            .method("setPowerStatus")
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

    /// Changes the WoL (Wake-on-LAN) mode settings of the device.\
    /// The mode indicates whether the device receives the WoL packet to power on.
    ///
    /// # Arguments
    /// `enabled` - Information on the WoL mode setting.
    ///
    /// # Authentication Level
    /// Generic
    pub async fn set_wol_mode(&self, enabled: bool) -> Result<()> {
        let mut params = Map::new();
        params.insert(String::from("enabled"), Value::from(enabled));
        let body = RequestBodyBuilder::default()
            .id(55)
            .method("setWolMode")
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
