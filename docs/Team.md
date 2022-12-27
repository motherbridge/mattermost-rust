# Team

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**create_at** | Option<**i64**> | The time in milliseconds a team was created | [optional]
**update_at** | Option<**i64**> | The time in milliseconds a team was last updated | [optional]
**delete_at** | Option<**i64**> | The time in milliseconds a team was deleted | [optional]
**display_name** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**allowed_domains** | Option<**String**> |  | [optional]
**invite_id** | Option<**String**> |  | [optional]
**allow_open_invite** | Option<**bool**> |  | [optional]
**policy_id** | Option<**String**> | The data retention policy to which this team has been assigned. If no such policy exists, or the caller does not have the `sysconsole_read_compliance_data_retention` permission, this field will be null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


