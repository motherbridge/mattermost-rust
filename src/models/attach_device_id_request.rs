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
pub struct AttachDeviceIdRequest {
    /// Mobile device id. For Android prefix the id with `android:` and Apple with `apple:`
    #[serde(rename = "device_id")]
    pub device_id: String,
}

impl AttachDeviceIdRequest {
    pub fn new(device_id: String) -> AttachDeviceIdRequest {
        AttachDeviceIdRequest {
            device_id,
        }
    }
}


