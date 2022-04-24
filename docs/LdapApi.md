# \LdapApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ldap_groups**](LdapApi.md#get_ldap_groups) | **GET** /ldap/groups | Returns a list of LDAP groups
[**link_ldap_group**](LdapApi.md#link_ldap_group) | **POST** /ldap/groups/{remote_id}/link | Link a LDAP group



## get_ldap_groups

> Vec<crate::models::LdapGroupsPaged> get_ldap_groups(q, page, per_page)
Returns a list of LDAP groups

##### Permissions Must have `manage_system` permission. __Minimum server version__: 5.11 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Search term |  |
**page** | Option<**i64**> | The page to select. |  |[default to 0]
**per_page** | Option<**i64**> | The number of users per page. There is a maximum limit of 200 users per page. |  |[default to 60]

### Return type

[**Vec<crate::models::LdapGroupsPaged>**](LDAPGroupsPaged.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_ldap_group

> crate::models::StatusOk link_ldap_group(remote_id)
Link a LDAP group

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

