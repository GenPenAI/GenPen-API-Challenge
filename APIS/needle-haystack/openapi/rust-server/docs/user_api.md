# user_api

All URIs are relative to *http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**createUser**](user_api.md#createUser) | **POST** /user | Create user
**createUsersWithListInput**](user_api.md#createUsersWithListInput) | **POST** /user/createWithList | Creates list of users with given input array
**deleteUser**](user_api.md#deleteUser) | **DELETE** /user/{username} | Delete user
**getUserByName**](user_api.md#getUserByName) | **GET** /user/{username} | Get user by user name
**loginUser**](user_api.md#loginUser) | **GET** /user/login | Logs user into the system
**logoutUser**](user_api.md#logoutUser) | **GET** /user/logout | Logs out current logged in user session
**updateUser**](user_api.md#updateUser) | **PUT** /user/{username} | Update user


# **createUser**
> models::User createUser(optional)
Create user

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user** | [**User**](User.md)| Created user object | 

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createUsersWithListInput**
> models::User createUsersWithListInput(optional)
Creates list of users with given input array

Creates list of users with given input array

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user** | [**User**](User.md)|  | 

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteUser**
> deleteUser(username)
Delete user

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **username** | **String**| The name that needs to be deleted | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUserByName**
> models::User getUserByName(username)
Get user by user name



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **username** | **String**| The name that needs to be fetched. Use user1 for testing.  | 

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **loginUser**
> String loginUser(optional)
Logs user into the system



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **username** | **String**| The user name for login | 
 **password** | **String**| The password for login in clear text | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **logoutUser**
> logoutUser()
Logs out current logged in user session



### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateUser**
> updateUser(username, optional)
Update user

This can only be done by the logged in user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **username** | **String**| name that need to be deleted | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **username** | **String**| name that need to be deleted | 
 **user** | [**User**](User.md)| Update an existent user in the store | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

