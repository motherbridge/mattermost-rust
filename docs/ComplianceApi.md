# \ComplianceApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_compliance_report**](ComplianceApi.md#create_compliance_report) | **POST** /compliance/reports | Create report
[**download_compliance_report**](ComplianceApi.md#download_compliance_report) | **GET** /compliance/reports/{report_id}/download | Download a report
[**get_compliance_report**](ComplianceApi.md#get_compliance_report) | **GET** /compliance/reports/{report_id} | Get a report
[**get_compliance_reports**](ComplianceApi.md#get_compliance_reports) | **GET** /compliance/reports | Get reports



## create_compliance_report

> crate::models::Compliance create_compliance_report()
Create report

Create and save a compliance report. ##### Permissions Must have `manage_system` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Compliance**](Compliance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_compliance_report

> download_compliance_report(report_id)
Download a report

Download the full contents of a report as a file. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | Compliance report GUID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compliance_report

> crate::models::Compliance get_compliance_report(report_id)
Get a report

Get a compliance reports previously created. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | Compliance report GUID | [required] |

### Return type

[**crate::models::Compliance**](Compliance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compliance_reports

> Vec<crate::models::Compliance> get_compliance_reports(page, per_page)
Get reports

Get a list of compliance reports previously created by page, selected with `page` and `per_page` query parameters. ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of reports per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Compliance>**](Compliance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

