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
pub struct UserNotifyProps {
    /// Set to \"true\" to enable email notifications, \"false\" to disable. Defaults to \"true\".
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    /// Set to \"all\" to receive push notifications for all activity, \"mention\" for mentions and direct messages only, and \"none\" to disable. Defaults to \"mention\".
    #[serde(rename = "push", skip_serializing_if = "Option::is_none")]
    pub push: Option<String>,
    /// Set to \"all\" to receive desktop notifications for all activity, \"mention\" for mentions and direct messages only, and \"none\" to disable. Defaults to \"all\".
    #[serde(rename = "desktop", skip_serializing_if = "Option::is_none")]
    pub desktop: Option<String>,
    /// Set to \"true\" to enable sound on desktop notifications, \"false\" to disable. Defaults to \"true\".
    #[serde(rename = "desktop_sound", skip_serializing_if = "Option::is_none")]
    pub desktop_sound: Option<bool>,
    /// A comma-separated list of words to count as mentions. Defaults to username and @username.
    #[serde(rename = "mention_keys", skip_serializing_if = "Option::is_none")]
    pub mention_keys: Option<String>,
    /// Set to \"true\" to enable channel-wide notifications (@channel, @all, etc.), \"false\" to disable. Defaults to \"true\".
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<bool>,
    /// Set to \"true\" to enable mentions for first name. Defaults to \"true\" if a first name is set, \"false\" otherwise.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<bool>,
}

impl UserNotifyProps {
    pub fn new() -> UserNotifyProps {
        UserNotifyProps {
            email: None,
            push: None,
            desktop: None,
            desktop_sound: None,
            mention_keys: None,
            channel: None,
            first_name: None,
        }
    }
}


