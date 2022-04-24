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
pub struct InlineResponse2007 {
    /// The channels that matched the query.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<crate::models::Channel>>,
    /// The total number of results, regardless of page and per_page requested.
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f32>,
}

impl InlineResponse2007 {
    pub fn new() -> InlineResponse2007 {
        InlineResponse2007 {
            channels: None,
            total_count: None,
        }
    }
}


