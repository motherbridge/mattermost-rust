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
pub struct CreatePostEphemeralRequest {
    /// The target user id for the ephemeral post
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "post")]
    pub post: Box<crate::models::CreatePostEphemeralRequestPost>,
}

impl CreatePostEphemeralRequest {
    pub fn new(user_id: String, post: crate::models::CreatePostEphemeralRequestPost) -> CreatePostEphemeralRequest {
        CreatePostEphemeralRequest {
            user_id,
            post: Box::new(post),
        }
    }
}


