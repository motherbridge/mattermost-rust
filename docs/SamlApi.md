# \SamlApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_saml_idp_certificate**](SamlApi.md#delete_saml_idp_certificate) | **DELETE** /saml/certificate/idp | Remove IDP certificate
[**delete_saml_private_certificate**](SamlApi.md#delete_saml_private_certificate) | **DELETE** /saml/certificate/private | Remove private key
[**delete_saml_public_certificate**](SamlApi.md#delete_saml_public_certificate) | **DELETE** /saml/certificate/public | Remove public certificate
[**get_saml_certificate_status**](SamlApi.md#get_saml_certificate_status) | **GET** /saml/certificate/status | Get certificate status
[**get_saml_metadata**](SamlApi.md#get_saml_metadata) | **GET** /saml/metadata | Get metadata
[**get_saml_metadata_from_idp**](SamlApi.md#get_saml_metadata_from_idp) | **POST** /saml/metadatafromidp | Get metadata from Identity Provider
[**migrate_auth_to_saml**](SamlApi.md#migrate_auth_to_saml) | **POST** /users/migrate_auth/saml | Migrate user accounts authentication type to SAML.
[**reset_saml_auth_data_to_email**](SamlApi.md#reset_saml_auth_data_to_email) | **POST** /saml/reset_auth_data | Reset AuthData to Email
[**upload_saml_idp_certificate**](SamlApi.md#upload_saml_idp_certificate) | **POST** /saml/certificate/idp | Upload IDP certificate
[**upload_saml_private_certificate**](SamlApi.md#upload_saml_private_certificate) | **POST** /saml/certificate/private | Upload private key
[**upload_saml_public_certificate**](SamlApi.md#upload_saml_public_certificate) | **POST** /saml/certificate/public | Upload public certificate



## delete_saml_idp_certificate

> crate::models::StatusOk delete_saml_idp_certificate()
Remove IDP certificate

Delete the current IDP certificate being used with your SAML configuration. This will also disable SAML on your system as this certificate is required for SAML. ##### Permissions Must have `sysconsole_write_authentication` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_saml_private_certificate

> crate::models::StatusOk delete_saml_private_certificate()
Remove private key

Delete the current private key being used with your SAML configuration. This will also disable encryption for SAML on your system as this key is required for that. ##### Permissions Must have `sysconsole_write_authentication` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_saml_public_certificate

> crate::models::StatusOk delete_saml_public_certificate()
Remove public certificate

Delete the current public certificate being used with your SAML configuration. This will also disable encryption for SAML on your system as this certificate is required for that. ##### Permissions Must have `sysconsole_write_authentication` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saml_certificate_status

> crate::models::SamlCertificateStatus get_saml_certificate_status()
Get certificate status

Get the status of the uploaded certificates and keys in use by your SAML configuration. ##### Permissions Must have `sysconsole_write_authentication` permission. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SamlCertificateStatus**](SamlCertificateStatus.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saml_metadata

> String get_saml_metadata()
Get metadata

Get SAML metadata from the server. SAML must be configured properly. ##### Permissions No permission required. 

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saml_metadata_from_idp

> String get_saml_metadata_from_idp(get_saml_metadata_from_idp_request)
Get metadata from Identity Provider

Get SAML metadata from the Identity Provider. SAML must be configured properly. ##### Permissions No permission required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_saml_metadata_from_idp_request** | Option<[**GetSamlMetadataFromIdpRequest**](GetSamlMetadataFromIdpRequest.md)> |  |  |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_auth_to_saml

> migrate_auth_to_saml(migrate_auth_to_saml_request)
Migrate user accounts authentication type to SAML.

Migrates accounts from one authentication provider to another. For example, you can upgrade your authentication provider from email to SAML. __Minimum server version__: 5.28 ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**migrate_auth_to_saml_request** | Option<[**MigrateAuthToSamlRequest**](MigrateAuthToSamlRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_saml_auth_data_to_email

> crate::models::ResetSamlAuthDataToEmail200Response reset_saml_auth_data_to_email(reset_saml_auth_data_to_email_request)
Reset AuthData to Email

Reset the AuthData field of SAML users to their email. This is meant to be used when the \"id\" attribute is set to an empty value (\"\") from a previously non-empty value. __Minimum server version__: 5.35 ##### Permissions Must have `manage_system` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reset_saml_auth_data_to_email_request** | Option<[**ResetSamlAuthDataToEmailRequest**](ResetSamlAuthDataToEmailRequest.md)> |  |  |

### Return type

[**crate::models::ResetSamlAuthDataToEmail200Response**](ResetSamlAuthDataToEmail_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_saml_idp_certificate

> crate::models::StatusOk upload_saml_idp_certificate(certificate)
Upload IDP certificate

Upload the IDP certificate to be used with your SAML configuration. The server will pick a hard-coded filename for the IdpCertificateFile setting in your `config.json`. ##### Permissions Must have `sysconsole_write_authentication` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate** | **std::path::PathBuf** | The IDP certificate file | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_saml_private_certificate

> crate::models::StatusOk upload_saml_private_certificate(certificate)
Upload private key

Upload the private key to be used for encryption with your SAML configuration. The server will pick a hard-coded filename for the PrivateKeyFile setting in your `config.json`. ##### Permissions Must have `sysconsole_write_authentication` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate** | **std::path::PathBuf** | The private key file | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_saml_public_certificate

> crate::models::StatusOk upload_saml_public_certificate(certificate)
Upload public certificate

Upload the public certificate to be used for encryption with your SAML configuration. The server will pick a hard-coded filename for the PublicCertificateFile setting in your `config.json`. ##### Permissions Must have `sysconsole_write_authentication` permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate** | **std::path::PathBuf** | The public certificate file | [required] |

### Return type

[**crate::models::StatusOk**](StatusOK.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

