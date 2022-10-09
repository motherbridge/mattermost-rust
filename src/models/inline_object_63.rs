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
pub struct InlineObject63 {
    /// Target time for the reminder
    #[serde(rename = "target_time")]
    pub target_time: i64,
}

impl InlineObject63 {
    pub fn new(target_time: i64) -> InlineObject63 {
        InlineObject63 {
            target_time,
        }
    }
}


