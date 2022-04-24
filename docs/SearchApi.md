# \SearchApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_files**](SearchApi.md#search_files) | **POST** /teams/{team_id}/files/search | Search files in a team



## search_files

> crate::models::FileInfoList search_files(team_id, terms, is_or_search, time_zone_offset, include_deleted_channels, page, per_page)
Search files in a team

Search for files in a team based on file name, extention and file content (if file content extraction is enabled and supported for the files). __Minimum server version__: 5.34 ##### Permissions Must be authenticated and have the `view_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**terms** | **String** | The search terms as inputed by the user. To search for files from a user include `from:someusername`, using a user's username. To search in a specific channel include `in:somechannel`, using the channel name (not the display name). To search for specific extensions included `ext:extension`. | [required] |
**is_or_search** | **bool** | Set to true if an Or search should be performed vs an And search. | [required] |
**time_zone_offset** | Option<**i64**> | Offset from UTC of user timezone for date searches. |  |[default to 0]
**include_deleted_channels** | Option<**bool**> | Set to true if deleted channels should be included in the search. (archived channels) |  |
**page** | Option<**i64**> | The page to select. (Only works with Elasticsearch) |  |[default to 0]
**per_page** | Option<**i64**> | The number of posts per page. (Only works with Elasticsearch) |  |[default to 60]

### Return type

[**crate::models::FileInfoList**](FileInfoList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

