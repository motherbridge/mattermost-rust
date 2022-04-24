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
pub struct InlineObject95 {
    /// Channel Id where the command will execute
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// The slash command to execute
    #[serde(rename = "command")]
    pub command: String,
}

impl InlineObject95 {
    pub fn new(channel_id: String, command: String) -> InlineObject95 {
        InlineObject95 {
            channel_id,
            command,
        }
    }
}


