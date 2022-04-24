# \ChannelsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_channel_member**](ChannelsApi.md#add_channel_member) | **POST** /channels/{channel_id}/members | Add user to channel
[**autocomplete_channels_for_team**](ChannelsApi.md#autocomplete_channels_for_team) | **GET** /teams/{team_id}/channels/autocomplete | Autocomplete channels
[**autocomplete_channels_for_team_for_search**](ChannelsApi.md#autocomplete_channels_for_team_for_search) | **GET** /teams/{team_id}/channels/search_autocomplete | Autocomplete channels for search
[**channel_members_minus_group_members**](ChannelsApi.md#channel_members_minus_group_members) | **GET** /channels/{channel_id}/members_minus_group_members | Channel members minus group members.
[**create_channel**](ChannelsApi.md#create_channel) | **POST** /channels | Create a channel
[**create_direct_channel**](ChannelsApi.md#create_direct_channel) | **POST** /channels/direct | Create a direct message channel
[**create_group_channel**](ChannelsApi.md#create_group_channel) | **POST** /channels/group | Create a group message channel
[**create_sidebar_category_for_team_for_user**](ChannelsApi.md#create_sidebar_category_for_team_for_user) | **POST** /users/{user_id}/teams/{team_id}/channels/categories | Create user's sidebar category
[**delete_channel**](ChannelsApi.md#delete_channel) | **DELETE** /channels/{channel_id} | Delete a channel
[**get_all_channels**](ChannelsApi.md#get_all_channels) | **GET** /channels | Get a list of all channels
[**get_channel**](ChannelsApi.md#get_channel) | **GET** /channels/{channel_id} | Get a channel
[**get_channel_by_name**](ChannelsApi.md#get_channel_by_name) | **GET** /teams/{team_id}/channels/name/{channel_name} | Get a channel by name
[**get_channel_by_name_for_team_name**](ChannelsApi.md#get_channel_by_name_for_team_name) | **GET** /teams/name/{team_name}/channels/name/{channel_name} | Get a channel by name and team name
[**get_channel_member**](ChannelsApi.md#get_channel_member) | **GET** /channels/{channel_id}/members/{user_id} | Get channel member
[**get_channel_member_counts_by_group**](ChannelsApi.md#get_channel_member_counts_by_group) | **GET** /channels/{channel_id}/member_counts_by_group | Channel members counts for each group that has atleast one member in the channel
[**get_channel_members**](ChannelsApi.md#get_channel_members) | **GET** /channels/{channel_id}/members | Get channel members
[**get_channel_members_by_ids**](ChannelsApi.md#get_channel_members_by_ids) | **POST** /channels/{channel_id}/members/ids | Get channel members by ids
[**get_channel_members_for_user**](ChannelsApi.md#get_channel_members_for_user) | **GET** /users/{user_id}/teams/{team_id}/channels/members | Get channel memberships and roles for a user
[**get_channel_members_timezones**](ChannelsApi.md#get_channel_members_timezones) | **GET** /channels/{channel_id}/timezones | Get timezones in a channel
[**get_channel_moderations**](ChannelsApi.md#get_channel_moderations) | **GET** /channels/{channel_id}/moderations | Get information about channel's moderation.
[**get_channel_stats**](ChannelsApi.md#get_channel_stats) | **GET** /channels/{channel_id}/stats | Get channel statistics
[**get_channel_unread**](ChannelsApi.md#get_channel_unread) | **GET** /users/{user_id}/channels/{channel_id}/unread | Get unread messages
[**get_channels_for_team_for_user**](ChannelsApi.md#get_channels_for_team_for_user) | **GET** /users/{user_id}/teams/{team_id}/channels | Get channels for user
[**get_channels_for_user**](ChannelsApi.md#get_channels_for_user) | **GET** /users/{user_id}/channels | Get all channels from all teams
[**get_deleted_channels_for_team**](ChannelsApi.md#get_deleted_channels_for_team) | **GET** /teams/{team_id}/channels/deleted | Get deleted channels
[**get_pinned_posts**](ChannelsApi.md#get_pinned_posts) | **GET** /channels/{channel_id}/pinned | Get a channel's pinned posts
[**get_private_channels_for_team**](ChannelsApi.md#get_private_channels_for_team) | **GET** /teams/{team_id}/channels/private | Get private channels
[**get_public_channels_by_ids_for_team**](ChannelsApi.md#get_public_channels_by_ids_for_team) | **POST** /teams/{team_id}/channels/ids | Get a list of channels by ids
[**get_public_channels_for_team**](ChannelsApi.md#get_public_channels_for_team) | **GET** /teams/{team_id}/channels | Get public channels
[**get_sidebar_categories_for_team_for_user**](ChannelsApi.md#get_sidebar_categories_for_team_for_user) | **GET** /users/{user_id}/teams/{team_id}/channels/categories | Get user's sidebar categories
[**get_sidebar_category_for_team_for_user**](ChannelsApi.md#get_sidebar_category_for_team_for_user) | **GET** /users/{user_id}/teams/{team_id}/channels/categories/{category_id} | Get sidebar category
[**get_sidebar_category_order_for_team_for_user**](ChannelsApi.md#get_sidebar_category_order_for_team_for_user) | **GET** /users/{user_id}/teams/{team_id}/channels/categories/order | Get user's sidebar category order
[**move_channel**](ChannelsApi.md#move_channel) | **POST** /channels/{channel_id}/move | Move a channel
[**patch_channel**](ChannelsApi.md#patch_channel) | **PUT** /channels/{channel_id}/patch | Patch a channel
[**patch_channel_moderations**](ChannelsApi.md#patch_channel_moderations) | **PUT** /channels/{channel_id}/moderations/patch | Update a channel's moderation settings.
[**remove_sidebar_category_for_team_for_user**](ChannelsApi.md#remove_sidebar_category_for_team_for_user) | **DELETE** /users/{user_id}/teams/{team_id}/channels/categories/{category_id} | Delete sidebar category
[**remove_user_from_channel**](ChannelsApi.md#remove_user_from_channel) | **DELETE** /channels/{channel_id}/members/{user_id} | Remove user from channel
[**restore_channel**](ChannelsApi.md#restore_channel) | **POST** /channels/{channel_id}/restore | Restore a channel
[**search_all_channels**](ChannelsApi.md#search_all_channels) | **POST** /channels/search | Search all private and open type channels across all teams
[**search_archived_channels**](ChannelsApi.md#search_archived_channels) | **POST** /teams/{team_id}/channels/search_archived | Search archived channels
[**search_channels**](ChannelsApi.md#search_channels) | **POST** /teams/{team_id}/channels/search | Search channels
[**search_group_channels**](ChannelsApi.md#search_group_channels) | **POST** /channels/group/search | Search Group Channels
[**update_channel**](ChannelsApi.md#update_channel) | **PUT** /channels/{channel_id} | Update a channel
[**update_channel_member_scheme_roles**](ChannelsApi.md#update_channel_member_scheme_roles) | **PUT** /channels/{channel_id}/members/{user_id}/schemeRoles | Update the scheme-derived roles of a channel member.
[**update_channel_notify_props**](ChannelsApi.md#update_channel_notify_props) | **PUT** /channels/{channel_id}/members/{user_id}/notify_props | Update channel notifications
[**update_channel_privacy**](ChannelsApi.md#update_channel_privacy) | **PUT** /channels/{channel_id}/privacy | Update channel's privacy
[**update_channel_roles**](ChannelsApi.md#update_channel_roles) | **PUT** /channels/{channel_id}/members/{user_id}/roles | Update channel roles
[**update_channel_scheme**](ChannelsApi.md#update_channel_scheme) | **PUT** /channels/{channel_id}/scheme | Set a channel's scheme
[**update_sidebar_categories_for_team_for_user**](ChannelsApi.md#update_sidebar_categories_for_team_for_user) | **PUT** /users/{user_id}/teams/{team_id}/channels/categories | Update user's sidebar categories
[**update_sidebar_category_for_team_for_user**](ChannelsApi.md#update_sidebar_category_for_team_for_user) | **PUT** /users/{user_id}/teams/{team_id}/channels/categories/{category_id} | Update sidebar category
[**update_sidebar_category_order_for_team_for_user**](ChannelsApi.md#update_sidebar_category_order_for_team_for_user) | **PUT** /users/{user_id}/teams/{team_id}/channels/categories/order | Update user's sidebar category order
[**view_channel**](ChannelsApi.md#view_channel) | **POST** /channels/members/{user_id}/view | View channel



## add_channel_member

> crate::models::ChannelMember add_channel_member(channel_id, inline_object53)
Add user to channel

Add a user to a channel by creating a channel member object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The channel ID | [required] |
**inline_object53** | [**InlineObject53**](InlineObject53.md) |  | [required] |

### Return type

[**crate::models::ChannelMember**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## autocomplete_channels_for_team

> Vec<crate::models::Channel> autocomplete_channels_for_team(team_id, name)
Autocomplete channels

Autocomplete public channels on a team based on the search term provided in the request URL.  __Minimum server version__: 4.7  ##### Permissions Must have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**name** | **String** | Name or display name | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## autocomplete_channels_for_team_for_search

> Vec<crate::models::Channel> autocomplete_channels_for_team_for_search(team_id, name)
Autocomplete channels for search

Autocomplete your channels on a team based on the search term provided in the request URL.  __Minimum server version__: 5.4  ##### Permissions Must have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**name** | **String** | Name or display name | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channel_members_minus_group_members

> channel_members_minus_group_members(channel_id, group_ids, page, per_page)
Channel members minus group members.

Get the set of users who are members of the channel minus the set of users who are members of the given groups. Each user object contains an array of group objects representing the group memberships for that user. Each user object contains the boolean fields `scheme_guest`, `scheme_user`, and `scheme_admin` representing the roles that user has for the given channel.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**group_ids** | **String** | A comma-separated list of group ids. | [required] |[default to ]
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of users per page. |  |[default to 0]

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_channel

> crate::models::Channel create_channel(inline_object44)
Create a channel

Create a new channel. ##### Permissions If creating a public channel, `create_public_channel` permission is required. If creating a private channel, `create_private_channel` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object44** | [**InlineObject44**](InlineObject44.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_direct_channel

> crate::models::Channel create_direct_channel(request_body)
Create a direct message channel

Create a new direct message channel between two users. ##### Permissions Must be one of the two users and have `create_direct_channel` permission. Having the `manage_system` permission voids the previous requirements. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | The two user ids to be in the direct message | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_channel

> crate::models::Channel create_group_channel(request_body)
Create a group message channel

Create a new group message channel to group of users. If the logged in user's id is not included in the list, it will be appended to the end. ##### Permissions Must have `create_group_channel` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | User ids to be in the group message channel | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sidebar_category_for_team_for_user

> crate::models::SidebarCategory create_sidebar_category_for_team_for_user(team_id, user_id, sidebar_category)
Create user's sidebar category

Create a custom sidebar category for the user on the given team. __Minimum server version__: 5.26 ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**sidebar_category** | [**SidebarCategory**](SidebarCategory.md) |  | [required] |

### Return type

[**crate::models::SidebarCategory**](SidebarCategory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_channel

> crate::models::StatusOk delete_channel(channel_id)
Delete a channel

Archives a channel. This will set the `deleteAt` to the current timestamp in the database. Soft deleted channels may not be accessible in the user interface. They can be viewed and unarchived in the **System Console > User Management > Channels** based on your license. Direct and group message channels cannot be deleted.  As of server version 5.28, optionally use the `permanent=true` query parameter to permanently delete the channel for compliance reasons. To use this feature `ServiceSettings.EnableAPIChannelDeletion` must be set to `true` in the server's configuration.  If you permanently delete a channel this action is not recoverable outside of a database backup.  ##### Permissions `delete_public_channel` permission if the channel is public, `delete_private_channel` permission if the channel is private, or have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_channels

> Vec<crate::models::ChannelWithTeamData> get_all_channels(not_associated_to_group, page, per_page, exclude_default_channels, include_deleted, include_total_count, exclude_policy_constrained)
Get a list of all channels

##### Permissions `manage_system` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**not_associated_to_group** | Option<**String**> | A group id to exclude channels that are associated with that group via GroupChannel records. This can also be left blank with `not_associated_to_group=`. |  |
**page** | Option<**i32**> |  |  |[default to 0]
**per_page** | Option<**i32**> |  |  |[default to 0]
**exclude_default_channels** | Option<**bool**> | Whether to exclude default channels (ex Town Square, Off-Topic) from the results. |  |[default to false]
**include_deleted** | Option<**bool**> | Include channels that have been archived. This correlates to the `DeleteAt` flag being set in the database. |  |[default to false]
**include_total_count** | Option<**bool**> | Appends a total count of returned channels inside the response object - ex: `{ \"channels\": [], \"total_count\" : 0 }`.       |  |[default to false]
**exclude_policy_constrained** | Option<**bool**> | If set to true, channels which are part of a data retention policy will be excluded. The `sysconsole_read_compliance` permission is required to use this parameter. __Minimum server version__: 5.35 |  |[default to false]

### Return type

[**Vec<crate::models::ChannelWithTeamData>**](ChannelWithTeamData.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel

> crate::models::Channel get_channel(channel_id)
Get a channel

Get channel from the provided channel id string. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_by_name

> crate::models::Channel get_channel_by_name(team_id, channel_name, include_deleted)
Get a channel by name

Gets channel from the provided team id and channel name strings. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**channel_name** | **String** | Channel Name | [required] |
**include_deleted** | Option<**bool**> | Defines if deleted channels should be returned or not (Mattermost Server 5.26.0+) |  |[default to false]

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_by_name_for_team_name

> crate::models::Channel get_channel_by_name_for_team_name(team_name, channel_name, include_deleted)
Get a channel by name and team name

Gets a channel from the provided team name and channel name strings. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_name** | **String** | Team Name | [required] |
**channel_name** | **String** | Channel Name | [required] |
**include_deleted** | Option<**bool**> | Defines if deleted channels should be returned or not (Mattermost Server 5.26.0+) |  |[default to false]

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_member

> crate::models::ChannelMember get_channel_member(channel_id, user_id)
Get channel member

Get a channel member. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::ChannelMember**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_member_counts_by_group

> get_channel_member_counts_by_group(channel_id, include_timezones)
Channel members counts for each group that has atleast one member in the channel

Returns a set of ChannelMemberCountByGroup objects which contain a `group_id`, `channel_member_count` and a `channel_member_timezones_count`. ##### Permissions Must have `read_channel` permission for the given channel. __Minimum server version__: 5.24 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**include_timezones** | Option<**bool**> | Defines if member timezone counts should be returned or not |  |[default to false]

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_members

> Vec<crate::models::ChannelMember> get_channel_members(channel_id, page, per_page)
Get channel members

Get a page of members for a channel. ##### Permissions `read_channel` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of members per page. There is a maximum limit of 200 members. |  |[default to 60]

### Return type

[**Vec<crate::models::ChannelMember>**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_members_by_ids

> Vec<crate::models::ChannelMember> get_channel_members_by_ids(channel_id, request_body)
Get channel members by ids

Get a list of channel members based on the provided user ids. ##### Permissions Must have the `read_channel` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**request_body** | [**Vec<String>**](String.md) | List of user ids | [required] |

### Return type

[**Vec<crate::models::ChannelMember>**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_members_for_user

> Vec<crate::models::ChannelMember> get_channel_members_for_user(user_id, team_id)
Get channel memberships and roles for a user

Get all channel memberships and associated membership roles (i.e. `channel_user`, `channel_admin`) for a user on a specific team. ##### Permissions Logged in as the user and `view_team` permission for the team. Having `manage_system` permission voids the previous requirements. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**team_id** | **String** | Team GUID | [required] |

### Return type

[**Vec<crate::models::ChannelMember>**](ChannelMember.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_members_timezones

> Vec<String> get_channel_members_timezones(channel_id)
Get timezones in a channel

Get a list of timezones for the users who are in this channel.  __Minimum server version__: 5.6  ##### Permissions Must have the `read_channel` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_moderations

> Vec<crate::models::ChannelModeration> get_channel_moderations(channel_id)
Get information about channel's moderation.

##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.22 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**Vec<crate::models::ChannelModeration>**](ChannelModeration.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_stats

> crate::models::ChannelStats get_channel_stats(channel_id)
Get channel statistics

Get statistics for a channel. ##### Permissions Must have the `read_channel` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::ChannelStats**](ChannelStats.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_unread

> crate::models::ChannelUnread get_channel_unread(user_id, channel_id)
Get unread messages

Get the total unread messages and mentions for a channel for a user. ##### Permissions Must be logged in as user and have the `read_channel` permission, or have `edit_other_usrs` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::ChannelUnread**](ChannelUnread.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channels_for_team_for_user

> Vec<crate::models::Channel> get_channels_for_team_for_user(user_id, team_id, include_deleted, last_delete_at)
Get channels for user

Get all the channels on a team for a user. ##### Permissions Logged in as the user, or have `edit_other_users` permission, and `view_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**team_id** | **String** | Team GUID | [required] |
**include_deleted** | Option<**bool**> | Defines if deleted channels should be returned or not |  |[default to false]
**last_delete_at** | Option<**i32**> | Filters the deleted channels by this time in epoch format. Does not have any effect if include_deleted is set to false. |  |[default to 0]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channels_for_user

> Vec<crate::models::Channel> get_channels_for_user(user_id, last_delete_at, include_deleted)
Get all channels from all teams

Get all channels from all teams that a user is a member of.  __Minimum server version__: 6.1  ##### Permissions  Logged in as the user, or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. This can also be \"me\" which will point to the current user. | [required] |
**last_delete_at** | Option<**i32**> | Filters the deleted channels by this time in epoch format. Does not have any effect if include_deleted is set to false. |  |[default to 0]
**include_deleted** | Option<**bool**> | Defines if deleted channels should be returned or not |  |[default to false]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deleted_channels_for_team

> Vec<crate::models::Channel> get_deleted_channels_for_team(team_id, page, per_page)
Get deleted channels

Get a page of deleted channels on a team based on query string parameters - team_id, page and per_page.  __Minimum server version__: 3.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of public channels per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pinned_posts

> crate::models::PostList get_pinned_posts(channel_id)
Get a channel's pinned posts

Get a list of pinned posts for channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::PostList**](PostList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_private_channels_for_team

> Vec<crate::models::Channel> get_private_channels_for_team(team_id, page, per_page)
Get private channels

Get a page of private channels on a team based on query string parameters - team_id, page and per_page.  __Minimum server version__: 5.26  ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of private channels per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_channels_by_ids_for_team

> Vec<crate::models::Channel> get_public_channels_by_ids_for_team(team_id, request_body)
Get a list of channels by ids

Get a list of public channels on a team by id. ##### Permissions `view_team` for the team the channels are on. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**request_body** | [**Vec<String>**](String.md) | List of channel ids | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_channels_for_team

> Vec<crate::models::Channel> get_public_channels_for_team(team_id, page, per_page)
Get public channels

Get a page of public channels on a team based on query string parameters - page and per_page. ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of public channels per page. |  |[default to 60]

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sidebar_categories_for_team_for_user

> Vec<crate::models::OrderedSidebarCategories> get_sidebar_categories_for_team_for_user(team_id, user_id)
Get user's sidebar categories

Get a list of sidebar categories that will appear in the user's sidebar on the given team, including a list of channel IDs in each category. __Minimum server version__: 5.26 ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |

### Return type

[**Vec<crate::models::OrderedSidebarCategories>**](OrderedSidebarCategories.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sidebar_category_for_team_for_user

> crate::models::SidebarCategory get_sidebar_category_for_team_for_user(team_id, user_id, category_id)
Get sidebar category

Returns a single sidebar category for the user on the given team. __Minimum server version__: 5.26 ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**category_id** | **String** | Category GUID | [required] |

### Return type

[**crate::models::SidebarCategory**](SidebarCategory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sidebar_category_order_for_team_for_user

> Vec<String> get_sidebar_category_order_for_team_for_user(team_id, user_id)
Get user's sidebar category order

Returns the order of the sidebar categories for a user on the given team as an array of IDs. __Minimum server version__: 5.26 ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_channel

> crate::models::Channel move_channel(channel_id, inline_object50)
Move a channel

Move a channel to another team.  __Minimum server version__: 5.26  ##### Permissions  Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object50** | [**InlineObject50**](InlineObject50.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_channel

> crate::models::Channel patch_channel(channel_id, inline_object48)
Patch a channel

Partially update a channel by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored. ##### Permissions If updating a public channel, `manage_public_channel_members` permission is required. If updating a private channel, `manage_private_channel_members` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object48** | [**InlineObject48**](InlineObject48.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_channel_moderations

> Vec<crate::models::ChannelModeration> patch_channel_moderations(channel_id, channel_moderation_patch)
Update a channel's moderation settings.

##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.22 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**channel_moderation_patch** | [**ChannelModerationPatch**](ChannelModerationPatch.md) |  | [required] |

### Return type

[**Vec<crate::models::ChannelModeration>**](ChannelModeration.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_sidebar_category_for_team_for_user

> crate::models::SidebarCategory remove_sidebar_category_for_team_for_user(team_id, user_id, category_id)
Delete sidebar category

Deletes a single sidebar category for the user on the given team. Only custom categories can be deleted. __Minimum server version__: 5.26 ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**category_id** | **String** | Category GUID | [required] |

### Return type

[**crate::models::SidebarCategory**](SidebarCategory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_channel

> crate::models::StatusOk remove_user_from_channel(channel_id, user_id)
Remove user from channel

Delete a channel member, effectively removing them from a channel.  In server version 5.3 and later, channel members can only be deleted from public or private channels. ##### Permissions `manage_public_channel_members` permission if the channel is public. `manage_private_channel_members` permission if the channel is private. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_channel

> crate::models::Channel restore_channel(channel_id)
Restore a channel

Restore channel from the provided channel id string.  __Minimum server version__: 3.10  ##### Permissions `manage_team` permission for the team of the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_all_channels

> crate::models::InlineResponse2007 search_all_channels(inline_object45, system_console)
Search all private and open type channels across all teams

Returns all private and open type channels where 'term' matches on the name, display name, or purpose of the channel.  Configured 'default' channels (ex Town Square and Off-Topic) can be excluded from the results with the `exclude_default_channels` boolean parameter.  Channels that are associated (via GroupChannel records) to a given group can be excluded from the results with the `not_associated_to_group` parameter and a group id string. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object45** | [**InlineObject45**](InlineObject45.md) |  | [required] |
**system_console** | Option<**bool**> | Is the request from system_console. If this is set to true, it filters channels by the logged in user.  |  |[default to true]

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_archived_channels

> Vec<crate::models::Channel> search_archived_channels(team_id, inline_object52)
Search archived channels

Search archived channels on a team based on the search term provided in the request body.  __Minimum server version__: 5.18  ##### Permissions Must have the `list_team_channels` permission.  In server version 5.18 and later, a user without the `list_team_channels` permission will be able to use this endpoint, with the search results limited to the channels that the user is a member of. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object52** | [**InlineObject52**](InlineObject52.md) |  | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_channels

> Vec<crate::models::Channel> search_channels(team_id, inline_object51)
Search channels

Search public channels on a team based on the search term provided in the request body. ##### Permissions Must have the `list_team_channels` permission.  In server version 5.16 and later, a user without the `list_team_channels` permission will be able to use this endpoint, with the search results limited to the channels that the user is a member of. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**inline_object51** | [**InlineObject51**](InlineObject51.md) |  | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_group_channels

> Vec<crate::models::Channel> search_group_channels(inline_object46)
Search Group Channels

Get a list of group channels for a user which members' usernames match the search term.  __Minimum server version__: 5.14 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object46** | [**InlineObject46**](InlineObject46.md) |  | [required] |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel

> crate::models::Channel update_channel(channel_id, inline_object47)
Update a channel

Update a channel. The fields that can be updated are listed as parameters. Omitted fields will be treated as blanks. ##### Permissions If updating a public channel, `manage_public_channel_members` permission is required. If updating a private channel, `manage_private_channel_members` permission is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object47** | [**InlineObject47**](InlineObject47.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel_member_scheme_roles

> crate::models::StatusOk update_channel_member_scheme_roles(channel_id, user_id, inline_object55)
Update the scheme-derived roles of a channel member.

Update a channel member's scheme_admin/scheme_user properties. Typically this should either be `scheme_admin=false, scheme_user=true` for ordinary channel member, or `scheme_admin=true, scheme_user=true` for a channel admin. __Minimum server version__: 5.0 ##### Permissions Must be authenticated and have the `manage_channel_roles` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**inline_object55** | [**InlineObject55**](InlineObject55.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel_notify_props

> crate::models::StatusOk update_channel_notify_props(channel_id, user_id, channel_notify_props)
Update channel notifications

Update a user's notification properties for a channel. Only the provided fields are updated. ##### Permissions Must be logged in as the user or have `edit_other_users` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**channel_notify_props** | [**ChannelNotifyProps**](ChannelNotifyProps.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel_privacy

> crate::models::Channel update_channel_privacy(channel_id, inline_object49)
Update channel's privacy

Updates channel's privacy allowing changing a channel from Public to Private and back.  __Minimum server version__: 5.16  ##### Permissions `manage_team` permission for the channels team on version < 5.28. `convert_public_channel_to_private` permission for the channel if updating privacy to 'P' on version >= 5.28. `convert_private_channel_to_public` permission for the channel if updating privacy to 'O' on version >= 5.28. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object49** | [**InlineObject49**](InlineObject49.md) |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel_roles

> crate::models::StatusOk update_channel_roles(channel_id, user_id, inline_object54)
Update channel roles

Update a user's roles for a channel. ##### Permissions Must have `manage_channel_roles` permission for the channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**inline_object54** | [**InlineObject54**](InlineObject54.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel_scheme

> crate::models::StatusOk update_channel_scheme(channel_id, inline_object57)
Set a channel's scheme

Set a channel's scheme, more specifically sets the scheme_id value of a channel record.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 4.10 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel GUID | [required] |
**inline_object57** | [**InlineObject57**](InlineObject57.md) |  | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sidebar_categories_for_team_for_user

> crate::models::SidebarCategory update_sidebar_categories_for_team_for_user(team_id, user_id, sidebar_category)
Update user's sidebar categories

Update any number of sidebar categories for the user on the given team. This can be used to reorder the channels in these categories. __Minimum server version__: 5.26 ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**sidebar_category** | [**Vec<crate::models::SidebarCategory>**](SidebarCategory.md) |  | [required] |

### Return type

[**crate::models::SidebarCategory**](SidebarCategory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sidebar_category_for_team_for_user

> crate::models::SidebarCategory update_sidebar_category_for_team_for_user(team_id, user_id, category_id, sidebar_category)
Update sidebar category

Updates a single sidebar category for the user on the given team. __Minimum server version__: 5.26 ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**category_id** | **String** | Category GUID | [required] |
**sidebar_category** | [**SidebarCategory**](SidebarCategory.md) |  | [required] |

### Return type

[**crate::models::SidebarCategory**](SidebarCategory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sidebar_category_order_for_team_for_user

> Vec<String> update_sidebar_category_order_for_team_for_user(team_id, user_id, request_body)
Update user's sidebar category order

Updates the order of the sidebar categories for a user on the given team. The provided array must include the IDs of all categories on the team. __Minimum server version__: 5.26 ##### Permissions Must be authenticated and have the `list_team_channels` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**user_id** | **String** | User GUID | [required] |
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_channel

> crate::models::InlineResponse2008 view_channel(user_id, inline_object56)
View channel

Perform all the actions involved in viewing a channel. This includes marking channels as read, clearing push notifications, and updating the active channel. ##### Permissions Must be logged in as user or have `edit_other_users` permission.  __Response only includes `last_viewed_at_times` in Mattermost server 4.3 and newer.__ 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID to perform the view action for | [required] |
**inline_object56** | [**InlineObject56**](InlineObject56.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

