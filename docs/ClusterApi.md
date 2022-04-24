# \ClusterApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cluster_status**](ClusterApi.md#get_cluster_status) | **GET** /cluster/status | Get cluster status



## get_cluster_status

> Vec<crate::models::ClusterInfo> get_cluster_status()
Get cluster status

Get a set of information for each node in the cluster, useful for checking the status and health of each node. ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ClusterInfo>**](ClusterInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

