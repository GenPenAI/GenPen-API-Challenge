# OpenAPI\Client\RestaurantApi

All URIs are relative to http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3, except if the operation defines another base path.

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**createRestaurant()**](RestaurantApi.md#createRestaurant) | **POST** /restaurant | Create Restaurant |
| [**createRestaurantsWithListInput()**](RestaurantApi.md#createRestaurantsWithListInput) | **POST** /restaurant/createWithList | Creates list of Restaurant with given input array |
| [**deleteRestaurant()**](RestaurantApi.md#deleteRestaurant) | **DELETE** /restaurant/{name} | Delete restaurant |
| [**getRestaurantByName()**](RestaurantApi.md#getRestaurantByName) | **GET** /restaurant/{name} | Get restaurant by name |
| [**updateRestaurant()**](RestaurantApi.md#updateRestaurant) | **PUT** /restaurant/{name} | Update restaurant |


## `createRestaurant()`

```php
createRestaurant($restaurant): \OpenAPI\Client\Model\Restaurant
```

Create Restaurant

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\RestaurantApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$restaurant = new \OpenAPI\Client\Model\Restaurant(); // \OpenAPI\Client\Model\Restaurant | Created restaurant object

try {
    $result = $apiInstance->createRestaurant($restaurant);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling RestaurantApi->createRestaurant: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **restaurant** | [**\OpenAPI\Client\Model\Restaurant**](../Model/Restaurant.md)| Created restaurant object | [optional] |

### Return type

[**\OpenAPI\Client\Model\Restaurant**](../Model/Restaurant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`, `application/xml`, `application/x-www-form-urlencoded`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `createRestaurantsWithListInput()`

```php
createRestaurantsWithListInput($restaurant): \OpenAPI\Client\Model\Restaurant
```

Creates list of Restaurant with given input array

Creates list of restaurant with given input array

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\RestaurantApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$restaurant = array(new \OpenAPI\Client\Model\Restaurant()); // \OpenAPI\Client\Model\Restaurant[]

try {
    $result = $apiInstance->createRestaurantsWithListInput($restaurant);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling RestaurantApi->createRestaurantsWithListInput: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **restaurant** | [**\OpenAPI\Client\Model\Restaurant[]**](../Model/Restaurant.md)|  | [optional] |

### Return type

[**\OpenAPI\Client\Model\Restaurant**](../Model/Restaurant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `deleteRestaurant()`

```php
deleteRestaurant($name)
```

Delete restaurant

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\RestaurantApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The restaurant that needs to be deleted by name

try {
    $apiInstance->deleteRestaurant($name);
} catch (Exception $e) {
    echo 'Exception when calling RestaurantApi->deleteRestaurant: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The restaurant that needs to be deleted by name | |

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

## `getRestaurantByName()`

```php
getRestaurantByName($name): \OpenAPI\Client\Model\Restaurant
```

Get restaurant by name



### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\RestaurantApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The name that needs to be fetched. Use restaurant1 for testing.

try {
    $result = $apiInstance->getRestaurantByName($name);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling RestaurantApi->getRestaurantByName: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The name that needs to be fetched. Use restaurant1 for testing. | |

### Return type

[**\OpenAPI\Client\Model\Restaurant**](../Model/Restaurant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `updateRestaurant()`

```php
updateRestaurant($name, $restaurant)
```

Update restaurant

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\RestaurantApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | name of restaurant that needs to be deleted
$restaurant = new \OpenAPI\Client\Model\Restaurant(); // \OpenAPI\Client\Model\Restaurant | Update an existent restaurant in the system

try {
    $apiInstance->updateRestaurant($name, $restaurant);
} catch (Exception $e) {
    echo 'Exception when calling RestaurantApi->updateRestaurant: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| name of restaurant that needs to be deleted | |
| **restaurant** | [**\OpenAPI\Client\Model\Restaurant**](../Model/Restaurant.md)| Update an existent restaurant in the system | [optional] |

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
