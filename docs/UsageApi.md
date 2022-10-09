# \UsageApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_posts_usage**](UsageApi.md#get_posts_usage) | **GET** /usage/posts | Get current usage of posts
[**get_storage_usage**](UsageApi.md#get_storage_usage) | **GET** /usage/storage | Get the total file storage usage for the instance in bytes.



## get_posts_usage

> crate::models::PostsUsage get_posts_usage()
Get current usage of posts

Retrieve rounded off total no. of posts for this instance. Example: returns 4000 instead of 4321 ##### Permissions Must be authenticated. __Minimum server version__: 7.0 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PostsUsage**](PostsUsage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_usage

> crate::models::StorageUsage get_storage_usage()
Get the total file storage usage for the instance in bytes.

Get the total file storage usage for the instance in bytes rounded down to the most significant digit. Example: returns 4000 instead of 4321 ##### Permissions Must be authenticated. __Minimum server version__: 7.1 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StorageUsage**](StorageUsage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

