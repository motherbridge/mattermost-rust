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
pub struct Config {
    #[serde(rename = "ServiceSettings", skip_serializing_if = "Option::is_none")]
    pub service_settings: Option<Box<crate::models::ConfigServiceSettings>>,
    #[serde(rename = "TeamSettings", skip_serializing_if = "Option::is_none")]
    pub team_settings: Option<Box<crate::models::ConfigTeamSettings>>,
    #[serde(rename = "SqlSettings", skip_serializing_if = "Option::is_none")]
    pub sql_settings: Option<Box<crate::models::ConfigSqlSettings>>,
    #[serde(rename = "LogSettings", skip_serializing_if = "Option::is_none")]
    pub log_settings: Option<Box<crate::models::ConfigLogSettings>>,
    #[serde(rename = "PasswordSettings", skip_serializing_if = "Option::is_none")]
    pub password_settings: Option<Box<crate::models::ConfigPasswordSettings>>,
    #[serde(rename = "FileSettings", skip_serializing_if = "Option::is_none")]
    pub file_settings: Option<Box<crate::models::ConfigFileSettings>>,
    #[serde(rename = "EmailSettings", skip_serializing_if = "Option::is_none")]
    pub email_settings: Option<Box<crate::models::ConfigEmailSettings>>,
    #[serde(rename = "RateLimitSettings", skip_serializing_if = "Option::is_none")]
    pub rate_limit_settings: Option<Box<crate::models::ConfigRateLimitSettings>>,
    #[serde(rename = "PrivacySettings", skip_serializing_if = "Option::is_none")]
    pub privacy_settings: Option<Box<crate::models::ConfigPrivacySettings>>,
    #[serde(rename = "SupportSettings", skip_serializing_if = "Option::is_none")]
    pub support_settings: Option<Box<crate::models::ConfigSupportSettings>>,
    #[serde(rename = "GitLabSettings", skip_serializing_if = "Option::is_none")]
    pub git_lab_settings: Option<Box<crate::models::ConfigGitLabSettings>>,
    #[serde(rename = "GoogleSettings", skip_serializing_if = "Option::is_none")]
    pub google_settings: Option<Box<crate::models::ConfigGitLabSettings>>,
    #[serde(rename = "Office365Settings", skip_serializing_if = "Option::is_none")]
    pub office365_settings: Option<Box<crate::models::ConfigGitLabSettings>>,
    #[serde(rename = "LdapSettings", skip_serializing_if = "Option::is_none")]
    pub ldap_settings: Option<Box<crate::models::ConfigLdapSettings>>,
    #[serde(rename = "ComplianceSettings", skip_serializing_if = "Option::is_none")]
    pub compliance_settings: Option<Box<crate::models::ConfigComplianceSettings>>,
    #[serde(rename = "LocalizationSettings", skip_serializing_if = "Option::is_none")]
    pub localization_settings: Option<Box<crate::models::ConfigLocalizationSettings>>,
    #[serde(rename = "SamlSettings", skip_serializing_if = "Option::is_none")]
    pub saml_settings: Option<Box<crate::models::ConfigSamlSettings>>,
    #[serde(rename = "NativeAppSettings", skip_serializing_if = "Option::is_none")]
    pub native_app_settings: Option<Box<crate::models::ConfigNativeAppSettings>>,
    #[serde(rename = "ClusterSettings", skip_serializing_if = "Option::is_none")]
    pub cluster_settings: Option<Box<crate::models::ConfigClusterSettings>>,
    #[serde(rename = "MetricsSettings", skip_serializing_if = "Option::is_none")]
    pub metrics_settings: Option<Box<crate::models::ConfigMetricsSettings>>,
    #[serde(rename = "AnalyticsSettings", skip_serializing_if = "Option::is_none")]
    pub analytics_settings: Option<Box<crate::models::ConfigAnalyticsSettings>>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            service_settings: None,
            team_settings: None,
            sql_settings: None,
            log_settings: None,
            password_settings: None,
            file_settings: None,
            email_settings: None,
            rate_limit_settings: None,
            privacy_settings: None,
            support_settings: None,
            git_lab_settings: None,
            google_settings: None,
            office365_settings: None,
            ldap_settings: None,
            compliance_settings: None,
            localization_settings: None,
            saml_settings: None,
            native_app_settings: None,
            cluster_settings: None,
            metrics_settings: None,
            analytics_settings: None,
        }
    }
}


