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
pub struct TeamStats {
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "total_member_count", skip_serializing_if = "Option::is_none")]
    pub total_member_count: Option<i64>,
    #[serde(rename = "active_member_count", skip_serializing_if = "Option::is_none")]
    pub active_member_count: Option<i64>,
}

impl TeamStats {
    pub fn new() -> TeamStats {
        TeamStats {
            team_id: None,
            total_member_count: None,
            active_member_count: None,
        }
    }
}


