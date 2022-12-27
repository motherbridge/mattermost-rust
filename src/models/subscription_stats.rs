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
pub struct SubscriptionStats {
    #[serde(rename = "remaining_seats", skip_serializing_if = "Option::is_none")]
    pub remaining_seats: Option<i32>,
    #[serde(rename = "is_paid_tier", skip_serializing_if = "Option::is_none")]
    pub is_paid_tier: Option<String>,
}

impl SubscriptionStats {
    pub fn new() -> SubscriptionStats {
        SubscriptionStats {
            remaining_seats: None,
            is_paid_tier: None,
        }
    }
}


