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
pub struct InlineObject102 {
    /// The ID of the plugin to install.
    #[serde(rename = "id")]
    pub id: String,
    /// The version of the plugin to install.
    #[serde(rename = "version")]
    pub version: String,
}

impl InlineObject102 {
    pub fn new(id: String, version: String) -> InlineObject102 {
        InlineObject102 {
            id,
            version,
        }
    }
}


