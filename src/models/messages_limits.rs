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
pub struct MessagesLimits {
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<i64>,
}

impl MessagesLimits {
    pub fn new() -> MessagesLimits {
        MessagesLimits {
            history: None,
        }
    }
}


