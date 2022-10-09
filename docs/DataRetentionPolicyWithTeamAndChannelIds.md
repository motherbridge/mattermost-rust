# DataRetentionPolicyWithTeamAndChannelIds

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_name** | Option<**String**> | The display name for this retention policy. | [optional]
**post_duration** | Option<**i32**> | The number of days a message will be retained before being deleted by this policy. If this value is less than 0, the policy has infinite retention (i.e. messages are never deleted).  | [optional]
**team_ids** | Option<**Vec<String>**> | The IDs of the teams to which this policy should be applied. | [optional]
**channel_ids** | Option<**Vec<String>**> | The IDs of the channels to which this policy should be applied. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


