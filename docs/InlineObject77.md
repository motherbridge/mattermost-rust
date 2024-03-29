# InlineObject77

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_id** | **String** | The ID of the team that the webhook watchs | 
**channel_id** | Option<**String**> | The ID of a public channel that the webhook watchs | [optional]
**creator_id** | Option<**String**> | The ID of the owner of the webhook if different than the requester. Required in [local mode](https://docs.mattermost.com/administration/mmctl-cli-tool.html#local-mode). | [optional]
**description** | Option<**String**> | The description for this outgoing webhook | [optional]
**display_name** | **String** | The display name for this outgoing webhook | 
**trigger_words** | **Vec<String>** | List of words for the webhook to trigger on | 
**trigger_when** | Option<**i32**> | When to trigger the webhook, `0` when a trigger word is present at all and `1` if the message starts with a trigger word | [optional]
**callback_urls** | **Vec<String>** | The URLs to POST the payloads to when the webhook is triggered | 
**content_type** | Option<**String**> | The format to POST the data in, either `application/json` or `application/x-www-form-urlencoded` | [optional][default to application/x-www-form-urlencoded]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


