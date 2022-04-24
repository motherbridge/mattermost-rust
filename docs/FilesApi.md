# \FilesApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_file**](FilesApi.md#get_file) | **GET** /files/{file_id} | Get a file
[**get_file_info**](FilesApi.md#get_file_info) | **GET** /files/{file_id}/info | Get metadata for a file
[**get_file_link**](FilesApi.md#get_file_link) | **GET** /files/{file_id}/link | Get a public file link
[**get_file_preview**](FilesApi.md#get_file_preview) | **GET** /files/{file_id}/preview | Get a file's preview
[**get_file_public**](FilesApi.md#get_file_public) | **GET** /files/{file_id}/public | Get a public file
[**get_file_thumbnail**](FilesApi.md#get_file_thumbnail) | **GET** /files/{file_id}/thumbnail | Get a file's thumbnail
[**search_files**](FilesApi.md#search_files) | **POST** /teams/{team_id}/files/search | Search files in a team
[**upload_file**](FilesApi.md#upload_file) | **POST** /files | Upload a file



## get_file

> get_file(file_id)
Get a file

Gets a file that has been uploaded previously. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_info

> crate::models::FileInfo get_file_info(file_id)
Get metadata for a file

Gets a file's info. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file info to get | [required] |

### Return type

[**crate::models::FileInfo**](FileInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_link

> crate::models::InlineResponse2009 get_file_link(file_id)
Get a public file link

Gets a public link for a file that can be accessed without logging into Mattermost. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get a link for | [required] |

### Return type

[**crate::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_preview

> get_file_preview(file_id)
Get a file's preview

Gets a file's preview. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_public

> get_file_public(file_id, h)
Get a public file

##### Permissions No permissions required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get | [required] |
**h** | **String** | File hash | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_thumbnail

> get_file_thumbnail(file_id)
Get a file's thumbnail

Gets a file's thumbnail. ##### Permissions Must have `read_channel` permission or be uploader of the file. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **String** | The ID of the file to get | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_files

> crate::models::FileInfoList search_files(team_id, terms, is_or_search, time_zone_offset, include_deleted_channels, page, per_page)
Search files in a team

Search for files in a team based on file name, extention and file content (if file content extraction is enabled and supported for the files). __Minimum server version__: 5.34 ##### Permissions Must be authenticated and have the `view_team` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | Team GUID | [required] |
**terms** | **String** | The search terms as inputed by the user. To search for files from a user include `from:someusername`, using a user's username. To search in a specific channel include `in:somechannel`, using the channel name (not the display name). To search for specific extensions included `ext:extension`. | [required] |
**is_or_search** | **bool** | Set to true if an Or search should be performed vs an And search. | [required] |
**time_zone_offset** | Option<**i64**> | Offset from UTC of user timezone for date searches. |  |[default to 0]
**include_deleted_channels** | Option<**bool**> | Set to true if deleted channels should be included in the search. (archived channels) |  |
**page** | Option<**i64**> | The page to select. (Only works with Elasticsearch) |  |[default to 0]
**per_page** | Option<**i64**> | The number of posts per page. (Only works with Elasticsearch) |  |[default to 60]

### Return type

[**crate::models::FileInfoList**](FileInfoList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file

> crate::models::InlineResponse201 upload_file(channel_id, filename, files, channel_id2, client_ids)
Upload a file

Uploads a file that can later be attached to a post.  This request can either be a multipart/form-data request with a channel_id, files and optional client_ids defined in the FormData, or it can be a request with the channel_id and filename defined as query parameters with the contents of a single file in the body of the request.  Only multipart/form-data requests are supported by server versions up to and including 4.7. Server versions 4.8 and higher support both types of requests.  ##### Permissions Must have `upload_file` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | Option<**String**> | The ID of the channel that this file will be uploaded to |  |
**filename** | Option<**String**> | The name of the file to be uploaded |  |
**files** | Option<**std::path::PathBuf**> | A file to be uploaded |  |
**channel_id2** | Option<**String**> | The ID of the channel that this file will be uploaded to |  |
**client_ids** | Option<**String**> | A unique identifier for the file that will be returned in the response |  |

### Return type

[**crate::models::InlineResponse201**](inline_response_201.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

