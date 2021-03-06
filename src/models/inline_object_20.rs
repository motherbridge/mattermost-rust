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
pub struct InlineObject20 {
    /// The user access token GUID to revoke
    #[serde(rename = "token_id")]
    pub token_id: String,
}

impl InlineObject20 {
    pub fn new(token_id: String) -> InlineObject20 {
        InlineObject20 {
            token_id,
        }
    }
}


