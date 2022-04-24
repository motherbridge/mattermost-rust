# Notice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Notice ID | [optional]
**sys_admin_only** | Option<**bool**> | Does this notice apply only to sysadmins | [optional]
**team_admin_only** | Option<**bool**> | Does this notice apply only to team admins | [optional]
**action** | Option<**String**> | Optional action to perform on action button click. (defaults to closing the notice) | [optional]
**action_param** | Option<**String**> | Optional action parameter.  Example: {\"action\": \"url\", actionParam: \"/console/some-page\"} | [optional]
**action_text** | Option<**String**> | Optional override for the action button text (defaults to OK) | [optional]
**description** | Option<**String**> | Notice content. Use {{Mattermost}} instead of plain text to support white-labeling. Text supports Markdown. | [optional]
**image** | Option<**String**> | URL of image to display | [optional]
**title** | Option<**String**> | Notice title. Use {{Mattermost}} instead of plain text to support white-labeling. Text supports Markdown. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


