# \RolesApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_role**](RolesApi.md#get_role) | **GET** /roles/{role_id} | Get a role
[**get_role_by_name**](RolesApi.md#get_role_by_name) | **GET** /roles/name/{role_name} | Get a role
[**get_roles_by_names**](RolesApi.md#get_roles_by_names) | **POST** /roles/names | Get a list of roles by name
[**patch_role**](RolesApi.md#patch_role) | **PUT** /roles/{role_id}/patch | Patch a role
[**roles_get**](RolesApi.md#roles_get) | **GET** /roles | Get a list of all the roles



## get_role

> crate::models::Role get_role(role_id)
Get a role

Get a role from the provided role id.  ##### Permissions Requires an active session but no other permissions.  __Minimum server version__: 4.9 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role GUID | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_by_name

> crate::models::Role get_role_by_name(role_name)
Get a role

Get a role from the provided role name.  ##### Permissions Requires an active session but no other permissions.  __Minimum server version__: 4.9 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** | Role Name | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles_by_names

> Vec<crate::models::Role> get_roles_by_names(request_body)
Get a list of roles by name

Get a list of roles from their names.  ##### Permissions Requires an active session but no other permissions.  __Minimum server version__: 4.9 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | List of role names | [required] |

### Return type

[**Vec<crate::models::Role>**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_role

> crate::models::Role patch_role(role_id, inline_object102)
Patch a role

Partially update a role by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 4.9 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role GUID | [required] |
**inline_object102** | [**InlineObject102**](InlineObject102.md) |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## roles_get

> Vec<crate::models::Role> roles_get()
Get a list of all the roles

##### Permissions  `manage_system` permission is required.  __Minimum server version__: 5.33 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Role>**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

