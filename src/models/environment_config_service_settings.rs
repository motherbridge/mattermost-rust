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
pub struct EnvironmentConfigServiceSettings {
    #[serde(rename = "SiteURL", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<bool>,
    #[serde(rename = "ListenAddress", skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<bool>,
    #[serde(rename = "ConnectionSecurity", skip_serializing_if = "Option::is_none")]
    pub connection_security: Option<bool>,
    #[serde(rename = "TLSCertFile", skip_serializing_if = "Option::is_none")]
    pub tls_cert_file: Option<bool>,
    #[serde(rename = "TLSKeyFile", skip_serializing_if = "Option::is_none")]
    pub tls_key_file: Option<bool>,
    #[serde(rename = "UseLetsEncrypt", skip_serializing_if = "Option::is_none")]
    pub use_lets_encrypt: Option<bool>,
    #[serde(rename = "LetsEncryptCertificateCacheFile", skip_serializing_if = "Option::is_none")]
    pub lets_encrypt_certificate_cache_file: Option<bool>,
    #[serde(rename = "Forward80To443", skip_serializing_if = "Option::is_none")]
    pub forward80_to443: Option<bool>,
    #[serde(rename = "ReadTimeout", skip_serializing_if = "Option::is_none")]
    pub read_timeout: Option<bool>,
    #[serde(rename = "WriteTimeout", skip_serializing_if = "Option::is_none")]
    pub write_timeout: Option<bool>,
    #[serde(rename = "MaximumLoginAttempts", skip_serializing_if = "Option::is_none")]
    pub maximum_login_attempts: Option<bool>,
    #[serde(rename = "SegmentDeveloperKey", skip_serializing_if = "Option::is_none")]
    pub segment_developer_key: Option<bool>,
    #[serde(rename = "GoogleDeveloperKey", skip_serializing_if = "Option::is_none")]
    pub google_developer_key: Option<bool>,
    #[serde(rename = "EnableOAuthServiceProvider", skip_serializing_if = "Option::is_none")]
    pub enable_o_auth_service_provider: Option<bool>,
    #[serde(rename = "EnableIncomingWebhooks", skip_serializing_if = "Option::is_none")]
    pub enable_incoming_webhooks: Option<bool>,
    #[serde(rename = "EnableOutgoingWebhooks", skip_serializing_if = "Option::is_none")]
    pub enable_outgoing_webhooks: Option<bool>,
    #[serde(rename = "EnableCommands", skip_serializing_if = "Option::is_none")]
    pub enable_commands: Option<bool>,
    #[serde(rename = "EnableOnlyAdminIntegrations", skip_serializing_if = "Option::is_none")]
    pub enable_only_admin_integrations: Option<bool>,
    #[serde(rename = "EnablePostUsernameOverride", skip_serializing_if = "Option::is_none")]
    pub enable_post_username_override: Option<bool>,
    #[serde(rename = "EnablePostIconOverride", skip_serializing_if = "Option::is_none")]
    pub enable_post_icon_override: Option<bool>,
    #[serde(rename = "EnableTesting", skip_serializing_if = "Option::is_none")]
    pub enable_testing: Option<bool>,
    #[serde(rename = "EnableDeveloper", skip_serializing_if = "Option::is_none")]
    pub enable_developer: Option<bool>,
    #[serde(rename = "EnableSecurityFixAlert", skip_serializing_if = "Option::is_none")]
    pub enable_security_fix_alert: Option<bool>,
    #[serde(rename = "EnableInsecureOutgoingConnections", skip_serializing_if = "Option::is_none")]
    pub enable_insecure_outgoing_connections: Option<bool>,
    #[serde(rename = "EnableMultifactorAuthentication", skip_serializing_if = "Option::is_none")]
    pub enable_multifactor_authentication: Option<bool>,
    #[serde(rename = "EnforceMultifactorAuthentication", skip_serializing_if = "Option::is_none")]
    pub enforce_multifactor_authentication: Option<bool>,
    #[serde(rename = "AllowCorsFrom", skip_serializing_if = "Option::is_none")]
    pub allow_cors_from: Option<bool>,
    #[serde(rename = "SessionLengthWebInDays", skip_serializing_if = "Option::is_none")]
    pub session_length_web_in_days: Option<bool>,
    #[serde(rename = "SessionLengthMobileInDays", skip_serializing_if = "Option::is_none")]
    pub session_length_mobile_in_days: Option<bool>,
    #[serde(rename = "SessionLengthSSOInDays", skip_serializing_if = "Option::is_none")]
    pub session_length_ssoin_days: Option<bool>,
    #[serde(rename = "SessionCacheInMinutes", skip_serializing_if = "Option::is_none")]
    pub session_cache_in_minutes: Option<bool>,
    #[serde(rename = "WebsocketSecurePort", skip_serializing_if = "Option::is_none")]
    pub websocket_secure_port: Option<bool>,
    #[serde(rename = "WebsocketPort", skip_serializing_if = "Option::is_none")]
    pub websocket_port: Option<bool>,
    #[serde(rename = "WebserverMode", skip_serializing_if = "Option::is_none")]
    pub webserver_mode: Option<bool>,
    #[serde(rename = "EnableCustomEmoji", skip_serializing_if = "Option::is_none")]
    pub enable_custom_emoji: Option<bool>,
    #[serde(rename = "RestrictCustomEmojiCreation", skip_serializing_if = "Option::is_none")]
    pub restrict_custom_emoji_creation: Option<bool>,
}

impl EnvironmentConfigServiceSettings {
    pub fn new() -> EnvironmentConfigServiceSettings {
        EnvironmentConfigServiceSettings {
            site_url: None,
            listen_address: None,
            connection_security: None,
            tls_cert_file: None,
            tls_key_file: None,
            use_lets_encrypt: None,
            lets_encrypt_certificate_cache_file: None,
            forward80_to443: None,
            read_timeout: None,
            write_timeout: None,
            maximum_login_attempts: None,
            segment_developer_key: None,
            google_developer_key: None,
            enable_o_auth_service_provider: None,
            enable_incoming_webhooks: None,
            enable_outgoing_webhooks: None,
            enable_commands: None,
            enable_only_admin_integrations: None,
            enable_post_username_override: None,
            enable_post_icon_override: None,
            enable_testing: None,
            enable_developer: None,
            enable_security_fix_alert: None,
            enable_insecure_outgoing_connections: None,
            enable_multifactor_authentication: None,
            enforce_multifactor_authentication: None,
            allow_cors_from: None,
            session_length_web_in_days: None,
            session_length_mobile_in_days: None,
            session_length_ssoin_days: None,
            session_cache_in_minutes: None,
            websocket_secure_port: None,
            websocket_port: None,
            webserver_mode: None,
            enable_custom_emoji: None,
            restrict_custom_emoji_creation: None,
        }
    }
}


