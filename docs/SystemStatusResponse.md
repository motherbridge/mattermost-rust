# SystemStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**android_latest_version** | Option<**String**> | Latest Android version supported | [optional]
**android_min_version** | Option<**String**> | Minimum Android version supported | [optional]
**desktop_latest_version** | Option<**String**> | Latest desktop version supported | [optional]
**desktop_min_version** | Option<**String**> | Minimum desktop version supported | [optional]
**ios_latest_version** | Option<**String**> | Latest iOS version supported | [optional]
**ios_min_version** | Option<**String**> | Minimum iOS version supported | [optional]
**database_status** | Option<**String**> | Status of database (\"OK\" or \"UNHEALTHY\"). Included when get_server_status parameter set. | [optional]
**filestore_status** | Option<**String**> | Status of filestore (\"OK\" or \"UNHEALTHY\"). Included when get_server_status parameter set. | [optional]
**status** | Option<**String**> | Status of server (\"OK\" or \"UNHEALTHY\"). Included when get_server_status parameter set. | [optional]
**can_receive_notifications** | Option<**String**> | Whether the device id provided can receive notifications (\"true\", \"false\" or \"unknown\"). Included when device_id parameter set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


