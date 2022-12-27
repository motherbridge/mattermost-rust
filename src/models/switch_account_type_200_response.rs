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
pub struct SwitchAccountType200Response {
    /// The link for the user to follow to login or to complete the account switching when the current service is OAuth2/SAML
    #[serde(rename = "follow_link", skip_serializing_if = "Option::is_none")]
    pub follow_link: Option<String>,
}

impl SwitchAccountType200Response {
    pub fn new() -> SwitchAccountType200Response {
        SwitchAccountType200Response {
            follow_link: None,
        }
    }
}

