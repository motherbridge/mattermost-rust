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
pub struct Session {
    /// The time in milliseconds a session was created
    #[serde(rename = "create_at", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<i64>,
    #[serde(rename = "device_id", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// The time in milliseconds a session will expire
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "is_oauth", skip_serializing_if = "Option::is_none")]
    pub is_oauth: Option<bool>,
    /// The time in milliseconds of the last activity of a session
    #[serde(rename = "last_activity_at", skip_serializing_if = "Option::is_none")]
    pub last_activity_at: Option<i64>,
    #[serde(rename = "props", skip_serializing_if = "Option::is_none")]
    pub props: Option<serde_json::Value>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<String>,
    #[serde(rename = "team_members", skip_serializing_if = "Option::is_none")]
    pub team_members: Option<Vec<crate::models::TeamMember>>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            create_at: None,
            device_id: None,
            expires_at: None,
            id: None,
            is_oauth: None,
            last_activity_at: None,
            props: None,
            roles: None,
            team_members: None,
            token: None,
            user_id: None,
        }
    }
}


