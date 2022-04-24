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
pub struct GlobalDataRetentionPolicy {
    /// Indicates whether data retention policy deletion of messages is enabled globally.
    #[serde(rename = "message_deletion_enabled", skip_serializing_if = "Option::is_none")]
    pub message_deletion_enabled: Option<bool>,
    /// Indicates whether data retention policy deletion of file attachments is enabled globally.
    #[serde(rename = "file_deletion_enabled", skip_serializing_if = "Option::is_none")]
    pub file_deletion_enabled: Option<bool>,
    /// The current server timestamp before which messages should be deleted.
    #[serde(rename = "message_retention_cutoff", skip_serializing_if = "Option::is_none")]
    pub message_retention_cutoff: Option<i64>,
    /// The current server timestamp before which files should be deleted.
    #[serde(rename = "file_retention_cutoff", skip_serializing_if = "Option::is_none")]
    pub file_retention_cutoff: Option<i64>,
}

impl GlobalDataRetentionPolicy {
    pub fn new() -> GlobalDataRetentionPolicy {
        GlobalDataRetentionPolicy {
            message_deletion_enabled: None,
            file_deletion_enabled: None,
            message_retention_cutoff: None,
            file_retention_cutoff: None,
        }
    }
}


