# \CommandsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_command**](CommandsApi.md#create_command) | **POST** /commands | Create a command
[**delete_command**](CommandsApi.md#delete_command) | **DELETE** /commands/{command_id} | Delete a command
[**execute_command**](CommandsApi.md#execute_command) | **POST** /commands/execute | Execute a command
[**get_command_by_id**](CommandsApi.md#get_command_by_id) | **GET** /commands/{command_id} | Get a command
[**list_autocomplete_commands**](CommandsApi.md#list_autocomplete_commands) | **GET** /teams/{team_id}/commands/autocomplete | List autocomplete commands
[**list_command_autocomplete_suggestions**](CommandsApi.md#list_command_autocomplete_suggestions) | **GET** /teams/{team_id}/commands/autocomplete_suggestions | List commands' autocomplete data
[**list_commands**](CommandsApi.md#list_commands) | **GET** /commands | List commands for a team
[**move_command**](CommandsApi.md#move_command) | **PUT** /commands/{command_id}/move | Move a command
[**regen_command_token**](CommandsApi.md#regen_command_token) | **PUT** /commands/{command_id}/regen_token | Generate a new token
[**update_command**](CommandsApi.md#update_command) | **PUT** /commands/{command_id} | Update a command



## create_command

> crate::models::Command create_command(inline_object94)
Create a command

Create a command for a team. ##### Permissions `manage_slash_commands` for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object94** | [**InlineObject94**](InlineObject94.md) |  | [required] |

### Return type

[**crate::models::Command**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_command

> crate::models::StatusOk delete_command(command_id)
Delete a command

Delete a command based on command id string. ##### Permissions Must have `manage_slash_commands` permission for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to delete | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_command

> crate::models::CommandResponse execute_command(inline_object96)
Execute a command

Execute a command on a team. ##### Permissions Must have `use_slash_commands` permission for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object96** | [**InlineObject96**](InlineObject96.md) |  | [required] |

### Return type

[**crate::models::CommandResponse**](CommandResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_command_by_id

> crate::models::Command get_command_by_id(command_id)
Get a command

Get a command definition based on command id string. ##### Permissions Must have `manage_slash_commands` permission for the team the command is in.  __Minimum server version__: 5.22 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to get | [required] |

### Return type

[**crate::models::Command**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_autocomplete_commands

> Vec<crate::models::Command> list_autocomplete_commands(team_id)
List autocomplete commands

List autocomplete commands in the team. ##### Permissions `view_team` for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |

### Return type

[**Vec<crate::models::Command>**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_command_autocomplete_suggestions

> Vec<crate::models::AutocompleteSuggestion> list_command_autocomplete_suggestions(team_id, user_input)
List commands' autocomplete data

List commands' autocomplete data for the team. ##### Permissions `view_team` for the team. __Minimum server version__: 5.24 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_input** | **String** | String inputted by the user. | [required] |

### Return type

[**Vec<crate::models::AutocompleteSuggestion>**](AutocompleteSuggestion.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_commands

> Vec<crate::models::Command> list_commands(team_id, custom_only)
List commands for a team

List commands for a team. ##### Permissions `manage_slash_commands` if need list custom commands. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | Option<**String**> | The team id. |  |
**custom_only** | Option<**bool**> | To get only the custom commands. If set to false will get the custom if the user have access plus the system commands, otherwise just the system commands.  |  |[default to false]

### Return type

[**Vec<crate::models::Command>**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_command

> crate::models::StatusOk move_command(command_id, inline_object95)
Move a command

Move a command to a different team based on command id string. ##### Permissions Must have `manage_slash_commands` permission for the team the command is currently in and the destination team.  __Minimum server version__: 5.22 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to move | [required] |
**inline_object95** | [**InlineObject95**](InlineObject95.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regen_command_token

> crate::models::InlineResponse20015 regen_command_token(command_id)
Generate a new token

Generate a new token for the command based on command id string. ##### Permissions Must have `manage_slash_commands` permission for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to generate the new token | [required] |

### Return type

[**crate::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_command

> crate::models::Command update_command(command_id, command)
Update a command

Update a single command based on command id string and Command struct. ##### Permissions Must have `manage_slash_commands` permission for the team the command is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_id** | **String** | ID of the command to update | [required] |
**command** | [**Command**](Command.md) |  | [required] |

### Return type

[**crate::models::Command**](Command.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

