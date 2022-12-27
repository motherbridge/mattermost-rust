# \StatusApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_status**](StatusApi.md#get_user_status) | **GET** /users/{user_id}/status | Get user status
[**get_users_statuses_by_ids**](StatusApi.md#get_users_statuses_by_ids) | **POST** /users/status/ids | Get user statuses by id
[**post_user_recent_custom_status_delete**](StatusApi.md#post_user_recent_custom_status_delete) | **POST** /users/{user_id}/status/custom/recent/delete | Delete user's recent custom status
[**remove_recent_custom_status**](StatusApi.md#remove_recent_custom_status) | **DELETE** /users/{user_id}/status/custom/recent | Delete user's recent custom status
[**unset_user_custom_status**](StatusApi.md#unset_user_custom_status) | **DELETE** /users/{user_id}/status/custom | Unsets user custom status
[**update_user_custom_status**](StatusApi.md#update_user_custom_status) | **PUT** /users/{user_id}/status/custom | Update user custom status
[**update_user_status**](StatusApi.md#update_user_status) | **PUT** /users/{user_id}/status | Update user status



## get_user_status

> crate::models::Status get_user_status(user_id)
Get user status

Get user status by id from the server. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_statuses_by_ids

> Vec<crate::models::Status> get_users_statuses_by_ids(request_body)
Get user statuses by id

Get a list of user statuses by id from the server. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | List of user ids to fetch | [required] |

### Return type

[**Vec<crate::models::Status>**](Status.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user_recent_custom_status_delete

> post_user_recent_custom_status_delete(user_id, remove_recent_custom_status_request)
Delete user's recent custom status

Deletes a user's recent custom status by removing the specific status from the recentCustomStatuses in the user's props and updates the user. ##### Permissions Must be logged in as the user whose recent custom status is being deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**remove_recent_custom_status_request** | [**RemoveRecentCustomStatusRequest**](RemoveRecentCustomStatusRequest.md) | Custom Status object that is to be removed from the recent custom statuses. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_recent_custom_status

> remove_recent_custom_status(user_id, remove_recent_custom_status_request)
Delete user's recent custom status

Deletes a user's recent custom status by removing the specific status from the recentCustomStatuses in the user's props and updates the user. ##### Permissions Must be logged in as the user whose recent custom status is being deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**remove_recent_custom_status_request** | [**RemoveRecentCustomStatusRequest**](RemoveRecentCustomStatusRequest.md) | Custom Status object that is to be removed from the recent custom statuses. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unset_user_custom_status

> unset_user_custom_status(user_id)
Unsets user custom status

Unsets a user's custom status by updating the user's props and updates the user ##### Permissions Must be logged in as the user whose custom status is being removed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_custom_status

> update_user_custom_status(user_id, update_user_custom_status_request)
Update user custom status

Updates a user's custom status by setting the value in the user's props and updates the user. Also save the given custom status to the recent custom statuses in the user's props ##### Permissions Must be logged in as the user whose custom status is being updated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**update_user_custom_status_request** | [**UpdateUserCustomStatusRequest**](UpdateUserCustomStatusRequest.md) | Custom status object that is to be updated | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_status

> crate::models::Status update_user_status(user_id, update_user_status_request)
Update user status

Manually set a user's status. When setting a user's status, the status will remain that value until set \"online\" again, which will return the status to being automatically updated based on user activity. ##### Permissions Must have `edit_other_users` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**update_user_status_request** | [**UpdateUserStatusRequest**](UpdateUserStatusRequest.md) | Status object that is to be updated | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

