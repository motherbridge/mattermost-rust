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
pub struct InlineObject69 {
    /// Number of users requested (20% extra is going to be added)
    #[serde(rename = "users")]
    pub users: i32,
}

impl InlineObject69 {
    pub fn new(users: i32) -> InlineObject69 {
        InlineObject69 {
            users,
        }
    }
}


