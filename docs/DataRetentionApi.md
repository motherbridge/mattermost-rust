# \DataRetentionApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_channels_to_retention_policy**](DataRetentionApi.md#add_channels_to_retention_policy) | **POST** /data_retention/policies/{policy_id}/channels | Add channels to a granular data retention policy
[**add_teams_to_retention_policy**](DataRetentionApi.md#add_teams_to_retention_policy) | **POST** /data_retention/policies/{policy_id}/teams | Add teams to a granular data retention policy
[**create_data_retention_policy**](DataRetentionApi.md#create_data_retention_policy) | **POST** /data_retention/policies | Create a new granular data retention policy
[**delete_data_retention_policy**](DataRetentionApi.md#delete_data_retention_policy) | **DELETE** /data_retention/policies/{policy_id} | Delete a granular data retention policy
[**get_channel_policies_for_user**](DataRetentionApi.md#get_channel_policies_for_user) | **GET** /users/{user_id}/data_retention/channel_policies | Get the policies which are applied to a user's channels
[**get_channels_for_retention_policy**](DataRetentionApi.md#get_channels_for_retention_policy) | **GET** /data_retention/policies/{policy_id}/channels | Get the channels for a granular data retention policy
[**get_data_retention_policies**](DataRetentionApi.md#get_data_retention_policies) | **GET** /data_retention/policies | Get the granular data retention policies
[**get_data_retention_policies_count**](DataRetentionApi.md#get_data_retention_policies_count) | **GET** /data_retention/policies_count | Get the number of granular data retention policies
[**get_data_retention_policy**](DataRetentionApi.md#get_data_retention_policy) | **GET** /data_retention/policy | Get the global data retention policy
[**get_data_retention_policy_by_id**](DataRetentionApi.md#get_data_retention_policy_by_id) | **GET** /data_retention/policies/{policy_id} | Get a granular data retention policy
[**get_team_policies_for_user**](DataRetentionApi.md#get_team_policies_for_user) | **GET** /users/{user_id}/data_retention/team_policies | Get the policies which are applied to a user's teams
[**get_teams_for_retention_policy**](DataRetentionApi.md#get_teams_for_retention_policy) | **GET** /data_retention/policies/{policy_id}/teams | Get the teams for a granular data retention policy
[**patch_data_retention_policy**](DataRetentionApi.md#patch_data_retention_policy) | **PATCH** /data_retention/policies/{policy_id} | Patch a granular data retention policy
[**remove_channels_from_retention_policy**](DataRetentionApi.md#remove_channels_from_retention_policy) | **DELETE** /data_retention/policies/{policy_id}/channels | Delete channels from a granular data retention policy
[**remove_teams_from_retention_policy**](DataRetentionApi.md#remove_teams_from_retention_policy) | **DELETE** /data_retention/policies/{policy_id}/teams | Delete teams from a granular data retention policy
[**search_channels_for_retention_policy**](DataRetentionApi.md#search_channels_for_retention_policy) | **POST** /data_retention/policies/{policy_id}/channels/search | Search for the channels in a granular data retention policy
[**search_teams_for_retention_policy**](DataRetentionApi.md#search_teams_for_retention_policy) | **POST** /data_retention/policies/{policy_id}/teams/search | Search for the teams in a granular data retention policy



## add_channels_to_retention_policy

> crate::models::StatusOk add_channels_to_retention_policy(policy_id, request_body)
Add channels to a granular data retention policy

Adds channels to a granular data retention policy.   __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_write_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_teams_to_retention_policy

> crate::models::StatusOk add_teams_to_retention_policy(policy_id, request_body)
Add teams to a granular data retention policy

Adds teams to a granular data retention policy.   __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_write_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_data_retention_policy

> crate::models::DataRetentionPolicyWithTeamAndChannelCounts create_data_retention_policy(data_retention_policy_create)
Create a new granular data retention policy

Creates a new granular data retention policy with the specified display name and post duration.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_write_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_retention_policy_create** | [**DataRetentionPolicyCreate**](DataRetentionPolicyCreate.md) |  | [required] |

### Return type

[**crate::models::DataRetentionPolicyWithTeamAndChannelCounts**](DataRetentionPolicyWithTeamAndChannelCounts.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_retention_policy

> crate::models::StatusOk delete_data_retention_policy(policy_id)
Delete a granular data retention policy

Deletes a granular data retention policy.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_write_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_policies_for_user

> crate::models::RetentionPolicyForChannelList get_channel_policies_for_user(user_id, page, per_page)
Get the policies which are applied to a user's channels

Gets the policies which are applied to the all of the channels to which a user belongs.  __Minimum server version__: 5.35  ##### Permissions Must be logged in as the user or have the `manage_system` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of policies per page. There is a maximum limit of 200 per page. |  |[default to 60]

### Return type

[**crate::models::RetentionPolicyForChannelList**](RetentionPolicyForChannelList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channels_for_retention_policy

> Vec<crate::models::ChannelWithTeamData> get_channels_for_retention_policy(policy_id, page, per_page)
Get the channels for a granular data retention policy

Gets the channels to which a granular data retention policy is applied.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_read_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of channels per page. There is a maximum limit of 200 per page. |  |[default to 60]

### Return type

[**Vec<crate::models::ChannelWithTeamData>**](ChannelWithTeamData.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_retention_policies

> Vec<crate::models::DataRetentionPolicyWithTeamAndChannelCounts> get_data_retention_policies(page, per_page)
Get the granular data retention policies

Gets details about the granular (i.e. team or channel-specific) data retention policies from the server.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_read_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of policies per page. There is a maximum limit of 200 per page. |  |[default to 60]

### Return type

[**Vec<crate::models::DataRetentionPolicyWithTeamAndChannelCounts>**](DataRetentionPolicyWithTeamAndChannelCounts.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_retention_policies_count

> crate::models::InlineResponse20016 get_data_retention_policies_count()
Get the number of granular data retention policies

Gets the number of granular (i.e. team or channel-specific) data retention policies from the server.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_read_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_retention_policy

> crate::models::GlobalDataRetentionPolicy get_data_retention_policy()
Get the global data retention policy

Gets the current global data retention policy details from the server, including what data should be purged and the cutoff times for each data type that should be purged.  __Minimum server version__: 4.3  ##### Permissions Requires an active session but no other permissions.  ##### License Requires an E20 license. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GlobalDataRetentionPolicy**](GlobalDataRetentionPolicy.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_retention_policy_by_id

> crate::models::DataRetentionPolicyWithTeamAndChannelCounts get_data_retention_policy_by_id(policy_id)
Get a granular data retention policy

Gets details about a granular data retention policies by ID.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_read_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |

### Return type

[**crate::models::DataRetentionPolicyWithTeamAndChannelCounts**](DataRetentionPolicyWithTeamAndChannelCounts.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_policies_for_user

> crate::models::RetentionPolicyForTeamList get_team_policies_for_user(user_id, page, per_page)
Get the policies which are applied to a user's teams

Gets the policies which are applied to the all of the teams to which a user belongs.  __Minimum server version__: 5.35  ##### Permissions Must be logged in as the user or have the `manage_system` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of policies per page. There is a maximum limit of 200 per page. |  |[default to 60]

### Return type

[**crate::models::RetentionPolicyForTeamList**](RetentionPolicyForTeamList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_for_retention_policy

> Vec<crate::models::Team> get_teams_for_retention_policy(policy_id, page, per_page)
Get the teams for a granular data retention policy

Gets the teams to which a granular data retention policy is applied.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_read_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of teams per page. There is a maximum limit of 200 per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_data_retention_policy

> crate::models::DataRetentionPolicyWithTeamAndChannelCounts patch_data_retention_policy(policy_id, data_retention_policy_with_team_and_channel_ids)
Patch a granular data retention policy

Patches (i.e. replaces the fields of) a granular data retention policy. If any fields are omitted, they will not be changed.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_write_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |
**data_retention_policy_with_team_and_channel_ids** | [**DataRetentionPolicyWithTeamAndChannelIds**](DataRetentionPolicyWithTeamAndChannelIds.md) |  | [required] |

### Return type

[**crate::models::DataRetentionPolicyWithTeamAndChannelCounts**](DataRetentionPolicyWithTeamAndChannelCounts.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_channels_from_retention_policy

> crate::models::StatusOk remove_channels_from_retention_policy(policy_id, request_body)
Delete channels from a granular data retention policy

Delete channels from a granular data retention policy.   __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_write_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_teams_from_retention_policy

> crate::models::StatusOk remove_teams_from_retention_policy(policy_id, request_body)
Delete teams from a granular data retention policy

Delete teams from a granular data retention policy.   __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_write_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_channels_for_retention_policy

> Vec<crate::models::ChannelWithTeamData> search_channels_for_retention_policy(policy_id, inline_object100)
Search for the channels in a granular data retention policy

Searches for the channels to which a granular data retention policy is applied.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_read_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |
**inline_object100** | [**InlineObject100**](InlineObject100.md) |  | [required] |

### Return type

[**Vec<crate::models::ChannelWithTeamData>**](ChannelWithTeamData.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_teams_for_retention_policy

> Vec<crate::models::Team> search_teams_for_retention_policy(policy_id, inline_object99)
Search for the teams in a granular data retention policy

Searches for the teams to which a granular data retention policy is applied.  __Minimum server version__: 5.35  ##### Permissions Must have the `sysconsole_read_compliance_data_retention` permission.  ##### License Requires an E20 license. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | The ID of the granular retention policy. | [required] |
**inline_object99** | [**InlineObject99**](InlineObject99.md) |  | [required] |

### Return type

[**Vec<crate::models::Team>**](Team.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

