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
pub struct GenerateMfaSecret200Response {
    /// The MFA secret as a string
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A base64 encoded QR code image
    #[serde(rename = "qr_code", skip_serializing_if = "Option::is_none")]
    pub qr_code: Option<String>,
}

impl GenerateMfaSecret200Response {
    pub fn new() -> GenerateMfaSecret200Response {
        GenerateMfaSecret200Response {
            secret: None,
            qr_code: None,
        }
    }
}

