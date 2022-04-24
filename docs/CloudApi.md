# \CloudApi

All URIs are relative to *http://your-mattermost-url.com/api/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirm_customer_payment**](CloudApi.md#confirm_customer_payment) | **POST** /cloud/payment/confirm | Completes the payment setup intent
[**create_customer_payment**](CloudApi.md#create_customer_payment) | **POST** /cloud/payment | Create a customer setup payment intent
[**get_cloud_customer**](CloudApi.md#get_cloud_customer) | **GET** /cloud/customer | Get cloud customer
[**get_cloud_products**](CloudApi.md#get_cloud_products) | **GET** /cloud/products | Get cloud products
[**get_invoice_for_subscription_as_pdf**](CloudApi.md#get_invoice_for_subscription_as_pdf) | **GET** /cloud/subscription/invoices/{invoice_id}/pdf | Get cloud invoice PDF
[**get_invoices_for_subscription**](CloudApi.md#get_invoices_for_subscription) | **GET** /cloud/subscription/invoices | Get cloud subscription invoices
[**get_subscription**](CloudApi.md#get_subscription) | **GET** /cloud/subscription | Get cloud subscription
[**get_subscription_stats**](CloudApi.md#get_subscription_stats) | **GET** /cloud/subscription/stats | GET endpoint for cloud subscription stats
[**post_endpoint_for_cws_webhooks**](CloudApi.md#post_endpoint_for_cws_webhooks) | **POST** /cloud/webhook | POST endpoint for CWS Webhooks
[**send_admin_upgrade_request_email**](CloudApi.md#send_admin_upgrade_request_email) | **POST** /cloud/subscription/limitreached/invite | POST endpoint for triggering sending emails to admin with request to upgrade workspace
[**send_admin_upgrade_request_email_on_join**](CloudApi.md#send_admin_upgrade_request_email_on_join) | **POST** /cloud/subscription/limitreached/join | POST endpoint for triggering sending emails to admin with request to upgrade workspace
[**update_cloud_customer**](CloudApi.md#update_cloud_customer) | **PUT** /cloud/customer | Update cloud customer
[**update_cloud_customer_address**](CloudApi.md#update_cloud_customer_address) | **PUT** /cloud/customer/address | Update cloud customer address



## confirm_customer_payment

> confirm_customer_payment(stripe_setup_intent_id)
Completes the payment setup intent

Confirms the payment setup intent initiated when posting to `/cloud/payment`. ##### Permissions Must have `manage_system` permission and be licensed for Cloud. __Minimum server version__: 5.28 __Note:__ This is intended for internal use and is subject to change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stripe_setup_intent_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_customer_payment

> crate::models::PaymentSetupIntent create_customer_payment()
Create a customer setup payment intent

Creates a customer setup payment intent for the given Mattermost cloud installation.  ##### Permissions  Must have `manage_system` permission and be licensed for Cloud.  __Minimum server version__: 5.28 __Note:__: This is intended for internal use and is subject to change. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PaymentSetupIntent**](PaymentSetupIntent.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_customer

> crate::models::CloudCustomer get_cloud_customer()
Get cloud customer

Retrieves the customer information for the Mattermost Cloud customer bound to this installation. ##### Permissions Must have `manage_system` permission and be licensed for Cloud. __Minimum server version__: 5.28 __Note:__ This is intended for internal use and is subject to change. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CloudCustomer**](CloudCustomer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_products

> Vec<crate::models::Product> get_cloud_products()
Get cloud products

Retrieve a list of all products that are offered for Mattermost Cloud. ##### Permissions Must have `manage_system` permission and be licensed for Cloud. __Minimum server version__: 5.28 __Note:__ This is intended for internal use and is subject to change. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Product>**](Product.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice_for_subscription_as_pdf

> get_invoice_for_subscription_as_pdf(invoice_id)
Get cloud invoice PDF

Retrieves the PDF for the invoice passed as parameter ##### Permissions Must have `manage_system` permission and be licensed for Cloud. __Minimum server version__: 5.30 __Note:__ This is intended for internal use and is subject to change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | Invoice ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoices_for_subscription

> Vec<crate::models::Invoice> get_invoices_for_subscription()
Get cloud subscription invoices

Retrieves the invoices for the subscription bound to this installation. ##### Permissions Must have `manage_system` permission and be licensed for Cloud. __Minimum server version__: 5.30 __Note:__ This is intended for internal use and is subject to change. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Invoice>**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription

> crate::models::Subscription get_subscription()
Get cloud subscription

Retrieves the subscription information for the Mattermost Cloud customer bound to this installation. ##### Permissions Must have `manage_system` permission and be licensed for Cloud. __Minimum server version__: 5.28 __Note:__ This is intended for internal use and is subject to change. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Subscription**](Subscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_stats

> crate::models::SubscriptionStats get_subscription_stats()
GET endpoint for cloud subscription stats

An endpoint that returns stats about a user's subscription. For example remaining seats on a free tier ##### Permissions This endpoint should only be accessed in a Mattermost Cloud instance __Minimum server version__: 5.34 __Note:__ This is intended for internal use and is subject to change. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SubscriptionStats**](SubscriptionStats.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_endpoint_for_cws_webhooks

> post_endpoint_for_cws_webhooks()
POST endpoint for CWS Webhooks

An endpoint for processing webhooks from the Customer Portal ##### Permissions This endpoint should only be accessed by CWS, in a Mattermost Cloud instance __Minimum server version__: 5.30 __Note:__ This is intended for internal use and is subject to change. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_admin_upgrade_request_email

> send_admin_upgrade_request_email()
POST endpoint for triggering sending emails to admin with request to upgrade workspace

An endpoint that triggers sending emails to all sys admins to request them to upgrade the workspace when a user tries to invite more users ##### Permissions This endpoint should only be accessed in a Mattermost Cloud instance __Minimum server version__: 5.34 __Note:__ This is intended for internal use and is subject to change. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_admin_upgrade_request_email_on_join

> send_admin_upgrade_request_email_on_join()
POST endpoint for triggering sending emails to admin with request to upgrade workspace

An endpoint that triggers sending emails to all sys admins to request them to upgrade the workspace when a user tries to join the workspace ##### Permissions This endpoint should only be accessed in a Mattermost Cloud instance __Minimum server version__: 5.34 __Note:__ This is intended for internal use and is subject to change. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cloud_customer

> crate::models::CloudCustomer update_cloud_customer(inline_object113)
Update cloud customer

Updates the customer information for the Mattermost Cloud customer bound to this installation. ##### Permissions Must have `manage_system` permission and be licensed for Cloud. __Minimum server version__: 5.29 __Note:__ This is intended for internal use and is subject to change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object113** | [**InlineObject113**](InlineObject113.md) |  | [required] |

### Return type

[**crate::models::CloudCustomer**](CloudCustomer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cloud_customer_address

> crate::models::CloudCustomer update_cloud_customer_address(address)
Update cloud customer address

Updates the company address for the Mattermost Cloud customer bound to this installation. ##### Permissions Must have `manage_system` permission and be licensed for Cloud. __Minimum server version__: 5.29 __Note:__ This is intended for internal use and is subject to change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | [**Address**](Address.md) | Company address information to update | [required] |

### Return type

[**crate::models::CloudCustomer**](CloudCustomer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

