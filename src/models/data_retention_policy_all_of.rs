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
pub struct DataRetentionPolicyAllOf {
    /// The ID of this retention policy.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl DataRetentionPolicyAllOf {
    pub fn new() -> DataRetentionPolicyAllOf {
        DataRetentionPolicyAllOf {
            id: None,
        }
    }
}


