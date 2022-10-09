# InlineObject45

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**term** | **String** | The string to search in the channel name, display name, and purpose. | 
**not_associated_to_group** | Option<**String**> | A group id to exclude channels that are associated to that group via GroupChannel records. | [optional]
**exclude_default_channels** | Option<**bool**> | Exclude default channels from the results by setting this parameter to true. | [optional]
**team_ids** | Option<**Vec<String>**> | Filters results to channels belonging to the given team ids  __Minimum server version__: 5.26  | [optional]
**group_constrained** | Option<**bool**> | Filters results to only return channels constrained to a group  __Minimum server version__: 5.26  | [optional]
**exclude_group_constrained** | Option<**bool**> | Filters results to exclude channels constrained to a group  __Minimum server version__: 5.26  | [optional]
**public** | Option<**bool**> | Filters results to only return Public / Open channels, can be used in conjunction with `private` to return both `public` and `private` channels  __Minimum server version__: 5.26  | [optional]
**private** | Option<**bool**> | Filters results to only return Private channels, can be used in conjunction with `public` to return both `private` and `public` channels  __Minimum server version__: 5.26  | [optional]
**deleted** | Option<**bool**> | Filters results to only return deleted / archived channels  __Minimum server version__: 5.26  | [optional]
**page** | Option<**String**> | The page number to return, if paginated. If this parameter is not present with the `per_page` parameter then the results will be returned un-paged. | [optional]
**per_page** | Option<**String**> | The number of entries to return per page, if paginated. If this parameter is not present with the `page` parameter then the results will be returned un-paged. | [optional]
**exclude_policy_constrained** | Option<**bool**> | If set to true, only channels which do not have a granular retention policy assigned to them will be returned. The `sysconsole_read_compliance_data_retention` permission is required to use this parameter. __Minimum server version__: 5.35  | [optional][default to false]
**include_search_by_id** | Option<**bool**> | If set to true, returns channels where given search 'term' matches channel ID. __Minimum server version__: 5.35  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


