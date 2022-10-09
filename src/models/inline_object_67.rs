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
pub struct InlineObject67 {
    /// The type of job to create
    #[serde(rename = "type")]
    pub _type: String,
    /// An object containing any additional data required for this job type
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl InlineObject67 {
    pub fn new(_type: String) -> InlineObject67 {
        InlineObject67 {
            _type,
            data: None,
        }
    }
}


