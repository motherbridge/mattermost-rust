# RelationalIntegrityCheckData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parent_name** | Option<**String**> | the name of the parent relation (table). | [optional]
**child_name** | Option<**String**> | the name of the child relation (table). | [optional]
**parent_id_attr** | Option<**String**> | the name of the attribute (column) containing the parent id. | [optional]
**child_id_attr** | Option<**String**> | the name of the attribute (column) containing the child id. | [optional]
**records** | Option<[**Vec<crate::models::OrphanedRecord>**](OrphanedRecord.md)> | the list of orphaned records found. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


