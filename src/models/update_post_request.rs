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
pub struct UpdatePostRequest {
    /// ID of the post to update
    #[serde(rename = "id")]
    pub id: String,
    /// Set to `true` to pin the post to the channel it is in
    #[serde(rename = "is_pinned", skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
    /// The message text of the post
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Set to `true` if the post has reactions to it
    #[serde(rename = "has_reactions", skip_serializing_if = "Option::is_none")]
    pub has_reactions: Option<bool>,
    /// A general JSON property bag to attach to the post
    #[serde(rename = "props", skip_serializing_if = "Option::is_none")]
    pub props: Option<String>,
}

impl UpdatePostRequest {
    pub fn new(id: String) -> UpdatePostRequest {
        UpdatePostRequest {
            id,
            is_pinned: None,
            message: None,
            has_reactions: None,
            props: None,
        }
    }
}


