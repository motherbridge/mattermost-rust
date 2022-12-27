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
pub struct CreateSchemeRequest {
    /// The name of the scheme
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the scheme
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The scope of the scheme (\"team\" or \"channel\")
    #[serde(rename = "scope")]
    pub scope: String,
}

impl CreateSchemeRequest {
    pub fn new(name: String, scope: String) -> CreateSchemeRequest {
        CreateSchemeRequest {
            name,
            description: None,
            scope,
        }
    }
}

