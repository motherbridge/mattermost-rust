# \ReactionsApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_reaction**](ReactionsApi.md#delete_reaction) | **DELETE** /users/{user_id}/posts/{post_id}/reactions/{emoji_name} | Remove a reaction from a post
[**get_bulk_reactions**](ReactionsApi.md#get_bulk_reactions) | **POST** /posts/ids/reactions | Bulk get the reaction for posts
[**get_reactions**](ReactionsApi.md#get_reactions) | **GET** /posts/{post_id}/reactions | Get a list of reactions to a post
[**get_top_reactions_for_team**](ReactionsApi.md#get_top_reactions_for_team) | **GET** /teams/{team_id}/top/reactions | Get a list of the top reactions for a team.
[**get_top_reactions_for_user**](ReactionsApi.md#get_top_reactions_for_user) | **GET** /users/me/top/reactions | Get a list of the top reactions across all public and private channels (the user is a member of) for a given user.
[**save_reaction**](ReactionsApi.md#save_reaction) | **POST** /reactions | Create a reaction



## delete_reaction

> crate::models::StatusOk delete_reaction(user_id, post_id, emoji_name)
Remove a reaction from a post

Deletes a reaction made by a user from the given post. ##### Permissions Must be user or have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**post_id** | **String** | ID of the post | [required] |
**emoji_name** | **String** | emoji name | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bulk_reactions

> ::std::collections::HashMap<String, Vec<crate::models::Reaction>> get_bulk_reactions(request_body)
Bulk get the reaction for posts

Get a list of reactions made by all users to a given post. ##### Permissions Must have `read_channel` permission for the channel the post is in.  __Minimum server version__: 5.8 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | Array of post IDs | [required] |

### Return type

[**::std::collections::HashMap<String, Vec<crate::models::Reaction>>**](array.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reactions

> Vec<crate::models::Reaction> get_reactions(post_id)
Get a list of reactions to a post

Get a list of reactions made by all users to a given post. ##### Permissions Must have `read_channel` permission for the channel the post is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_id** | **String** | ID of a post | [required] |

### Return type

[**Vec<crate::models::Reaction>**](Reaction.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_reactions_for_team

> crate::models::TopReactionList get_top_reactions_for_team(team_id, time_range, page, per_page)
Get a list of the top reactions for a team.

Get a list of the top reactions across all public and private channels (the user is a member of) for a given team. ##### Permissions Must have `view_team` permission for the team. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: reactions posted on the current day. - `7_day`: reactions posted in the last 7 days. - `28_day`: reactions posted in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of reactions per page, up to a maximum of 200. |  |[default to 60]

### Return type

[**crate::models::TopReactionList**](TopReactionList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_top_reactions_for_user

> crate::models::TopReactionList get_top_reactions_for_user(user_id, time_range, page, per_page, team_id)
Get a list of the top reactions across all public and private channels (the user is a member of) for a given user.

Get a list of the top reactions for a user. ##### Permissions Must be logged in as the user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User GUID | [required] |
**time_range** | **String** | Time range can be \"today\", \"7_day\", or \"28_day\". - `today`: reactions posted on the current day. - `7_day`: reactions posted in the last 7 days. - `28_day`: reactions posted in the last 28 days.  | [required] |
**page** | Option<**i32**> | The page to select. |  |[default to 0]
**per_page** | Option<**i32**> | The number of reactions per page, up to a maximum of 200. |  |[default to 60]
**team_id** | Option<**String**> | Team ID will scope the response to a given team. ##### Permissions Must have `view_team` permission for the team.  |  |

### Return type

[**crate::models::TopReactionList**](TopReactionList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_reaction

> crate::models::Reaction save_reaction(reaction)
Create a reaction

Create a reaction. ##### Permissions Must have `read_channel` permission for the channel the post is in. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reaction** | [**Reaction**](Reaction.md) | The user's reaction with its post_id, user_id, and emoji_name fields set | [required] |

### Return type

[**crate::models::Reaction**](Reaction.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

