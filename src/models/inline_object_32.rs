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
pub struct InlineObject32 {
    /// Unique handler for a team, will be present in the team URL
    #[serde(rename = "name")]
    pub name: String,
    /// Non-unique UI name for the team
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// `'O'` for open, `'I'` for invite only
    #[serde(rename = "type")]
    pub _type: String,
}

impl InlineObject32 {
    pub fn new(name: String, display_name: String, _type: String) -> InlineObject32 {
        InlineObject32 {
            name,
            display_name,
            _type,
        }
    }
}


