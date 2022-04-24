# \BleveApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**purge_bleve_indexes**](BleveApi.md#purge_bleve_indexes) | **POST** /bleve/purge_indexes | Purge all Bleve indexes



## purge_bleve_indexes

> crate::models::StatusOk purge_bleve_indexes()
Purge all Bleve indexes

Deletes all Bleve indexes and their contents. After calling this endpoint, it is necessary to schedule a new Bleve indexing job to repopulate the indexes. __Minimum server version__: 5.24 ##### Permissions Must have `sysconsole_write_experimental` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

