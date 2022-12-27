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
pub struct SearchEmojiRequest {
    /// The term to match against the emoji name.
    #[serde(rename = "term")]
    pub term: String,
    /// Set to only search for names starting with the search term.
    #[serde(rename = "prefix_only", skip_serializing_if = "Option::is_none")]
    pub prefix_only: Option<String>,
}

impl SearchEmojiRequest {
    pub fn new(term: String) -> SearchEmojiRequest {
        SearchEmojiRequest {
            term,
            prefix_only: None,
        }
    }
}

