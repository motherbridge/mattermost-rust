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
pub struct ChannelMember {
    #[serde(rename = "channel_id", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<String>,
    /// The time in milliseconds the channel was last viewed by the user
    #[serde(rename = "last_viewed_at", skip_serializing_if = "Option::is_none")]
    pub last_viewed_at: Option<i64>,
    #[serde(rename = "msg_count", skip_serializing_if = "Option::is_none")]
    pub msg_count: Option<i64>,
    #[serde(rename = "mention_count", skip_serializing_if = "Option::is_none")]
    pub mention_count: Option<i64>,
    #[serde(rename = "notify_props", skip_serializing_if = "Option::is_none")]
    pub notify_props: Option<Box<crate::models::ChannelNotifyProps>>,
    /// The time in milliseconds the channel member was last updated
    #[serde(rename = "last_update_at", skip_serializing_if = "Option::is_none")]
    pub last_update_at: Option<i64>,
}

impl ChannelMember {
    pub fn new() -> ChannelMember {
        ChannelMember {
            channel_id: None,
            user_id: None,
            roles: None,
            last_viewed_at: None,
            msg_count: None,
            mention_count: None,
            notify_props: None,
            last_update_at: None,
        }
    }
}


