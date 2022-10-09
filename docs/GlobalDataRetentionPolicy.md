# GlobalDataRetentionPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message_deletion_enabled** | Option<**bool**> | Indicates whether data retention policy deletion of messages is enabled globally. | [optional]
**file_deletion_enabled** | Option<**bool**> | Indicates whether data retention policy deletion of file attachments is enabled globally. | [optional]
**message_retention_cutoff** | Option<**i32**> | The current server timestamp before which messages should be deleted. | [optional]
**file_retention_cutoff** | Option<**i32**> | The current server timestamp before which files should be deleted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


