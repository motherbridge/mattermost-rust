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
pub struct DataRetentionPolicyWithTeamAndChannelIds {
    /// The display name for this retention policy.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The number of days a message will be retained before being deleted by this policy. If this value is less than 0, the policy has infinite retention (i.e. messages are never deleted). 
    #[serde(rename = "post_duration", skip_serializing_if = "Option::is_none")]
    pub post_duration: Option<i64>,
    /// The IDs of the teams to which this policy should be applied.
    #[serde(rename = "team_ids", skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    /// The IDs of the channels to which this policy should be applied.
    #[serde(rename = "channel_ids", skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
}

impl DataRetentionPolicyWithTeamAndChannelIds {
    pub fn new() -> DataRetentionPolicyWithTeamAndChannelIds {
        DataRetentionPolicyWithTeamAndChannelIds {
            display_name: None,
            post_duration: None,
            team_ids: None,
            channel_ids: None,
        }
    }
}


