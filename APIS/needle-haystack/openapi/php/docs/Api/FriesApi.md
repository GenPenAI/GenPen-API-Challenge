# OpenAPI\Client\FriesApi

All URIs are relative to http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3, except if the operation defines another base path.

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**createFries()**](FriesApi.md#createFries) | **POST** /fries | Create Fries |
| [**createFriessWithListInput()**](FriesApi.md#createFriessWithListInput) | **POST** /fries/createWithList | Creates list of Fries with given input array |
| [**deleteFries()**](FriesApi.md#deleteFries) | **DELETE** /fries/{name} | Delete fries |
| [**getFriesByName()**](FriesApi.md#getFriesByName) | **GET** /fries/{name} | Get fries by name |
| [**updateFries()**](FriesApi.md#updateFries) | **PUT** /fries/{name} | Update fries |


## `createFries()`

```php
createFries($fries): \OpenAPI\Client\Model\Fries
```

Create Fries

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FriesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$fries = new \OpenAPI\Client\Model\Fries(); // \OpenAPI\Client\Model\Fries | Created fries object

try {
    $result = $apiInstance->createFries($fries);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling FriesApi->createFries: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **fries** | [**\OpenAPI\Client\Model\Fries**](../Model/Fries.md)| Created fries object | [optional] |

### Return type

[**\OpenAPI\Client\Model\Fries**](../Model/Fries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`, `application/xml`, `application/x-www-form-urlencoded`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `createFriessWithListInput()`

```php
createFriessWithListInput($fries): \OpenAPI\Client\Model\Fries
```

Creates list of Fries with given input array

Creates list of fries with given input array

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FriesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$fries = array(new \OpenAPI\Client\Model\Fries()); // \OpenAPI\Client\Model\Fries[]

try {
    $result = $apiInstance->createFriessWithListInput($fries);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling FriesApi->createFriessWithListInput: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **fries** | [**\OpenAPI\Client\Model\Fries[]**](../Model/Fries.md)|  | [optional] |

### Return type

[**\OpenAPI\Client\Model\Fries**](../Model/Fries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `deleteFries()`

```php
deleteFries($name)
```

Delete fries

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FriesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The fries that needs to be deleted by name

try {
    $apiInstance->deleteFries($name);
} catch (Exception $e) {
    echo 'Exception when calling FriesApi->deleteFries: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The fries that needs to be deleted by name | |

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

## `getFriesByName()`

```php
getFriesByName($name): \OpenAPI\Client\Model\Fries
```

Get fries by name



### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FriesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The name that needs to be fetched. Use fries1 for testing.

try {
    $result = $apiInstance->getFriesByName($name);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling FriesApi->getFriesByName: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The name that needs to be fetched. Use fries1 for testing. | |

### Return type

[**\OpenAPI\Client\Model\Fries**](../Model/Fries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `updateFries()`

```php
updateFries($name, $fries)
```

Update fries

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\FriesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | name of fries that needs to be deleted
$fries = new \OpenAPI\Client\Model\Fries(); // \OpenAPI\Client\Model\Fries | Update an existent fries in the system

try {
    $apiInstance->updateFries($name, $fries);
} catch (Exception $e) {
    echo 'Exception when calling FriesApi->updateFries: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| name of fries that needs to be deleted | |
| **fries** | [**\OpenAPI\Client\Model\Fries**](../Model/Fries.md)| Update an existent fries in the system | [optional] |

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
