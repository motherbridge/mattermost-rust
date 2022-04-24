# \ThreadsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_thread_mention_counts_by_channel**](ThreadsApi.md#get_thread_mention_counts_by_channel) | **GET** /users/{user_id}/teams/{team_id}/threads/mention_counts | Get all unread mention counts from followed threads, per-channel
[**get_user_thread**](ThreadsApi.md#get_user_thread) | **GET** /users/{user_id}/teams/{team_id}/threads/{thread_id} | Get a thread followed by the user
[**get_user_threads**](ThreadsApi.md#get_user_threads) | **GET** /users/{user_id}/teams/{team_id}/threads | Get all threads that user is following
[**start_following_thread**](ThreadsApi.md#start_following_thread) | **PUT** /users/{user_id}/teams/{team_id}/threads/{thread_id}/following | Start following a thread
[**stop_following_thread**](ThreadsApi.md#stop_following_thread) | **DELETE** /users/{user_id}/teams/{team_id}/threads/{thread_id}/following | Stop following a thread
[**update_thread_read_for_user**](ThreadsApi.md#update_thread_read_for_user) | **PUT** /users/{user_id}/teams/{team_id}/threads/{thread_id}/read/{timestamp} | Mark a thread that user is following read state to the timestamp
[**update_threads_read_for_user**](ThreadsApi.md#update_threads_read_for_user) | **PUT** /users/{user_id}/teams/{team_id}/threads/read | Mark all threads that user is following as read



## get_thread_mention_counts_by_channel

> get_thread_mention_counts_by_channel(user_id, team_id)
Get all unread mention counts from followed threads, per-channel

Get all unread mention counts from followed threads  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**team_id** | **String** | The ID of the team in which the thread is. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_thread

> get_user_thread(user_id, team_id, thread_id)
Get a thread followed by the user

Get a thread  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**team_id** | **String** | The ID of the team in which the thread is. | [required] |
**thread_id** | **String** | The ID of the thread to follow | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_threads

> crate::models::UserThreads get_user_threads(user_id, team_id, since, deleted, extended, page, page_size, totals_only)
Get all threads that user is following

Get all threads that user is following  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**team_id** | **String** | The ID of the team in which the thread is. | [required] |
**since** | Option<**i64**> | Since filters the threads based on their LastUpdateAt timestamp. |  |
**deleted** | Option<**bool**> | Deleted will specify that even deleted threads should be returned (For mobile sync). |  |[default to false]
**extended** | Option<**bool**> | Extended will enrich the response with participant details. |  |[default to false]
**page** | Option<**i64**> | Page specifies which part of the results to return, by PageSize. |  |[default to 0]
**page_size** | Option<**i64**> | PageSize specifies the size of the returned chunk of results. |  |[default to 30]
**totals_only** | Option<**bool**> | Setting this to true will only return the total counts. |  |[default to false]

### Return type

[**crate::models::UserThreads**](UserThreads.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_following_thread

> start_following_thread(user_id, team_id, thread_id)
Start following a thread

Start following a thread  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**team_id** | **String** | The ID of the team in which the thread is. | [required] |
**thread_id** | **String** | The ID of the thread to follow | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_following_thread

> stop_following_thread(user_id, team_id, thread_id)
Stop following a thread

Stop following a thread  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**team_id** | **String** | The ID of the team in which the thread is. | [required] |
**thread_id** | **String** | The ID of the thread to update | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_thread_read_for_user

> update_thread_read_for_user(user_id, team_id, thread_id, timestamp)
Mark a thread that user is following read state to the timestamp

Mark a thread that user is following as read  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**team_id** | **String** | The ID of the team in which the thread is. | [required] |
**thread_id** | **String** | The ID of the thread to update | [required] |
**timestamp** | **String** | The timestamp to which the thread's \"last read\" state will be reset. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_threads_read_for_user

> update_threads_read_for_user(user_id, team_id)
Mark all threads that user is following as read

Mark all threads that user is following as read  __Minimum server version__: 5.29  ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**team_id** | **String** | The ID of the team in which the thread is. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

