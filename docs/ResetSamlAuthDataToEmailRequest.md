# ResetSamlAuthDataToEmailRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | Whether to include deleted users. | [optional][default to false]
**dry_run** | Option<**bool**> | If set to true, the number of users who would be affected is returned. | [optional][default to false]
**user_ids** | Option<**Vec<String>**> | If set to a non-empty array, then users whose IDs are not in the array will be excluded. | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


