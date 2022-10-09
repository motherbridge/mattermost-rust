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
pub struct InlineObject87 {
    #[serde(rename = "group")]
    pub group: Box<crate::models::GroupsGroup>,
    /// The user ids of the group members to add.
    #[serde(rename = "user_ids")]
    pub user_ids: Vec<String>,
}

impl InlineObject87 {
    pub fn new(group: crate::models::GroupsGroup, user_ids: Vec<String>) -> InlineObject87 {
        InlineObject87 {
            group: Box::new(group),
            user_ids,
        }
    }
}


