# dell274k_uh_d_monitor_s2721_qs_api

All URIs are relative to *http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**createDell274kUhdMonitor-S2721qs**](dell274k_uh_d_monitor_s2721_qs_api.md#createDell274kUhdMonitor-S2721qs) | **POST** /dell274kUhDMonitor-S2721QS | Create Dell274kUhdMonitor-S2721qs
**createDell274kUhdMonitor-S2721qssWithListInput**](dell274k_uh_d_monitor_s2721_qs_api.md#createDell274kUhdMonitor-S2721qssWithListInput) | **POST** /dell274kUhDMonitor-S2721QS/createWithList | Creates list of Dell274kUhdMonitor-S2721qs with given input array
**deleteDell274kUhdMonitor-S2721qs**](dell274k_uh_d_monitor_s2721_qs_api.md#deleteDell274kUhdMonitor-S2721qs) | **DELETE** /dell274kUhDMonitor-S2721QS/{name} | Delete dell274kUhDMonitor-S2721QS
**getDell274kUhdMonitor-S2721qsByName**](dell274k_uh_d_monitor_s2721_qs_api.md#getDell274kUhdMonitor-S2721qsByName) | **GET** /dell274kUhDMonitor-S2721QS/{name} | Get dell274kUhDMonitor-S2721QS by name
**updateDell274kUhdMonitor-S2721qs**](dell274k_uh_d_monitor_s2721_qs_api.md#updateDell274kUhdMonitor-S2721qs) | **PUT** /dell274kUhDMonitor-S2721QS/{name} | Update dell274kUhDMonitor-S2721QS


# **createDell274kUhdMonitor-S2721qs**
> models::Dell274kUhdMonitorS2721qs createDell274kUhdMonitor-S2721qs(optional)
Create Dell274kUhdMonitor-S2721qs

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dell274k_uhd_monitor_s2721qs** | [**Dell274kUhdMonitorS2721qs**](Dell274kUhdMonitorS2721qs.md)| Created dell274kUhDMonitor-S2721QS object | 

### Return type

[**models::Dell274kUhdMonitorS2721qs**](Dell274kUhdMonitor-S2721qs.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createDell274kUhdMonitor-S2721qssWithListInput**
> models::Dell274kUhdMonitorS2721qs createDell274kUhdMonitor-S2721qssWithListInput(optional)
Creates list of Dell274kUhdMonitor-S2721qs with given input array

Creates list of dell274kUhDMonitor-S2721QS with given input array

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dell274k_uhd_monitor_s2721qs** | [**Dell274kUhdMonitor-S2721qs**](Dell274kUhdMonitor-S2721qs.md)|  | 

### Return type

[**models::Dell274kUhdMonitorS2721qs**](Dell274kUhdMonitor-S2721qs.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteDell274kUhdMonitor-S2721qs**
> deleteDell274kUhdMonitor-S2721qs(name)
Delete dell274kUhDMonitor-S2721QS

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The dell274kUhDMonitor-S2721QS that needs to be deleted by name | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getDell274kUhdMonitor-S2721qsByName**
> models::Dell274kUhdMonitorS2721qs getDell274kUhdMonitor-S2721qsByName(name)
Get dell274kUhDMonitor-S2721QS by name



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The name that needs to be fetched. Use dell274kUhDMonitor-S2721QS1 for testing.  | 

### Return type

[**models::Dell274kUhdMonitorS2721qs**](Dell274kUhdMonitor-S2721qs.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateDell274kUhdMonitor-S2721qs**
> updateDell274kUhdMonitor-S2721qs(name, optional)
Update dell274kUhDMonitor-S2721QS

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| name of dell274kUhDMonitor-S2721QS that needs to be deleted | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of dell274kUhDMonitor-S2721QS that needs to be deleted | 
 **dell274k_uhd_monitor_s2721qs** | [**Dell274kUhdMonitorS2721qs**](Dell274kUhdMonitorS2721qs.md)| Update an existent dell274kUhDMonitor-S2721QS in the system | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

