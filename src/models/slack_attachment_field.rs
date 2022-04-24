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
pub struct SlackAttachmentField {
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The value of the attachment, set as string but capable with golang interface
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Short", skip_serializing_if = "Option::is_none")]
    pub short: Option<bool>,
}

impl SlackAttachmentField {
    pub fn new() -> SlackAttachmentField {
        SlackAttachmentField {
            title: None,
            value: None,
            short: None,
        }
    }
}


