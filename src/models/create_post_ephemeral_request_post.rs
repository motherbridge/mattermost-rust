/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

/// CreatePostEphemeralRequestPost : Post object to create



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePostEphemeralRequestPost {
    /// The channel ID to post in
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// The message contents, can be formatted with Markdown
    #[serde(rename = "message")]
    pub message: String,
}

impl CreatePostEphemeralRequestPost {
    /// Post object to create
    pub fn new(channel_id: String, message: String) -> CreatePostEphemeralRequestPost {
        CreatePostEphemeralRequestPost {
            channel_id,
            message,
        }
    }
}

