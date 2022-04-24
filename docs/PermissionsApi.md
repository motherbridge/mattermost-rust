# \PermissionsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ancillary_permissions**](PermissionsApi.md#get_ancillary_permissions) | **GET** /permissions/ancillary | Return all system console subsection ancillary permissions



## get_ancillary_permissions

> Vec<String> get_ancillary_permissions(subsection_permissions)
Return all system console subsection ancillary permissions

Returns all the ancillary permissions for the corresponding system console subsection permissions appended to the requested permission subsections.  __Minimum server version__: 5.35 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subsection_permissions** | Option<**String**> | The subsection permissions to return the ancillary permissions for. These values are comma seperated. Ex. subsection_permissions=sysconsole_read_reporting_site_statistics,sysconsole_write_reporting_site_statistics,sysconsole_write_user_management_channels  |  |

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

