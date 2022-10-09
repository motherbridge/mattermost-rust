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
pub struct TopChannelList {
    /// Indicates if there is another page of channels that can be fetched.
    #[serde(rename = "has_next", skip_serializing_if = "Option::is_none")]
    pub has_next: Option<bool>,
    /// List of channels.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::TopChannel>>,
}

impl TopChannelList {
    pub fn new() -> TopChannelList {
        TopChannelList {
            has_next: None,
            items: None,
        }
    }
}


