# \RootApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**acknowledge_notification**](RootApi.md#acknowledge_notification) | **POST** /notifications/ack | Acknowledge receiving of a notification



## acknowledge_notification

> crate::models::PushNotification acknowledge_notification()
Acknowledge receiving of a notification

__Minimum server version__: 3.10 ##### Permissions Must be logged in. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PushNotification**](PushNotification.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

