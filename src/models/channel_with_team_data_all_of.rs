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
pub struct ChannelWithTeamDataAllOf {
    /// The display name of the team to which this channel belongs.
    #[serde(rename = "team_display_name", skip_serializing_if = "Option::is_none")]
    pub team_display_name: Option<String>,
    /// The name of the team to which this channel belongs.
    #[serde(rename = "team_name", skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    /// The time at which the team to which this channel belongs was last updated.
    #[serde(rename = "team_update_at", skip_serializing_if = "Option::is_none")]
    pub team_update_at: Option<i32>,
    /// The data retention policy to which this team has been assigned. If no such policy exists, or the caller does not have the `sysconsole_read_compliance_data_retention` permission, this field will be null.
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}

impl ChannelWithTeamDataAllOf {
    pub fn new() -> ChannelWithTeamDataAllOf {
        ChannelWithTeamDataAllOf {
            team_display_name: None,
            team_name: None,
            team_update_at: None,
            policy_id: None,
        }
    }
}


