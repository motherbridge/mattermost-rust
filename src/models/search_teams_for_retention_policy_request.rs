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
pub struct SearchTeamsForRetentionPolicyRequest {
    /// The search term to match against the name or display name of teams
    #[serde(rename = "term", skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
}

impl SearchTeamsForRetentionPolicyRequest {
    pub fn new() -> SearchTeamsForRetentionPolicyRequest {
        SearchTeamsForRetentionPolicyRequest {
            term: None,
        }
    }
}


