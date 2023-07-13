# alienware_aurora_r15_gaming_desktop_api

All URIs are relative to *http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**createAlienwareAuroraR15GamingDesktop**](alienware_aurora_r15_gaming_desktop_api.md#createAlienwareAuroraR15GamingDesktop) | **POST** /alienwareAuroraR15GamingDesktop | Create AlienwareAuroraR15GamingDesktop
**createAlienwareAuroraR15GamingDesktopsWithListInput**](alienware_aurora_r15_gaming_desktop_api.md#createAlienwareAuroraR15GamingDesktopsWithListInput) | **POST** /alienwareAuroraR15GamingDesktop/createWithList | Creates list of AlienwareAuroraR15GamingDesktop with given input array
**deleteAlienwareAuroraR15GamingDesktop**](alienware_aurora_r15_gaming_desktop_api.md#deleteAlienwareAuroraR15GamingDesktop) | **DELETE** /alienwareAuroraR15GamingDesktop/{name} | Delete alienwareAuroraR15GamingDesktop
**getAlienwareAuroraR15GamingDesktopByName**](alienware_aurora_r15_gaming_desktop_api.md#getAlienwareAuroraR15GamingDesktopByName) | **GET** /alienwareAuroraR15GamingDesktop/{name} | Get alienwareAuroraR15GamingDesktop by name
**updateAlienwareAuroraR15GamingDesktop**](alienware_aurora_r15_gaming_desktop_api.md#updateAlienwareAuroraR15GamingDesktop) | **PUT** /alienwareAuroraR15GamingDesktop/{name} | Update alienwareAuroraR15GamingDesktop


# **createAlienwareAuroraR15GamingDesktop**
> models::AlienwareAuroraR15GamingDesktop createAlienwareAuroraR15GamingDesktop(optional)
Create AlienwareAuroraR15GamingDesktop

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **alienware_aurora_r15_gaming_desktop** | [**AlienwareAuroraR15GamingDesktop**](AlienwareAuroraR15GamingDesktop.md)| Created alienwareAuroraR15GamingDesktop object | 

### Return type

[**models::AlienwareAuroraR15GamingDesktop**](AlienwareAuroraR15GamingDesktop.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createAlienwareAuroraR15GamingDesktopsWithListInput**
> models::AlienwareAuroraR15GamingDesktop createAlienwareAuroraR15GamingDesktopsWithListInput(optional)
Creates list of AlienwareAuroraR15GamingDesktop with given input array

Creates list of alienwareAuroraR15GamingDesktop with given input array

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **alienware_aurora_r15_gaming_desktop** | [**AlienwareAuroraR15GamingDesktop**](AlienwareAuroraR15GamingDesktop.md)|  | 

### Return type

[**models::AlienwareAuroraR15GamingDesktop**](AlienwareAuroraR15GamingDesktop.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteAlienwareAuroraR15GamingDesktop**
> deleteAlienwareAuroraR15GamingDesktop(name)
Delete alienwareAuroraR15GamingDesktop

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The alienwareAuroraR15GamingDesktop that needs to be deleted by name | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAlienwareAuroraR15GamingDesktopByName**
> models::AlienwareAuroraR15GamingDesktop getAlienwareAuroraR15GamingDesktopByName(name)
Get alienwareAuroraR15GamingDesktop by name



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The name that needs to be fetched. Use alienwareAuroraR15GamingDesktop1 for testing.  | 

### Return type

[**models::AlienwareAuroraR15GamingDesktop**](AlienwareAuroraR15GamingDesktop.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateAlienwareAuroraR15GamingDesktop**
> updateAlienwareAuroraR15GamingDesktop(name, optional)
Update alienwareAuroraR15GamingDesktop

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| name of alienwareAuroraR15GamingDesktop that needs to be deleted | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of alienwareAuroraR15GamingDesktop that needs to be deleted | 
 **alienware_aurora_r15_gaming_desktop** | [**AlienwareAuroraR15GamingDesktop**](AlienwareAuroraR15GamingDesktop.md)| Update an existent alienwareAuroraR15GamingDesktop in the system | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

