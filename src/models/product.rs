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
pub struct Product {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "price_per_seat", skip_serializing_if = "Option::is_none")]
    pub price_per_seat: Option<String>,
    #[serde(rename = "add_ons", skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<crate::models::AddOn>>,
}

impl Product {
    pub fn new() -> Product {
        Product {
            id: None,
            name: None,
            description: None,
            price_per_seat: None,
            add_ons: None,
        }
    }
}


