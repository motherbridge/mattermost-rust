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
pub struct UpdateTeamPrivacyRequest {
    /// Team privacy setting: 'O' for a public (open) team, 'I' for a private (invitation only) team
    #[serde(rename = "privacy")]
    pub privacy: String,
}

impl UpdateTeamPrivacyRequest {
    pub fn new(privacy: String) -> UpdateTeamPrivacyRequest {
        UpdateTeamPrivacyRequest {
            privacy,
        }
    }
}


