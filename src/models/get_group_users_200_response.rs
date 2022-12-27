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
pub struct GetGroupUsers200Response {
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::User>>,
    #[serde(rename = "total_member_count", skip_serializing_if = "Option::is_none")]
    pub total_member_count: Option<i32>,
}

impl GetGroupUsers200Response {
    pub fn new() -> GetGroupUsers200Response {
        GetGroupUsers200Response {
            members: None,
            total_member_count: None,
        }
    }
}


