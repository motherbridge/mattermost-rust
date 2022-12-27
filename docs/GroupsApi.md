# \GroupsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_members**](GroupsApi.md#add_group_members) | **POST** /groups/{group_id}/members | Adds members to a custom group
[**create_group**](GroupsApi.md#create_group) | **POST** /groups | Create a custom group
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /groups/{group_id} | Deletes a custom group
[**delete_group_members**](GroupsApi.md#delete_group_members) | **DELETE** /groups/{group_id}/members | Removes members from a custom group
[**get_group**](GroupsApi.md#get_group) | **GET** /groups/{group_id} | Get a group
[**get_group_stats**](GroupsApi.md#get_group_stats) | **GET** /groups/{group_id}/stats | Get group stats
[**get_group_syncable_for_channel_id**](GroupsApi.md#get_group_syncable_for_channel_id) | **GET** /groups/{group_id}/channels/{channel_id} | Get GroupSyncable from channel ID
[**get_group_syncable_for_team_id**](GroupsApi.md#get_group_syncable_for_team_id) | **GET** /groups/{group_id}/teams/{team_id} | Get GroupSyncable from Team ID
[**get_group_syncables_channels**](GroupsApi.md#get_group_syncables_channels) | **GET** /groups/{group_id}/channels | Get group channels
[**get_group_syncables_teams**](GroupsApi.md#get_group_syncables_teams) | **GET** /groups/{group_id}/teams | Get group teams
[**get_group_users**](GroupsApi.md#get_group_users) | **GET** /groups/{group_id}/members | Get group users
[**get_groups**](GroupsApi.md#get_groups) | **GET** /groups | Get groups
[**get_groups_associated_to_channels_by_team**](GroupsApi.md#get_groups_associated_to_channels_by_team) | **GET** /teams/{team_id}/groups_by_channels | Get team groups by channels
[**get_groups_by_channel**](GroupsApi.md#get_groups_by_channel) | **GET** /channels/{channel_id}/groups | Get channel groups
[**get_groups_by_team**](GroupsApi.md#get_groups_by_team) | **GET** /teams/{team_id}/groups | Get team groups
[**get_groups_by_user_id**](GroupsApi.md#get_groups_by_user_id) | **GET** /users/{user_id}/groups | Get groups for a userId
[**link_group_syncable_for_channel**](GroupsApi.md#link_group_syncable_for_channel) | **POST** /groups/{group_id}/channels/{channel_id}/link | Link a channel to a group
[**link_group_syncable_for_team**](GroupsApi.md#link_group_syncable_for_team) | **POST** /groups/{group_id}/teams/{team_id}/link | Link a team to a group
[**patch_group**](GroupsApi.md#patch_group) | **PUT** /groups/{group_id}/patch | Patch a group
[**patch_group_syncable_for_channel**](GroupsApi.md#patch_group_syncable_for_channel) | **PUT** /groups/{group_id}/channels/{channel_id}/patch | Patch a GroupSyncable associated to Channel
[**patch_group_syncable_for_team**](GroupsApi.md#patch_group_syncable_for_team) | **PUT** /groups/{group_id}/teams/{team_id}/patch | Patch a GroupSyncable associated to Team
[**unlink_group_syncable_for_channel**](GroupsApi.md#unlink_group_syncable_for_channel) | **DELETE** /groups/{group_id}/channels/{channel_id}/link | Delete a link from a channel to a group
[**unlink_group_syncable_for_team**](GroupsApi.md#unlink_group_syncable_for_team) | **DELETE** /groups/{group_id}/teams/{team_id}/link | Delete a link from a team to a group
[**unlink_ldap_group**](GroupsApi.md#unlink_ldap_group) | **DELETE** /ldap/groups/{remote_id}/link | Delete a link for LDAP group



## add_group_members

> crate::models::StatusOk add_group_members(group_id, add_group_members_request)
Adds members to a custom group

Adds members to a custom group.  ##### Permissions Must have `custom_group_manage_members` permission for the given group.  __Minimum server version__: 6.3 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group. | [required] |
**add_group_members_request** | [**AddGroupMembersRequest**](AddGroupMembersRequest.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> create_group(create_group_request)
Create a custom group

Create a `custom` type group.  #### Permission Must have `create_custom_group` permission.  __Minimum server version__: 6.3 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_group_request** | [**CreateGroupRequest**](CreateGroupRequest.md) | Group object and initial members. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> crate::models::StatusOk delete_group(group_id)
Deletes a custom group

Soft deletes a custom group.  ##### Permissions Must have `custom_group_delete` permission for the given group.  __Minimum server version__: 6.3 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group. | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_members

> crate::models::StatusOk delete_group_members(group_id, delete_group_members_request)
Removes members from a custom group

Soft deletes a custom group members.  ##### Permissions Must have `custom_group_manage_members` permission for the given group.  __Minimum server version__: 6.3 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of the group to delete. | [required] |
**delete_group_members_request** | [**DeleteGroupMembersRequest**](DeleteGroupMembersRequest.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> crate::models::Group get_group(group_id)
Get a group

Get group from the provided group id string  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_stats

> crate::models::GetGroupStats200Response get_group_stats(group_id)
Get group stats

Retrieve the stats of a given group.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.26 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |

### Return type

[**crate::models::GetGroupStats200Response**](GetGroupStats_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_syncable_for_channel_id

> crate::models::GroupSyncableChannel get_group_syncable_for_channel_id(group_id, channel_id)
Get GroupSyncable from channel ID

Get the GroupSyncable object with group_id and channel_id from params ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::GroupSyncableChannel**](GroupSyncableChannel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_syncable_for_team_id

> crate::models::GroupSyncableTeam get_group_syncable_for_team_id(group_id, team_id)
Get GroupSyncable from Team ID

Get the GroupSyncable object with group_id and team_id from params ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::GroupSyncableTeam**](GroupSyncableTeam.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_syncables_channels

> Vec<crate::models::GroupSyncableChannels> get_group_syncables_channels(group_id)
Get group channels

Retrieve the list of channels associated to the group ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |

### Return type

[**Vec<crate::models::GroupSyncableChannels>**](GroupSyncableChannels.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_syncables_teams

> Vec<crate::models::GroupSyncableTeams> get_group_syncables_teams(group_id)
Get group teams

Retrieve the list of teams associated to the group ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |

### Return type

[**Vec<crate::models::GroupSyncableTeams>**](GroupSyncableTeams.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_users

> crate::models::GetGroupUsers200Response get_group_users(group_id, page, per_page)
Get group users

Retrieve the list of users associated with a given group.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]

### Return type

[**crate::models::GetGroupUsers200Response**](GetGroupUsers_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups

> Vec<crate::models::Group> get_groups(not_associated_to_team, not_associated_to_channel, page, per_page, q, include_member_count, since, filter_allow_reference)
Get groups

Retrieve a list of all groups not associated to a particular channel or team.  `not_associated_to_team` **OR** `not_associated_to_channel` is required.  If you use `not_associated_to_team`, you must be a team admin for that particular team (permission to manage that team).  If you use `not_associated_to_channel`, you must be a channel admin for that particular channel (permission to manage that channel).  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**not_associated_to_team** | **String** | Team GUID which is used to return all the groups not associated to this team | [required] |
**not_associated_to_channel** | **String** | Group GUID which is used to return all the groups not associated to this channel | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]
**q** | Option<**String**> | String to pattern match the `name` and `display_name` field. Will return all groups whose `name` and `display_name` field match any of the text. |  |
**include_member_count** | Option<**bool**> | Boolean which adds the `member_count` attribute to each group JSON object |  |
**since** | Option<**i32**> | Only return groups that have been modified since the given Unix timestamp (in milliseconds). All modified groups, including deleted and created groups, will be returned. __Minimum server version__: 5.24  |  |
**filter_allow_reference** | Option<**bool**> | Boolean which filters the group entries with the `allow_reference` attribute set. |  |[default to false]

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_associated_to_channels_by_team

> Vec<crate::models::Map> get_groups_associated_to_channels_by_team(team_id, page, per_page, filter_allow_reference, paginate)
Get team groups by channels

Retrieve the set of groups associated with the channels in the given team grouped by channel.  ##### Permissions Must have `manage_system` permission or can access only for current user  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]
**filter_allow_reference** | Option<**bool**> | Boolean which filters in the group entries with the `allow_reference` attribute set. |  |[default to false]
**paginate** | Option<**bool**> | Boolean to determine whether the pagination should be applied or not |  |[default to false]

### Return type

[**Vec<crate::models::Map>**](map.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_by_channel

> Vec<crate::models::Group> get_groups_by_channel(channel_id, page, per_page, filter_allow_reference)
Get channel groups

Retrieve the list of groups associated with a given channel.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]
**filter_allow_reference** | Option<**bool**> | Boolean which filters the group entries with the `allow_reference` attribute set. |  |[default to false]

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_by_team

> Vec<crate::models::Group> get_groups_by_team(team_id, page, per_page, filter_allow_reference)
Get team groups

Retrieve the list of groups associated with a given team.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of groups per page. |  |[default to 60]
**filter_allow_reference** | Option<**bool**> | Boolean which filters in the group entries with the `allow_reference` attribute set. |  |[default to false]

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_by_user_id

> Vec<crate::models::Group> get_groups_by_user_id(user_id)
Get groups for a userId

Retrieve the list of groups associated to the user  __Minimum server version__: 5.24 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_group_syncable_for_channel

> crate::models::GroupSyncableChannel link_group_syncable_for_channel(group_id, channel_id)
Link a channel to a group

Link a channel to a group ##### Permissions If the channel is private, you must have `manage_private_channel_members` permission. Otherwise, you must have the `manage_public_channel_members` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::GroupSyncableChannel**](GroupSyncableChannel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_group_syncable_for_team

> crate::models::GroupSyncableTeam link_group_syncable_for_team(group_id, team_id)
Link a team to a group

Link a team to a group ##### Permissions Must have `manage_team` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::GroupSyncableTeam**](GroupSyncableTeam.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_group

> crate::models::Group patch_group(group_id, patch_group_request)
Patch a group

Partially update a group by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**patch_group_request** | [**PatchGroupRequest**](PatchGroupRequest.md) | Group object that is to be updated | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_group_syncable_for_channel

> crate::models::GroupSyncableChannel patch_group_syncable_for_channel(group_id, channel_id, patch_group_syncable_for_team_request)
Patch a GroupSyncable associated to Channel

Partially update a GroupSyncable by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |
**patch_group_syncable_for_team_request** | [**PatchGroupSyncableForTeamRequest**](PatchGroupSyncableForTeamRequest.md) | GroupSyncable object that is to be updated | [required] |

### Return type

[**crate::models::GroupSyncableChannel**](GroupSyncableChannel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_group_syncable_for_team

> crate::models::GroupSyncableTeam patch_group_syncable_for_team(group_id, team_id, patch_group_syncable_for_team_request)
Patch a GroupSyncable associated to Team

Partially update a GroupSyncable by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**team_id** | **String** | Team GUID | [required] |
**patch_group_syncable_for_team_request** | [**PatchGroupSyncableForTeamRequest**](PatchGroupSyncableForTeamRequest.md) | GroupSyncable object that is to be updated | [required] |

### Return type

[**crate::models::GroupSyncableTeam**](GroupSyncableTeam.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_group_syncable_for_channel

> crate::models::StatusOk unlink_group_syncable_for_channel(group_id, channel_id)
Delete a link from a channel to a group

Delete a link from a channel to a group ##### Permissions If the channel is private, you must have `manage_private_channel_members` permission. Otherwise, you must have the `manage_public_channel_members` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_group_syncable_for_team

> crate::models::StatusOk unlink_group_syncable_for_team(group_id, team_id)
Delete a link from a team to a group

Delete a link from a team to a group ##### Permissions Must have `manage_team` permission.  __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_ldap_group

> crate::models::StatusOk unlink_ldap_group(remote_id)
Delete a link for LDAP group

##### Permissions Must have `manage_system` permission. __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote_id** | **String** | Group GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

