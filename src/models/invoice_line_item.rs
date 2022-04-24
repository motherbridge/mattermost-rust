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
pub struct InvoiceLineItem {
    #[serde(rename = "price_id", skip_serializing_if = "Option::is_none")]
    pub price_id: Option<String>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(rename = "price_per_unit", skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<i64>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<String>>,
}

impl InvoiceLineItem {
    pub fn new() -> InvoiceLineItem {
        InvoiceLineItem {
            price_id: None,
            total: None,
            quantity: None,
            price_per_unit: None,
            description: None,
            metadata: None,
        }
    }
}


