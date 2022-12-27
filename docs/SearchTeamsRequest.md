# SearchTeamsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**term** | Option<**String**> | The search term to match against the name or display name of teams | [optional]
**page** | Option<**String**> | The page number to return, if paginated. If this parameter is not present with the `per_page` parameter then the results will be returned un-paged. | [optional]
**per_page** | Option<**String**> | The number of entries to return per page, if paginated. If this parameter is not present with the `page` parameter then the results will be returned un-paged. | [optional]
**allow_open_invite** | Option<**bool**> | Filters results to teams where `allow_open_invite` is set to true or false, excludes group constrained channels if this filter option is passed. If this filter option is not passed then the query will remain unchanged. __Minimum server version__: 5.28  | [optional]
**group_constrained** | Option<**bool**> | Filters results to teams where `group_constrained` is set to true or false, returns the union of results when used with `allow_open_invite` If the filter option is not passed then the query will remain unchanged. __Minimum server version__: 5.28  | [optional]
**exclude_policy_constrained** | Option<**bool**> | If set to true, only teams which do not have a granular retention policy assigned to them will be returned. The `sysconsole_read_compliance_data_retention` permission is required to use this parameter. __Minimum server version__: 5.35  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


