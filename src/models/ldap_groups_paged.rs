/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

/// LdapGroupsPaged : A paged list of LDAP groups



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LdapGroupsPaged {
    /// Total number of groups
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<f32>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::LdapGroup>>,
}

impl LdapGroupsPaged {
    /// A paged list of LDAP groups
    pub fn new() -> LdapGroupsPaged {
        LdapGroupsPaged {
            count: None,
            groups: None,
        }
    }
}


