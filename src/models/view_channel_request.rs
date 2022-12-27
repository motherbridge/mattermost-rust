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
pub struct ViewChannelRequest {
    /// The channel ID that is being viewed. Use a blank string to indicate that all channels have lost focus.
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// The channel ID of the previous channel, used when switching channels. Providing this ID will cause push notifications to clear on the channel being switched to.
    #[serde(rename = "prev_channel_id", skip_serializing_if = "Option::is_none")]
    pub prev_channel_id: Option<String>,
}

impl ViewChannelRequest {
    pub fn new(channel_id: String) -> ViewChannelRequest {
        ViewChannelRequest {
            channel_id,
            prev_channel_id: None,
        }
    }
}

