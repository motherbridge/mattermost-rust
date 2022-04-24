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
pub struct OAuthApp {
    /// The client id of the application
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The client secret of the application
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// The name of the client application
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A short description of the application
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A URL to an icon to display with the application
    #[serde(rename = "icon_url", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// A list of callback URLs for the appliation
    #[serde(rename = "callback_urls", skip_serializing_if = "Option::is_none")]
    pub callback_urls: Option<Vec<String>>,
    /// A link to the website of the application
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// Set this to `true` to skip asking users for permission
    #[serde(rename = "is_trusted", skip_serializing_if = "Option::is_none")]
    pub is_trusted: Option<bool>,
    /// The time of registration for the application
    #[serde(rename = "create_at", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<i64>,
    /// The last time of update for the application
    #[serde(rename = "update_at", skip_serializing_if = "Option::is_none")]
    pub update_at: Option<i64>,
}

impl OAuthApp {
    pub fn new() -> OAuthApp {
        OAuthApp {
            id: None,
            client_secret: None,
            name: None,
            description: None,
            icon_url: None,
            callback_urls: None,
            homepage: None,
            is_trusted: None,
            create_at: None,
            update_at: None,
        }
    }
}


