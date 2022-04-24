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
pub struct GroupSyncableTeams {
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "team_display_name", skip_serializing_if = "Option::is_none")]
    pub team_display_name: Option<String>,
    #[serde(rename = "team_type", skip_serializing_if = "Option::is_none")]
    pub team_type: Option<String>,
    #[serde(rename = "group_id", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "auto_add", skip_serializing_if = "Option::is_none")]
    pub auto_add: Option<bool>,
    #[serde(rename = "create_at", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<i64>,
    #[serde(rename = "delete_at", skip_serializing_if = "Option::is_none")]
    pub delete_at: Option<i64>,
    #[serde(rename = "update_at", skip_serializing_if = "Option::is_none")]
    pub update_at: Option<i64>,
}

impl GroupSyncableTeams {
    pub fn new() -> GroupSyncableTeams {
        GroupSyncableTeams {
            team_id: None,
            team_display_name: None,
            team_type: None,
            group_id: None,
            auto_add: None,
            create_at: None,
            delete_at: None,
            update_at: None,
        }
    }
}


