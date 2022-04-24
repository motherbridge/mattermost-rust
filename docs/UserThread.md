# UserThread

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the post that is this thread's root | [optional]
**reply_count** | Option<**i64**> | number of replies in this thread | [optional]
**last_reply_at** | Option<**i64**> | timestamp of the last post to this thread | [optional]
**last_viewed_at** | Option<**i64**> | timestamp of the last time the user viewed this thread | [optional]
**participants** | Option<[**Vec<crate::models::Post>**](Post.md)> | list of users participating in this thread. only includes IDs unless 'extended' was set to 'true' | [optional]
**post** | Option<[**crate::models::Post**](Post.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


