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
pub struct TopDmList {
    /// Indicates if there is another page of top DMs that can be fetched.
    #[serde(rename = "has_next", skip_serializing_if = "Option::is_none")]
    pub has_next: Option<bool>,
    /// List of top DMs.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::TopDm>>,
}

impl TopDmList {
    pub fn new() -> TopDmList {
        TopDmList {
            has_next: None,
            items: None,
        }
    }
}

