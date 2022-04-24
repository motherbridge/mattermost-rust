# \JobsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_job**](JobsApi.md#cancel_job) | **POST** /jobs/{job_id}/cancel | Cancel a job.
[**create_job**](JobsApi.md#create_job) | **POST** /jobs | Create a new job.
[**download_job**](JobsApi.md#download_job) | **GET** /jobs/{job_id}/download | Download the results of a job.
[**get_job**](JobsApi.md#get_job) | **GET** /jobs/{job_id} | Get a job.
[**get_jobs**](JobsApi.md#get_jobs) | **GET** /jobs | Get the jobs.
[**get_jobs_by_type**](JobsApi.md#get_jobs_by_type) | **GET** /jobs/type/{type} | Get the jobs of the given type.



## cancel_job

> crate::models::StatusOk cancel_job(job_id)
Cancel a job.

Cancel a job. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job

> crate::models::Job create_job(inline_object66)
Create a new job.

Create a new job. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object66** | [**InlineObject66**](InlineObject66.md) |  | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_job

> download_job(job_id)
Download the results of a job.

Download the result of a single job. __Minimum server version: 5.28__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job GUID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job

> crate::models::Job get_job(job_id)
Get a job.

Gets a single job. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job GUID | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jobs

> Vec<crate::models::Job> get_jobs(page, per_page)
Get the jobs.

Get a page of jobs. Use the query parameters to modify the behaviour of this endpoint. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of jobs per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Job>**](Job.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jobs_by_type

> Vec<crate::models::Job> get_jobs_by_type(_type, page, per_page)
Get the jobs of the given type.

Get a page of jobs of the given type. Use the query parameters to modify the behaviour of this endpoint. __Minimum server version: 4.1__ ##### Permissions Must have `manage_jobs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | Job type | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of jobs per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Job>**](Job.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

