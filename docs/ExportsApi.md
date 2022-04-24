# \ExportsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_export**](ExportsApi.md#delete_export) | **DELETE** /exports/{export_name} | Delete an export file
[**download_export**](ExportsApi.md#download_export) | **GET** /exports/{export_name} | Download an export file
[**list_exports**](ExportsApi.md#list_exports) | **GET** /exports | List export files



## delete_export

> delete_export(export_name)
Delete an export file

Deletes an export file.   __Minimum server version__: 5.33  ##### Permissions  Must have `manage_system` permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_name** | **String** | The name of the export file to delete | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_export

> download_export(export_name)
Download an export file

Downloads an export file.   __Minimum server version__: 5.33  ##### Permissions  Must have `manage_system` permissions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_name** | **String** | The name of the export file to download | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_exports

> list_exports()
List export files

Lists all available export files. __Minimum server version__: 5.33 ##### Permissions Must have `manage_system` permissions. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

