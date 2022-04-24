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
    /// The Site URL to test
    #[serde(rename = "site_url")]
    pub site_url: String,
}

impl InlineObject67 {
    pub fn new(site_url: String) -> InlineObject67 {
        InlineObject67 {
            site_url,
        }
    }
}


