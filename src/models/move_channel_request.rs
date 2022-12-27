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
pub struct MoveChannelRequest {
    #[serde(rename = "team_id")]
    pub team_id: String,
    /// Remove members those are not member of target team before moving the channel.
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl MoveChannelRequest {
    pub fn new(team_id: String) -> MoveChannelRequest {
        MoveChannelRequest {
            team_id,
            force: None,
        }
    }
}


