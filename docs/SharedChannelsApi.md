# \SharedChannelsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_shared_channels**](SharedChannelsApi.md#get_all_shared_channels) | **GET** /sharedchannels/{team_id} | Get all shared channels for team.
[**get_remote_cluster_info**](SharedChannelsApi.md#get_remote_cluster_info) | **GET** /sharedchannels/remote_info/{remote_id} | Get remote cluster info by ID for user.



## get_all_shared_channels

> Vec<crate::models::SharedChannel> get_all_shared_channels(team_id, page, per_page)
Get all shared channels for team.

Get all shared channels for a team.  __Minimum server version__: 5.50  ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team Id | [required] |
**page** | Option<**i32**> |  |  |[default to 0]
**per_page** | Option<**i32**> |  |  |[default to 0]

### Return type

[**Vec<crate::models::SharedChannel>**](SharedChannel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_remote_cluster_info

> crate::models::RemoteClusterInfo get_remote_cluster_info(remote_id)
Get remote cluster info by ID for user.

Get remote cluster info based on remoteId.  __Minimum server version__: 5.50  ##### Permissions Must be authenticated and user must belong to at least one channel shared with the remote cluster. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote_id** | **String** | Remote Cluster GUID | [required] |

### Return type

[**crate::models::RemoteClusterInfo**](RemoteClusterInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

