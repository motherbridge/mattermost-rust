# \TermsOfServiceApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_terms_of_service**](TermsOfServiceApi.md#create_terms_of_service) | **POST** /terms_of_service | Creates a new terms of service
[**get_terms_of_service**](TermsOfServiceApi.md#get_terms_of_service) | **GET** /terms_of_service | Get latest terms of service
[**get_user_terms_of_service**](TermsOfServiceApi.md#get_user_terms_of_service) | **GET** /users/{user_id}/terms_of_service | Fetches user's latest terms of service action if the latest action was for acceptance.
[**register_terms_of_service_action**](TermsOfServiceApi.md#register_terms_of_service_action) | **POST** /users/{user_id}/terms_of_service | Records user action when they accept or decline custom terms of service



## create_terms_of_service

> crate::models::TermsOfService create_terms_of_service()
Creates a new terms of service

Creates new terms of service  __Minimum server version__: 5.4 ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TermsOfService**](TermsOfService.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terms_of_service

> crate::models::TermsOfService get_terms_of_service()
Get latest terms of service

Get latest terms of service from the server  __Minimum server version__: 5.4 ##### Permissions Must be authenticated. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TermsOfService**](TermsOfService.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_terms_of_service

> crate::models::UserTermsOfService get_user_terms_of_service(user_id)
Fetches user's latest terms of service action if the latest action was for acceptance.

Will be deprecated in v6.0 Fetches user's latest terms of service action if the latest action was for acceptance.  __Minimum server version__: 5.6 ##### Permissions Must be logged in as the user being acted on. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::UserTermsOfService**](UserTermsOfService.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_terms_of_service_action

> crate::models::StatusOk register_terms_of_service_action(user_id, inline_object24)
Records user action when they accept or decline custom terms of service

Records user action when they accept or decline custom terms of service. Records the action in audit table. Updates user's last accepted terms of service ID if they accepted it.  __Minimum server version__: 5.4 ##### Permissions Must be logged in as the user being acted on. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**inline_object24** | [**InlineObject24**](InlineObject24.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

