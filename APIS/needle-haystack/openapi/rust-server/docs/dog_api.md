# dog_api

All URIs are relative to *http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**createDog**](dog_api.md#createDog) | **POST** /dog | Create Dog
**createDogsWithListInput**](dog_api.md#createDogsWithListInput) | **POST** /dog/createWithList | Creates list of Dog with given input array
**deleteDog**](dog_api.md#deleteDog) | **DELETE** /dog/{name} | Delete dog
**getDogByName**](dog_api.md#getDogByName) | **GET** /dog/{name} | Get dog by name
**updateDog**](dog_api.md#updateDog) | **PUT** /dog/{name} | Update dog


# **createDog**
> models::Dog createDog(optional)
Create Dog

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dog** | [**Dog**](Dog.md)| Created dog object | 

### Return type

[**models::Dog**](Dog.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createDogsWithListInput**
> models::Dog createDogsWithListInput(optional)
Creates list of Dog with given input array

Creates list of dog with given input array

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **dog** | [**Dog**](Dog.md)|  | 

### Return type

[**models::Dog**](Dog.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteDog**
> deleteDog(name)
Delete dog

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The dog that needs to be deleted by name | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getDogByName**
> models::Dog getDogByName(name)
Get dog by name



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| The name that needs to be fetched. Use dog1 for testing.  | 

### Return type

[**models::Dog**](Dog.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateDog**
> updateDog(name, optional)
Update dog

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **name** | **String**| name of dog that needs to be deleted | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of dog that needs to be deleted | 
 **dog** | [**Dog**](Dog.md)| Update an existent dog in the system | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

