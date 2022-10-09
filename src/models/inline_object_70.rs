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
pub struct InlineObject70 {
    /// Number of users requested (20% extra is going to be added)
    #[serde(rename = "users")]
    pub users: i64,
}

impl InlineObject70 {
    pub fn new(users: i64) -> InlineObject70 {
        InlineObject70 {
            users,
        }
    }
}


