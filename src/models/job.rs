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
pub struct Job {
    /// The unique id of the job
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of job
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The time at which the job was created
    #[serde(rename = "create_at", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<i64>,
    /// The time at which the job was started
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The last time at which the job had activity
    #[serde(rename = "last_activity_at", skip_serializing_if = "Option::is_none")]
    pub last_activity_at: Option<i64>,
    /// The status of the job
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The progress (as a percentage) of the job
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// A freeform data field containing additional information about the job
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl Job {
    pub fn new() -> Job {
        Job {
            id: None,
            r#type: None,
            create_at: None,
            start_at: None,
            last_activity_at: None,
            status: None,
            progress: None,
            data: None,
        }
    }
}


