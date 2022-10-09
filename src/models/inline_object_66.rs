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
pub struct InlineObject66 {
    /// The ID of the channel to upload to.
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// The name of the file to upload.
    #[serde(rename = "filename")]
    pub filename: String,
    /// The size of the file to upload in bytes.
    #[serde(rename = "file_size")]
    pub file_size: i64,
}

impl InlineObject66 {
    pub fn new(channel_id: String, filename: String, file_size: i64) -> InlineObject66 {
        InlineObject66 {
            channel_id,
            filename,
            file_size,
        }
    }
}


