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
pub struct InlineObject36 {
    /// The search term to match against the name or display name of teams
    #[serde(rename = "term", skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// The page number to return, if paginated. If this parameter is not present with the `per_page` parameter then the results will be returned un-paged.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// The number of entries to return per page, if paginated. If this parameter is not present with the `page` parameter then the results will be returned un-paged.
    #[serde(rename = "per_page", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,
    /// Filters results to teams where `allow_open_invite` is set to true or false, excludes group constrained channels if this filter option is passed. If this filter option is not passed then the query will remain unchanged. __Minimum server version__: 5.28 
    #[serde(rename = "allow_open_invite", skip_serializing_if = "Option::is_none")]
    pub allow_open_invite: Option<bool>,
    /// Filters results to teams where `group_constrained` is set to true or false, returns the union of results when used with `allow_open_invite` If the filter option is not passed then the query will remain unchanged. __Minimum server version__: 5.28 
    #[serde(rename = "group_constrained", skip_serializing_if = "Option::is_none")]
    pub group_constrained: Option<bool>,
    /// If set to true, only teams which do not have a granular retention policy assigned to them will be returned. The `sysconsole_read_compliance_data_retention` permission is required to use this parameter. __Minimum server version__: 5.35 
    #[serde(rename = "exclude_policy_constrained", skip_serializing_if = "Option::is_none")]
    pub exclude_policy_constrained: Option<bool>,
}

impl InlineObject36 {
    pub fn new() -> InlineObject36 {
        InlineObject36 {
            term: None,
            page: None,
            per_page: None,
            allow_open_invite: None,
            group_constrained: None,
            exclude_policy_constrained: None,
        }
    }
}


