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
pub struct StorageUsage {
    /// Total file storage usage for the instance in bytes rounded down to the most significant digit
    #[serde(rename = "bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<f32>,
}

impl StorageUsage {
    pub fn new() -> StorageUsage {
        StorageUsage {
            bytes: None,
        }
    }
}


