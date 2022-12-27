# \IntegrationActionsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**open_interactive_dialog**](IntegrationActionsApi.md#open_interactive_dialog) | **POST** /actions/dialogs/open | Open a dialog
[**submit_interactive_dialog**](IntegrationActionsApi.md#submit_interactive_dialog) | **POST** /actions/dialogs/submit | Submit a dialog



## open_interactive_dialog

> crate::models::StatusOk open_interactive_dialog(open_interactive_dialog_request)
Open a dialog

Open an interactive dialog using a trigger ID provided by a slash command, or some other action payload. See https://docs.mattermost.com/developer/interactive-dialogs.html for more information on interactive dialogs. __Minimum server version: 5.6__ 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**open_interactive_dialog_request** | [**OpenInteractiveDialogRequest**](OpenInteractiveDialogRequest.md) | Metadata for the dialog to be opened | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_interactive_dialog

> crate::models::StatusOk submit_interactive_dialog(submit_interactive_dialog_request)
Submit a dialog

Endpoint used by the Mattermost clients to submit a dialog. See https://docs.mattermost.com/developer/interactive-dialogs.html for more information on interactive dialogs. __Minimum server version: 5.6__ 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_interactive_dialog_request** | [**SubmitInteractiveDialogRequest**](SubmitInteractiveDialogRequest.md) | Dialog submission data | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

