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
pub struct PatchUserRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "props", skip_serializing_if = "Option::is_none")]
    pub props: Option<serde_json::Value>,
    #[serde(rename = "notify_props", skip_serializing_if = "Option::is_none")]
    pub notify_props: Option<Box<crate::models::UserNotifyProps>>,
}

impl PatchUserRequest {
    pub fn new() -> PatchUserRequest {
        PatchUserRequest {
            email: None,
            username: None,
            first_name: None,
            last_name: None,
            nickname: None,
            locale: None,
            position: None,
            props: None,
            notify_props: None,
        }
    }
}


