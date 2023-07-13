# cat_api

All URIs are relative to *http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**createCat**](cat_api.md#createCat) | **POST** /cat | Create Cat
**createCatsWithListInput**](cat_api.md#createCatsWithListInput) | **POST** /cat/createWithList | Creates list of Cat with given input array
**deleteCat**](cat_api.md#deleteCat) | **DELETE** /cat/{name} | Delete cat
**getCatByName**](cat_api.md#getCatByName) | **GET** /cat/{name} | Get cat by name
**updateCat**](cat_api.md#updateCat) | **PUT** /cat/{name} | Update cat


# **createCat**
> models::Cat createCat(optional)
Create Cat

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cat** | [**Cat**](Cat.md)| Created cat object | 

### Return type

[**models::Cat**](Cat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createCatsWithListInput**
> models::Cat createCatsWithListInput(optional)
Creates list of Cat with given input array

Creates list of cat with given input array

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cat** | [**Cat**](Cat.md)|  | 

### Return type

[**models::Cat**](Cat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteCat**
> deleteCat(name)
Delete cat

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The cat that needs to be deleted by name | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getCatByName**
> models::Cat getCatByName(name)
Get cat by name



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The name that needs to be fetched. Use cat1 for testing.  | 

### Return type

[**models::Cat**](Cat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateCat**
> updateCat(name, optional)
Update cat

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| name of cat that needs to be deleted | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of cat that needs to be deleted | 
 **cat** | [**Cat**](Cat.md)| Update an existent cat in the system | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

