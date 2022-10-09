# \SchemesApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_scheme**](SchemesApi.md#create_scheme) | **POST** /schemes | Create a scheme
[**delete_scheme**](SchemesApi.md#delete_scheme) | **DELETE** /schemes/{scheme_id} | Delete a scheme
[**get_channels_for_scheme**](SchemesApi.md#get_channels_for_scheme) | **GET** /schemes/{scheme_id}/channels | Get a page of channels which use this scheme.
[**get_scheme**](SchemesApi.md#get_scheme) | **GET** /schemes/{scheme_id} | Get a scheme
[**get_schemes**](SchemesApi.md#get_schemes) | **GET** /schemes | Get the schemes.
[**get_teams_for_scheme**](SchemesApi.md#get_teams_for_scheme) | **GET** /schemes/{scheme_id}/teams | Get a page of teams which use this scheme.
[**patch_scheme**](SchemesApi.md#patch_scheme) | **PUT** /schemes/{scheme_id}/patch | Patch a scheme



## create_scheme

> crate::models::Scheme create_scheme(inline_object104)
Create a scheme

Create a new scheme.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object104** | [**InlineObject104**](InlineObject104.md) |  | [required] |

### Return type

[**crate::models::Scheme**](Scheme.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scheme

> crate::models::StatusOk delete_scheme(scheme_id)
Delete a scheme

Soft deletes a scheme, by marking the scheme as deleted in the database.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | ID of the scheme to delete | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channels_for_scheme

> Vec<crate::models::Channel> get_channels_for_scheme(scheme_id, page, per_page)
Get a page of channels which use this scheme.

Get a page of channels which use this scheme. The provided Scheme ID should be for a Channel-scoped Scheme. Use the query parameters to modify the behaviour of this endpoint.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | Scheme GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of channels per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scheme

> crate::models::Scheme get_scheme(scheme_id)
Get a scheme

Get a scheme from the provided scheme id.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | Scheme GUID | [required] |

### Return type

[**crate::models::Scheme**](Scheme.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schemes

> Vec<crate::models::Scheme> get_schemes(scope, page, per_page)
Get the schemes.

Get a page of schemes. Use the query parameters to modify the behaviour of this endpoint.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | Option<**String**> | Limit the results returned to the provided scope, either `team` or `channel`. |  |[default to ]
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of schemes per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Scheme>**](Scheme.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_for_scheme

> Vec<crate::models::Team> get_teams_for_scheme(scheme_id, page, per_page)
Get a page of teams which use this scheme.

Get a page of teams which use this scheme. The provided Scheme ID should be for a Team-scoped Scheme. Use the query parameters to modify the behaviour of this endpoint.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | Scheme GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of teams per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_scheme

> crate::models::Scheme patch_scheme(scheme_id, inline_object105)
Patch a scheme

Partially update a scheme by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 5.0 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheme_id** | **String** | Scheme GUID | [required] |
**inline_object105** | [**InlineObject105**](InlineObject105.md) |  | [required] |

### Return type

[**crate::models::Scheme**](Scheme.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

