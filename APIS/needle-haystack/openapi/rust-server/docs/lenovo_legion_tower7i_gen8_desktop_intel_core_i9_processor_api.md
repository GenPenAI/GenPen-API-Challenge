# lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_api

All URIs are relative to *http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**createLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor**](lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_api.md#createLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor) | **POST** /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor | Create LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
**createLenovoLegionTower7iGen8Desktop-IntelCoreI9ProcessorsWithListInput**](lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_api.md#createLenovoLegionTower7iGen8Desktop-IntelCoreI9ProcessorsWithListInput) | **POST** /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/createWithList | Creates list of LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor with given input array
**deleteLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor**](lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_api.md#deleteLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor) | **DELETE** /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name} | Delete lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
**getLenovoLegionTower7iGen8Desktop-IntelCoreI9ProcessorByName**](lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_api.md#getLenovoLegionTower7iGen8Desktop-IntelCoreI9ProcessorByName) | **GET** /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name} | Get lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor by name
**updateLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor**](lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_api.md#updateLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor) | **PUT** /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name} | Update lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor


# **createLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor**
> models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor createLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor(optional)
Create LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor** | [**LenovoLegionTower7iGen8DesktopIntelCoreI9Processor**](LenovoLegionTower7iGen8DesktopIntelCoreI9Processor.md)| Created lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor object | 

### Return type

[**models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor**](LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createLenovoLegionTower7iGen8Desktop-IntelCoreI9ProcessorsWithListInput**
> models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor createLenovoLegionTower7iGen8Desktop-IntelCoreI9ProcessorsWithListInput(optional)
Creates list of LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor with given input array

Creates list of lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor with given input array

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor** | [**LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor**](LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor.md)|  | 

### Return type

[**models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor**](LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor**
> deleteLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor(name)
Delete lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor that needs to be deleted by name | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getLenovoLegionTower7iGen8Desktop-IntelCoreI9ProcessorByName**
> models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor getLenovoLegionTower7iGen8Desktop-IntelCoreI9ProcessorByName(name)
Get lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor by name



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The name that needs to be fetched. Use lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor1 for testing.  | 

### Return type

[**models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor**](LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor**
> updateLenovoLegionTower7iGen8Desktop-IntelCoreI9Processor(name, optional)
Update lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| name of lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor that needs to be deleted | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor that needs to be deleted | 
 **lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor** | [**LenovoLegionTower7iGen8DesktopIntelCoreI9Processor**](LenovoLegionTower7iGen8DesktopIntelCoreI9Processor.md)| Update an existent lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor in the system | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

