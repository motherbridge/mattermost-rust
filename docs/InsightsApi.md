# \InsightsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_new_team_members**](InsightsApi.md#get_new_team_members) | **GET** /teams/{team_id}/top/team_members | Get a list of new team members.
[**get_top_channels_for_team**](InsightsApi.md#get_top_channels_for_team) | **GET** /teams/{team_id}/top/channels | Get a list of the top channels for a team.
[**get_top_channels_for_user**](InsightsApi.md#get_top_channels_for_user) | **GET** /users/me/top/channels | Get a list of the top channels for a user.
[**get_top_dms_for_user**](InsightsApi.md#get_top_dms_for_user) | **GET** /users/me/top/dms | Get a list of the top dms for a user.
[**get_top_reactions_for_team**](InsightsApi.md#get_top_reactions_for_team) | **GET** /teams/{team_id}/top/reactions | Get a list of the top reactions for a team.
[**get_top_reactions_for_user**](InsightsApi.md#get_top_reactions_for_user) | **GET** /users/me/top/reactions | Get a list of the top reactions for a user.
[**get_top_threads_for_team**](InsightsApi.md#get_top_threads_for_team) | **GET** /teams/{team_id}/top/threads | Get a list of the top threads for a team.
[**get_top_threads_for_user**](InsightsApi.md#get_top_threads_for_user) | **GET** /users/me/top/threads | Get a list of the top threads for a user.



## get_new_team_members

> crate::models::NewTeamMembersList get_new_team_members(team_id, time_range, page, per_page)
Get a list of new team members.

Get a list of all of the new team members that have joined the given team during the given time period. ##### Permissions Must have `view_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: team members who joined during the current day. - `7_day`: team members who joined in the last 7 days. - `28_day`: team members who joined in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of items per page. |  |[default to 60]

### Return type

[**crate::models::NewTeamMembersList**](NewTeamMembersList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_channels_for_team

> crate::models::TopChannelList get_top_channels_for_team(team_id, time_range, page, per_page)
Get a list of the top channels for a team.

Get a list of the top public and private channels (the user is a member of) for a given team. ##### Permissions Must have `view_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: channels with posts on the current day. - `7_day`: channels with posts in the last 7 days. - `28_day`: channels with posts in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of items per page, up to a maximum of 200. |  |[default to 60]

### Return type

[**crate::models::TopChannelList**](TopChannelList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_channels_for_user

> crate::models::TopChannelList get_top_channels_for_user(time_range, page, per_page, team_id)
Get a list of the top channels for a user.

Get a list of the top public and private channels (the user is a member of) for a given user. ##### Permissions Must be logged in as the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: channels with posts on the current day. - `7_day`: channels with posts in the last 7 days. - `28_day`: channels with posts in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of items per page, up to a maximum of 200. |  |[default to 60]
**team_id** | Option<**String**> | Team ID will scope the response to a given team. ##### Permissions Must have `view_team` permission for the team.  |  |

### Return type

[**crate::models::TopChannelList**](TopChannelList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_dms_for_user

> crate::models::TopDmList get_top_dms_for_user(time_range, page, per_page)
Get a list of the top dms for a user.

Get a list of the top dms for a given user. ##### Permissions Must be logged in as the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: threads with activity on the current day. - `7_day`: threads with activity in the last 7 days. - `28_day`: threads with activity in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of items per page, up to a maximum of 200. |  |[default to 60]

### Return type

[**crate::models::TopDmList**](TopDMList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_reactions_for_team

> crate::models::TopReactionList get_top_reactions_for_team(team_id, time_range, page, per_page)
Get a list of the top reactions for a team.

Get a list of the top reactions across all public and private channels (the user is a member of) for a given team. ##### Permissions Must have `view_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: reactions posted on the current day. - `7_day`: reactions posted in the last 7 days. - `28_day`: reactions posted in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of items per page, up to a maximum of 200. |  |[default to 60]

### Return type

[**crate::models::TopReactionList**](TopReactionList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_reactions_for_user

> crate::models::TopReactionList get_top_reactions_for_user(time_range, page, per_page, team_id)
Get a list of the top reactions for a user.

Get a list of the top reactions across all public and private channels (the user is a member of) for a given user. If no `team_id` is provided, this will also include reactions posted by the given user in direct and group messages. ##### Permissions Must be logged in as the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: reactions posted on the current day. - `7_day`: reactions posted in the last 7 days. - `28_day`: reactions posted in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of items per page, up to a maximum of 200. |  |[default to 60]
**team_id** | Option<**String**> | Team ID will scope the response to a given team and exclude direct and group messages. ##### Permissions Must have `view_team` permission for the team.  |  |

### Return type

[**crate::models::TopReactionList**](TopReactionList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_threads_for_team

> crate::models::TopThreadList get_top_threads_for_team(team_id, time_range, page, per_page)
Get a list of the top threads for a team.

Get a list of the top threads from public and private channels (the user is a member of) for a given team. ##### Permissions Must have `view_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: threads with activity on the current day. - `7_day`: threads with activity in the last 7 days. - `28_day`: threads with activity in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of items per page, up to a maximum of 200. |  |[default to 60]

### Return type

[**crate::models::TopThreadList**](TopThreadList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_threads_for_user

> crate::models::TopThreadList get_top_threads_for_user(time_range, page, per_page, team_id)
Get a list of the top threads for a user.

Get a list of the top threads from public and private channels (the user is a member of and participating in the thread) for a given user. ##### Permissions Must be logged in as the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: threads with activity on the current day. - `7_day`: threads with activity in the last 7 days. - `28_day`: threads with activity in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of items per page, up to a maximum of 200. |  |[default to 60]
**team_id** | Option<**String**> | Team ID will scope the response to a given team. ##### Permissions Must have `view_team` permission for the team.  |  |

### Return type

[**crate::models::TopThreadList**](TopThreadList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

