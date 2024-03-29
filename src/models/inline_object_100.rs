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
pub struct InlineObject100 {
    /// The string to search in the channel name, display name, and purpose.
    #[serde(rename = "term", skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// Filters results to channels belonging to the given team ids 
    #[serde(rename = "team_ids", skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    /// Filters results to only return Public / Open channels, can be used in conjunction with `private` to return both `public` and `private` channels 
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// Filters results to only return Private channels, can be used in conjunction with `public` to return both `private` and `public` channels 
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /// Filters results to only return deleted / archived channels 
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

impl InlineObject100 {
    pub fn new() -> InlineObject100 {
        InlineObject100 {
            term: None,
            team_ids: None,
            public: None,
            private: None,
            deleted: None,
        }
    }
}


