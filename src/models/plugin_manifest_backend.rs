/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

/// PluginManifestBackend : Deprecated in Mattermost 5.2 release.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PluginManifestBackend {
    /// Path to the executable binary.
    #[serde(rename = "executable", skip_serializing_if = "Option::is_none")]
    pub executable: Option<String>,
}

impl PluginManifestBackend {
    /// Deprecated in Mattermost 5.2 release.
    pub fn new() -> PluginManifestBackend {
        PluginManifestBackend {
            executable: None,
        }
    }
}


