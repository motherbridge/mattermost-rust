/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EnvironmentConfigMetricsSettings {
    #[serde(rename = "Enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "BlockProfileRate", skip_serializing_if = "Option::is_none")]
    pub block_profile_rate: Option<bool>,
    #[serde(rename = "ListenAddress", skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<bool>,
}

impl EnvironmentConfigMetricsSettings {
    pub fn new() -> EnvironmentConfigMetricsSettings {
        EnvironmentConfigMetricsSettings {
            enable: None,
            block_profile_rate: None,
            listen_address: None,
        }
    }
}


