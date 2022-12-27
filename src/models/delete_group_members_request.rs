/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

/// DeleteGroupMembersRequest : An object containing the user ids of the members to remove.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteGroupMembersRequest {
    #[serde(rename = "user_ids", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl DeleteGroupMembersRequest {
    /// An object containing the user ids of the members to remove.
    pub fn new() -> DeleteGroupMembersRequest {
        DeleteGroupMembersRequest {
            user_ids: None,
        }
    }
}

