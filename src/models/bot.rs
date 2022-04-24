/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

/// Bot : A bot account



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Bot {
    /// The user id of the associated user entry.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The time in milliseconds a bot was created
    #[serde(rename = "create_at", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<i64>,
    /// The time in milliseconds a bot was last updated
    #[serde(rename = "update_at", skip_serializing_if = "Option::is_none")]
    pub update_at: Option<i64>,
    /// The time in milliseconds a bot was deleted
    #[serde(rename = "delete_at", skip_serializing_if = "Option::is_none")]
    pub delete_at: Option<i64>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The user id of the user that currently owns this bot.
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

impl Bot {
    /// A bot account
    pub fn new() -> Bot {
        Bot {
            user_id: None,
            create_at: None,
            update_at: None,
            delete_at: None,
            username: None,
            display_name: None,
            description: None,
            owner_id: None,
        }
    }
}


