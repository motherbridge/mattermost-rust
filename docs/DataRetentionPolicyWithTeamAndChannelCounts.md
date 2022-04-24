# DataRetentionPolicyWithTeamAndChannelCounts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_name** | Option<**String**> | The display name for this retention policy. | [optional]
**post_duration** | Option<**i32**> | The number of days a message will be retained before being deleted by this policy. If this value is less than 0, the policy has infinite retention (i.e. messages are never deleted).  | [optional]
**id** | Option<**String**> | The ID of this retention policy. | [optional]
**team_count** | Option<**i32**> | The number of teams to which this policy is applied. | [optional]
**channel_count** | Option<**i32**> | The number of channels to which this policy is applied. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


