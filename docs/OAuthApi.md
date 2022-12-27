# \OAuthApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_o_auth_app**](OAuthApi.md#create_o_auth_app) | **POST** /oauth/apps | Register OAuth app
[**delete_o_auth_app**](OAuthApi.md#delete_o_auth_app) | **DELETE** /oauth/apps/{app_id} | Delete an OAuth app
[**get_authorized_o_auth_apps_for_user**](OAuthApi.md#get_authorized_o_auth_apps_for_user) | **GET** /users/{user_id}/oauth/apps/authorized | Get authorized OAuth apps
[**get_o_auth_app**](OAuthApi.md#get_o_auth_app) | **GET** /oauth/apps/{app_id} | Get an OAuth app
[**get_o_auth_app_info**](OAuthApi.md#get_o_auth_app_info) | **GET** /oauth/apps/{app_id}/info | Get info on an OAuth app
[**get_o_auth_apps**](OAuthApi.md#get_o_auth_apps) | **GET** /oauth/apps | Get OAuth apps
[**regenerate_o_auth_app_secret**](OAuthApi.md#regenerate_o_auth_app_secret) | **POST** /oauth/apps/{app_id}/regen_secret | Regenerate OAuth app secret
[**update_o_auth_app**](OAuthApi.md#update_o_auth_app) | **PUT** /oauth/apps/{app_id} | Update an OAuth app



## create_o_auth_app

> crate::models::OAuthApp create_o_auth_app(create_o_auth_app_request)
Register OAuth app

Register an OAuth 2.0 client application with Mattermost as the service provider. ##### Permissions Must have `manage_oauth` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_o_auth_app_request** | [**CreateOAuthAppRequest**](CreateOAuthAppRequest.md) | OAuth application to register | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_o_auth_app

> crate::models::StatusOk delete_o_auth_app(app_id)
Delete an OAuth app

Delete and unregister an OAuth 2.0 client application  ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorized_o_auth_apps_for_user

> Vec<crate::models::OAuthApp> get_authorized_o_auth_apps_for_user(user_id, page, per_page)
Get authorized OAuth apps

Get a page of OAuth 2.0 client applications authorized to access a user's account. ##### Permissions Must be authenticated as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of apps per page. |  |[default to 60]

### Return type

[**Vec<crate::models::OAuthApp>**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_o_auth_app

> crate::models::OAuthApp get_o_auth_app(app_id)
Get an OAuth app

Get an OAuth 2.0 client application registered with Mattermost. ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_o_auth_app_info

> crate::models::OAuthApp get_o_auth_app_info(app_id)
Get info on an OAuth app

Get public information about an OAuth 2.0 client application registered with Mattermost. The application's client secret will be blanked out. ##### Permissions Must be authenticated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_o_auth_apps

> Vec<crate::models::OAuthApp> get_o_auth_apps(page, per_page)
Get OAuth apps

Get a page of OAuth 2.0 client applications registered with Mattermost. ##### Permissions With `manage_oauth` permission, the apps registered by the logged in user are returned. With `manage_system_wide_oauth` permission, all apps regardless of creator are returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of apps per page. |  |[default to 60]

### Return type

[**Vec<crate::models::OAuthApp>**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regenerate_o_auth_app_secret

> crate::models::OAuthApp regenerate_o_auth_app_secret(app_id)
Regenerate OAuth app secret

Regenerate the client secret for an OAuth 2.0 client application registered with Mattermost. ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_o_auth_app

> crate::models::OAuthApp update_o_auth_app(app_id, update_o_auth_app_request)
Update an OAuth app

Update an OAuth 2.0 client application based on OAuth struct. ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Application client id | [required] |
**update_o_auth_app_request** | [**UpdateOAuthAppRequest**](UpdateOAuthAppRequest.md) | OAuth application to update | [required] |

### Return type

[**crate::models::OAuthApp**](OAuthApp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

