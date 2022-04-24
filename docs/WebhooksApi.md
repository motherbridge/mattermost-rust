# \WebhooksApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_incoming_webhook**](WebhooksApi.md#create_incoming_webhook) | **POST** /hooks/incoming | Create an incoming webhook
[**create_outgoing_webhook**](WebhooksApi.md#create_outgoing_webhook) | **POST** /hooks/outgoing | Create an outgoing webhook
[**delete_incoming_webhook**](WebhooksApi.md#delete_incoming_webhook) | **DELETE** /hooks/incoming/{hook_id} | Delete an incoming webhook
[**delete_outgoing_webhook**](WebhooksApi.md#delete_outgoing_webhook) | **DELETE** /hooks/outgoing/{hook_id} | Delete an outgoing webhook
[**get_incoming_webhook**](WebhooksApi.md#get_incoming_webhook) | **GET** /hooks/incoming/{hook_id} | Get an incoming webhook
[**get_incoming_webhooks**](WebhooksApi.md#get_incoming_webhooks) | **GET** /hooks/incoming | List incoming webhooks
[**get_outgoing_webhook**](WebhooksApi.md#get_outgoing_webhook) | **GET** /hooks/outgoing/{hook_id} | Get an outgoing webhook
[**get_outgoing_webhooks**](WebhooksApi.md#get_outgoing_webhooks) | **GET** /hooks/outgoing | List outgoing webhooks
[**regen_outgoing_hook_token**](WebhooksApi.md#regen_outgoing_hook_token) | **POST** /hooks/outgoing/{hook_id}/regen_token | Regenerate the token for the outgoing webhook.
[**update_incoming_webhook**](WebhooksApi.md#update_incoming_webhook) | **PUT** /hooks/incoming/{hook_id} | Update an incoming webhook
[**update_outgoing_webhook**](WebhooksApi.md#update_outgoing_webhook) | **PUT** /hooks/outgoing/{hook_id} | Update an outgoing webhook



## create_incoming_webhook

> crate::models::IncomingWebhook create_incoming_webhook(inline_object74)
Create an incoming webhook

Create an incoming webhook for a channel. ##### Permissions `manage_webhooks` for the team the webhook is in.  `manage_others_incoming_webhooks` for the team the webhook is in if the user is different than the requester. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object74** | [**InlineObject74**](InlineObject74.md) |  | [required] |

### Return type

[**crate::models::IncomingWebhook**](IncomingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_outgoing_webhook

> crate::models::OutgoingWebhook create_outgoing_webhook(inline_object76)
Create an outgoing webhook

Create an outgoing webhook for a team. ##### Permissions `manage_webhooks` for the team the webhook is in.  `manage_others_outgoing_webhooks` for the team the webhook is in if the user is different than the requester. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object76** | [**InlineObject76**](InlineObject76.md) |  | [required] |

### Return type

[**crate::models::OutgoingWebhook**](OutgoingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_incoming_webhook

> crate::models::StatusOk delete_incoming_webhook(hook_id)
Delete an incoming webhook

Delete an incoming webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Incoming webhook GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outgoing_webhook

> crate::models::StatusOk delete_outgoing_webhook(hook_id)
Delete an outgoing webhook

Delete an outgoing webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Outgoing webhook GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_incoming_webhook

> crate::models::IncomingWebhook get_incoming_webhook(hook_id)
Get an incoming webhook

Get an incoming webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Incoming Webhook GUID | [required] |

### Return type

[**crate::models::IncomingWebhook**](IncomingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_incoming_webhooks

> Vec<crate::models::IncomingWebhook> get_incoming_webhooks(page, per_page, team_id)
List incoming webhooks

Get a page of a list of incoming webhooks. Optionally filter for a specific team using query parameters. ##### Permissions `manage_webhooks` for the system or `manage_webhooks` for the specific team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of hooks per page. |  |[default to 60]
**team_id** | Option<**String**> | The ID of the team to get hooks for. |  |

### Return type

[**Vec<crate::models::IncomingWebhook>**](IncomingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outgoing_webhook

> crate::models::OutgoingWebhook get_outgoing_webhook(hook_id)
Get an outgoing webhook

Get an outgoing webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Outgoing webhook GUID | [required] |

### Return type

[**crate::models::OutgoingWebhook**](OutgoingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outgoing_webhooks

> Vec<crate::models::OutgoingWebhook> get_outgoing_webhooks(page, per_page, team_id, channel_id)
List outgoing webhooks

Get a page of a list of outgoing webhooks. Optionally filter for a specific team or channel using query parameters. ##### Permissions `manage_webhooks` for the system or `manage_webhooks` for the specific team/channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of hooks per page. |  |[default to 60]
**team_id** | Option<**String**> | The ID of the team to get hooks for. |  |
**channel_id** | Option<**String**> | The ID of the channel to get hooks for. |  |

### Return type

[**Vec<crate::models::OutgoingWebhook>**](OutgoingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regen_outgoing_hook_token

> crate::models::StatusOk regen_outgoing_hook_token(hook_id)
Regenerate the token for the outgoing webhook.

Regenerate the token for the outgoing webhook. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Outgoing webhook GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_incoming_webhook

> crate::models::IncomingWebhook update_incoming_webhook(hook_id, inline_object75)
Update an incoming webhook

Update an incoming webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | Incoming Webhook GUID | [required] |
**inline_object75** | [**InlineObject75**](InlineObject75.md) |  | [required] |

### Return type

[**crate::models::IncomingWebhook**](IncomingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_outgoing_webhook

> crate::models::OutgoingWebhook update_outgoing_webhook(hook_id, inline_object77)
Update an outgoing webhook

Update an outgoing webhook given the hook id. ##### Permissions `manage_webhooks` for system or `manage_webhooks` for the specific team or `manage_webhooks` for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hook_id** | **String** | outgoing Webhook GUID | [required] |
**inline_object77** | [**InlineObject77**](InlineObject77.md) |  | [required] |

### Return type

[**crate::models::OutgoingWebhook**](OutgoingWebhook.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

