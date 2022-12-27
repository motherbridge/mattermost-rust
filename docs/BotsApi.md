# \BotsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_bot**](BotsApi.md#assign_bot) | **POST** /bots/{bot_user_id}/assign/{user_id} | Assign a bot to a user
[**convert_bot_to_user**](BotsApi.md#convert_bot_to_user) | **POST** /bots/{bot_user_id}/convert_to_user | Convert a bot into a user
[**convert_user_to_bot**](BotsApi.md#convert_user_to_bot) | **POST** /users/{user_id}/convert_to_bot | Convert a user into a bot
[**create_bot**](BotsApi.md#create_bot) | **POST** /bots | Create a bot
[**delete_bot_icon_image**](BotsApi.md#delete_bot_icon_image) | **DELETE** /bots/{bot_user_id}/icon | Delete bot's LHS icon image
[**disable_bot**](BotsApi.md#disable_bot) | **POST** /bots/{bot_user_id}/disable | Disable a bot
[**enable_bot**](BotsApi.md#enable_bot) | **POST** /bots/{bot_user_id}/enable | Enable a bot
[**get_bot**](BotsApi.md#get_bot) | **GET** /bots/{bot_user_id} | Get a bot
[**get_bot_icon_image**](BotsApi.md#get_bot_icon_image) | **GET** /bots/{bot_user_id}/icon | Get bot's LHS icon
[**get_bots**](BotsApi.md#get_bots) | **GET** /bots | Get bots
[**patch_bot**](BotsApi.md#patch_bot) | **PUT** /bots/{bot_user_id} | Patch a bot
[**set_bot_icon_image**](BotsApi.md#set_bot_icon_image) | **POST** /bots/{bot_user_id}/icon | Set bot's LHS icon image



## assign_bot

> crate::models::Bot assign_bot(bot_user_id, user_id)
Assign a bot to a user

Assign a bot to a specified user. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**user_id** | **String** | The user ID to assign the bot to. | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_bot_to_user

> crate::models::StatusOk convert_bot_to_user(bot_user_id, convert_bot_to_user_request, set_system_admin)
Convert a bot into a user

Convert a bot into a user.  __Minimum server version__: 5.26  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**convert_bot_to_user_request** | [**ConvertBotToUserRequest**](ConvertBotToUserRequest.md) | Data to be used in the user creation | [required] |
**set_system_admin** | Option<**bool**> | Whether to give the user the system admin role. |  |[default to false]

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_user_to_bot

> crate::models::StatusOk convert_user_to_bot(user_id)
Convert a user into a bot

Convert a user into a bot.  __Minimum server version__: 5.26  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_bot

> crate::models::Bot create_bot(create_bot_request)
Create a bot

Create a new bot account on the system. Username is required. ##### Permissions Must have `create_bot` permission. __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_bot_request** | [**CreateBotRequest**](CreateBotRequest.md) | Bot to be created | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bot_icon_image

> crate::models::StatusOk delete_bot_icon_image(bot_user_id)
Delete bot's LHS icon image

Delete bot's LHS icon image based on bot_user_id string parameter. ##### Permissions Must have `manage_bots` permission. __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_bot

> crate::models::Bot disable_bot(bot_user_id)
Disable a bot

Disable a bot. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_bot

> crate::models::Bot enable_bot(bot_user_id)
Enable a bot

Enable a bot. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bot

> crate::models::Bot get_bot(bot_user_id, include_deleted)
Get a bot

Get a bot specified by its bot id. ##### Permissions Must have `read_bots` permission for bots you are managing, and `read_others_bots` permission for bots others are managing. __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**include_deleted** | Option<**bool**> | If deleted bots should be returned. |  |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bot_icon_image

> get_bot_icon_image(bot_user_id)
Get bot's LHS icon

Get a bot's LHS icon image based on bot_user_id string parameter. ##### Permissions Must be logged in. __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bots

> Vec<crate::models::Bot> get_bots(page, per_page, include_deleted, only_orphaned)
Get bots

Get a page of a list of bots. ##### Permissions Must have `read_bots` permission for bots you are managing, and `read_others_bots` permission for bots others are managing. __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of users per page. There is a maximum limit of 200 users per page. |  |[default to 60]
**include_deleted** | Option<**bool**> | If deleted bots should be returned. |  |
**only_orphaned** | Option<**bool**> | When true, only orphaned bots will be returned. A bot is consitered orphaned if it's owner has been deactivated. |  |

### Return type

[**Vec<crate::models::Bot>**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_bot

> crate::models::Bot patch_bot(bot_user_id, create_bot_request)
Patch a bot

Partially update a bot by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**create_bot_request** | [**CreateBotRequest**](CreateBotRequest.md) | Bot to be created | [required] |

### Return type

[**crate::models::Bot**](Bot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_bot_icon_image

> crate::models::StatusOk set_bot_icon_image(bot_user_id, image)
Set bot's LHS icon image

Set a bot's LHS icon image based on bot_user_id string parameter. Icon image must be SVG format, all other formats are rejected. ##### Permissions Must have `manage_bots` permission. __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_user_id** | **String** | Bot user ID | [required] |
**image** | **std::path::PathBuf** | SVG icon image to be uploaded | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

