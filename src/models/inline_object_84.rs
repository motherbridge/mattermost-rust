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
pub struct InlineObject84 {
    /// New IdAttribute value
    #[serde(rename = "toAttribute")]
    pub to_attribute: String,
}

impl InlineObject84 {
    pub fn new(to_attribute: String) -> InlineObject84 {
        InlineObject84 {
            to_attribute,
        }
    }
}


