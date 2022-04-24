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
pub struct Compliance {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "create_at", skip_serializing_if = "Option::is_none")]
    pub create_at: Option<i64>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,
    #[serde(rename = "keywords", skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<String>,
}

impl Compliance {
    pub fn new() -> Compliance {
        Compliance {
            id: None,
            create_at: None,
            user_id: None,
            status: None,
            count: None,
            desc: None,
            _type: None,
            start_at: None,
            end_at: None,
            keywords: None,
            emails: None,
        }
    }
}


