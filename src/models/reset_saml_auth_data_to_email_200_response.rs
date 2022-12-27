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
pub struct ResetSamlAuthDataToEmail200Response {
    /// The number of users whose AuthData field was reset.
    #[serde(rename = "num_affected", skip_serializing_if = "Option::is_none")]
    pub num_affected: Option<i32>,
}

impl ResetSamlAuthDataToEmail200Response {
    pub fn new() -> ResetSamlAuthDataToEmail200Response {
        ResetSamlAuthDataToEmail200Response {
            num_affected: None,
        }
    }
}

