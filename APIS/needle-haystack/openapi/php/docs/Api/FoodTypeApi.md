# OpenAPI\Client\FoodTypeApi

All URIs are relative to http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3, except if the operation defines another base path.

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**createFoodType()**](FoodTypeApi.md#createFoodType) | **POST** /foodType | Create FoodType |
| [**createFoodTypesWithListInput()**](FoodTypeApi.md#createFoodTypesWithListInput) | **POST** /foodType/createWithList | Creates list of FoodType with given input array |
| [**deleteFoodType()**](FoodTypeApi.md#deleteFoodType) | **DELETE** /foodType/{name} | Delete foodType |
| [**getFoodTypeByName()**](FoodTypeApi.md#getFoodTypeByName) | **GET** /foodType/{name} | Get foodType by name |
| [**updateFoodType()**](FoodTypeApi.md#updateFoodType) | **PUT** /foodType/{name} | Update foodType |


## `createFoodType()`

```php
createFoodType($food_type): \OpenAPI\Client\Model\FoodType
```

Create FoodType

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FoodTypeApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$food_type = new \OpenAPI\Client\Model\FoodType(); // \OpenAPI\Client\Model\FoodType | Created foodType object

try {
    $result = $apiInstance->createFoodType($food_type);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling FoodTypeApi->createFoodType: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **food_type** | [**\OpenAPI\Client\Model\FoodType**](../Model/FoodType.md)| Created foodType object | [optional] |

### Return type

[**\OpenAPI\Client\Model\FoodType**](../Model/FoodType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`, `application/xml`, `application/x-www-form-urlencoded`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `createFoodTypesWithListInput()`

```php
createFoodTypesWithListInput($food_type): \OpenAPI\Client\Model\FoodType
```

Creates list of FoodType with given input array

Creates list of foodType with given input array

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FoodTypeApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$food_type = array(new \OpenAPI\Client\Model\FoodType()); // \OpenAPI\Client\Model\FoodType[]

try {
    $result = $apiInstance->createFoodTypesWithListInput($food_type);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling FoodTypeApi->createFoodTypesWithListInput: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **food_type** | [**\OpenAPI\Client\Model\FoodType[]**](../Model/FoodType.md)|  | [optional] |

### Return type

[**\OpenAPI\Client\Model\FoodType**](../Model/FoodType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `deleteFoodType()`

```php
deleteFoodType($name)
```

Delete foodType

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FoodTypeApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The foodType that needs to be deleted by name

try {
    $apiInstance->deleteFoodType($name);
} catch (Exception $e) {
    echo 'Exception when calling FoodTypeApi->deleteFoodType: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The foodType that needs to be deleted by name | |

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `getFoodTypeByName()`

```php
getFoodTypeByName($name): \OpenAPI\Client\Model\FoodType
```

Get foodType by name



### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FoodTypeApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The name that needs to be fetched. Use foodType1 for testing.

try {
    $result = $apiInstance->getFoodTypeByName($name);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling FoodTypeApi->getFoodTypeByName: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The name that needs to be fetched. Use foodType1 for testing. | |

### Return type

[**\OpenAPI\Client\Model\FoodType**](../Model/FoodType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `updateFoodType()`

```php
updateFoodType($name, $food_type)
```

Update foodType

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FoodTypeApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | name of foodType that needs to be deleted
$food_type = new \OpenAPI\Client\Model\FoodType(); // \OpenAPI\Client\Model\FoodType | Update an existent foodType in the system

try {
    $apiInstance->updateFoodType($name, $food_type);
} catch (Exception $e) {
    echo 'Exception when calling FoodTypeApi->updateFoodType: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| name of foodType that needs to be deleted | |
| **food_type** | [**\OpenAPI\Client\Model\FoodType**](../Model/FoodType.md)| Update an existent foodType in the system | [optional] |

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`, `application/xml`, `application/x-www-form-urlencoded`
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)
