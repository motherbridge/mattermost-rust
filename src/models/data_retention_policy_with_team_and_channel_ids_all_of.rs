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
pub struct DataRetentionPolicyWithTeamAndChannelIdsAllOf {
    /// The IDs of the teams to which this policy should be applied.
    #[serde(rename = "team_ids", skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    /// The IDs of the channels to which this policy should be applied.
    #[serde(rename = "channel_ids", skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
}

impl DataRetentionPolicyWithTeamAndChannelIdsAllOf {
    pub fn new() -> DataRetentionPolicyWithTeamAndChannelIdsAllOf {
        DataRetentionPolicyWithTeamAndChannelIdsAllOf {
            team_ids: None,
            channel_ids: None,
        }
    }
}


