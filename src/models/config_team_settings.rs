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
pub struct ConfigTeamSettings {
    #[serde(rename = "SiteName", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[serde(rename = "MaxUsersPerTeam", skip_serializing_if = "Option::is_none")]
    pub max_users_per_team: Option<i32>,
    #[serde(rename = "EnableTeamCreation", skip_serializing_if = "Option::is_none")]
    pub enable_team_creation: Option<bool>,
    #[serde(rename = "EnableUserCreation", skip_serializing_if = "Option::is_none")]
    pub enable_user_creation: Option<bool>,
    #[serde(rename = "EnableOpenServer", skip_serializing_if = "Option::is_none")]
    pub enable_open_server: Option<bool>,
    #[serde(rename = "RestrictCreationToDomains", skip_serializing_if = "Option::is_none")]
    pub restrict_creation_to_domains: Option<String>,
    #[serde(rename = "EnableCustomBrand", skip_serializing_if = "Option::is_none")]
    pub enable_custom_brand: Option<bool>,
    #[serde(rename = "CustomBrandText", skip_serializing_if = "Option::is_none")]
    pub custom_brand_text: Option<String>,
    #[serde(rename = "CustomDescriptionText", skip_serializing_if = "Option::is_none")]
    pub custom_description_text: Option<String>,
    #[serde(rename = "RestrictDirectMessage", skip_serializing_if = "Option::is_none")]
    pub restrict_direct_message: Option<String>,
    #[serde(rename = "RestrictTeamInvite", skip_serializing_if = "Option::is_none")]
    pub restrict_team_invite: Option<String>,
    #[serde(rename = "RestrictPublicChannelManagement", skip_serializing_if = "Option::is_none")]
    pub restrict_public_channel_management: Option<String>,
    #[serde(rename = "RestrictPrivateChannelManagement", skip_serializing_if = "Option::is_none")]
    pub restrict_private_channel_management: Option<String>,
    #[serde(rename = "RestrictPublicChannelCreation", skip_serializing_if = "Option::is_none")]
    pub restrict_public_channel_creation: Option<String>,
    #[serde(rename = "RestrictPrivateChannelCreation", skip_serializing_if = "Option::is_none")]
    pub restrict_private_channel_creation: Option<String>,
    #[serde(rename = "RestrictPublicChannelDeletion", skip_serializing_if = "Option::is_none")]
    pub restrict_public_channel_deletion: Option<String>,
    #[serde(rename = "RestrictPrivateChannelDeletion", skip_serializing_if = "Option::is_none")]
    pub restrict_private_channel_deletion: Option<String>,
    #[serde(rename = "UserStatusAwayTimeout", skip_serializing_if = "Option::is_none")]
    pub user_status_away_timeout: Option<i32>,
    #[serde(rename = "MaxChannelsPerTeam", skip_serializing_if = "Option::is_none")]
    pub max_channels_per_team: Option<i32>,
    #[serde(rename = "MaxNotificationsPerChannel", skip_serializing_if = "Option::is_none")]
    pub max_notifications_per_channel: Option<i32>,
}

impl ConfigTeamSettings {
    pub fn new() -> ConfigTeamSettings {
        ConfigTeamSettings {
            site_name: None,
            max_users_per_team: None,
            enable_team_creation: None,
            enable_user_creation: None,
            enable_open_server: None,
            restrict_creation_to_domains: None,
            enable_custom_brand: None,
            custom_brand_text: None,
            custom_description_text: None,
            restrict_direct_message: None,
            restrict_team_invite: None,
            restrict_public_channel_management: None,
            restrict_private_channel_management: None,
            restrict_public_channel_creation: None,
            restrict_private_channel_creation: None,
            restrict_public_channel_deletion: None,
            restrict_private_channel_deletion: None,
            user_status_away_timeout: None,
            max_channels_per_team: None,
            max_notifications_per_channel: None,
        }
    }
}


