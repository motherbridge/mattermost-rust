# SearchChannelsForRetentionPolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**term** | Option<**String**> | The string to search in the channel name, display name, and purpose. | [optional]
**team_ids** | Option<**Vec<String>**> | Filters results to channels belonging to the given team ids  | [optional]
**public** | Option<**bool**> | Filters results to only return Public / Open channels, can be used in conjunction with `private` to return both `public` and `private` channels  | [optional]
**private** | Option<**bool**> | Filters results to only return Private channels, can be used in conjunction with `public` to return both `private` and `public` channels  | [optional]
**deleted** | Option<**bool**> | Filters results to only return deleted / archived channels  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


