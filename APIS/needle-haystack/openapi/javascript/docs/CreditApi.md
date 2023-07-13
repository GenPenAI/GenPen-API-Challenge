# ApiInspector.CreditApi

All URIs are relative to *http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createCredit**](CreditApi.md#createCredit) | **POST** /credit | Create Credit
[**createCreditsWithListInput**](CreditApi.md#createCreditsWithListInput) | **POST** /credit/createWithList | Creates list of Credit with given input array
[**deleteCredit**](CreditApi.md#deleteCredit) | **DELETE** /credit/{name} | Delete credit
[**getCreditByName**](CreditApi.md#getCreditByName) | **GET** /credit/{name} | Get credit by name
[**updateCredit**](CreditApi.md#updateCredit) | **PUT** /credit/{name} | Update credit



## createCredit

> Credit createCredit(opts)

Create Credit

This can only be done by the logged in user.

### Example

```javascript
import ApiInspector from 'api_inspector';

let apiInstance = new ApiInspector.CreditApi();
let opts = {
  'credit': new ApiInspector.Credit() // Credit | Created credit object
};
apiInstance.createCredit(opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **credit** | [**Credit**](Credit.md)| Created credit object | [optional] 

### Return type

[**Credit**](Credit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
- **Accept**: application/json, application/xml


## createCreditsWithListInput

> Credit createCreditsWithListInput(opts)

Creates list of Credit with given input array

Creates list of credit with given input array

### Example

```javascript
import ApiInspector from 'api_inspector';

let apiInstance = new ApiInspector.CreditApi();
let opts = {
  'credit': [new ApiInspector.Credit()] // [Credit] | 
};
apiInstance.createCreditsWithListInput(opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **credit** | [**[Credit]**](Credit.md)|  | [optional] 

### Return type

[**Credit**](Credit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml


## deleteCredit

> deleteCredit(name)

Delete credit

This can only be done by the logged in user.

### Example

```javascript
import ApiInspector from 'api_inspector';

let apiInstance = new ApiInspector.CreditApi();
let name = "name_example"; // String | The credit that needs to be deleted by name
apiInstance.deleteCredit(name, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully.');
  }
});
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| The credit that needs to be deleted by name | 

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined


## getCreditByName

> Credit getCreditByName(name)

Get credit by name



### Example

```javascript
import ApiInspector from 'api_inspector';

let apiInstance = new ApiInspector.CreditApi();
let name = "name_example"; // String | The name that needs to be fetched. Use credit1 for testing. 
apiInstance.getCreditByName(name, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| The name that needs to be fetched. Use credit1 for testing.  | 

### Return type

[**Credit**](Credit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml


## updateCredit

> updateCredit(name, opts)

Update credit

This can only be done by the logged in user.

### Example

```javascript
import ApiInspector from 'api_inspector';

let apiInstance = new ApiInspector.CreditApi();
let name = "name_example"; // String | name of credit that needs to be deleted
let opts = {
  'credit': new ApiInspector.Credit() // Credit | Update an existent credit in the system
};
apiInstance.updateCredit(name, opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully.');
  }
});
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of credit that needs to be deleted | 
 **credit** | [**Credit**](Credit.md)| Update an existent credit in the system | [optional] 

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
- **Accept**: Not defined

