/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

/// GroupsGroup : Group object to create.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupsGroup {
    /// The unique group name used for at-mentioning.
    #[serde(rename = "name")]
    pub name: String,
    /// The display name of the group which can include spaces.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// Must be `custom`
    #[serde(rename = "source")]
    pub source: String,
    /// Must be true
    #[serde(rename = "allow_reference")]
    pub allow_reference: bool,
}

impl GroupsGroup {
    /// Group object to create.
    pub fn new(name: String, display_name: String, source: String, allow_reference: bool) -> GroupsGroup {
        GroupsGroup {
            name,
            display_name,
            source,
            allow_reference,
        }
    }
}


