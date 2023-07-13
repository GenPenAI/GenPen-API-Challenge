# OpenAPI\Client\ShakesApi

All URIs are relative to http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3, except if the operation defines another base path.

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**createShakes()**](ShakesApi.md#createShakes) | **POST** /shakes | Create Shakes |
| [**createShakessWithListInput()**](ShakesApi.md#createShakessWithListInput) | **POST** /shakes/createWithList | Creates list of Shakes with given input array |
| [**deleteShakes()**](ShakesApi.md#deleteShakes) | **DELETE** /shakes/{name} | Delete shakes |
| [**getShakesByName()**](ShakesApi.md#getShakesByName) | **GET** /shakes/{name} | Get shakes by name |
| [**updateShakes()**](ShakesApi.md#updateShakes) | **PUT** /shakes/{name} | Update shakes |


## `createShakes()`

```php
createShakes($shakes): \OpenAPI\Client\Model\Shakes
```

Create Shakes

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\ShakesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$shakes = new \OpenAPI\Client\Model\Shakes(); // \OpenAPI\Client\Model\Shakes | Created shakes object

try {
    $result = $apiInstance->createShakes($shakes);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ShakesApi->createShakes: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **shakes** | [**\OpenAPI\Client\Model\Shakes**](../Model/Shakes.md)| Created shakes object | [optional] |

### Return type

[**\OpenAPI\Client\Model\Shakes**](../Model/Shakes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`, `application/xml`, `application/x-www-form-urlencoded`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `createShakessWithListInput()`

```php
createShakessWithListInput($shakes): \OpenAPI\Client\Model\Shakes
```

Creates list of Shakes with given input array

Creates list of shakes with given input array

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\ShakesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$shakes = array(new \OpenAPI\Client\Model\Shakes()); // \OpenAPI\Client\Model\Shakes[]

try {
    $result = $apiInstance->createShakessWithListInput($shakes);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ShakesApi->createShakessWithListInput: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **shakes** | [**\OpenAPI\Client\Model\Shakes[]**](../Model/Shakes.md)|  | [optional] |

### Return type

[**\OpenAPI\Client\Model\Shakes**](../Model/Shakes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `deleteShakes()`

```php
deleteShakes($name)
```

Delete shakes

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\ShakesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The shakes that needs to be deleted by name

try {
    $apiInstance->deleteShakes($name);
} catch (Exception $e) {
    echo 'Exception when calling ShakesApi->deleteShakes: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The shakes that needs to be deleted by name | |

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

## `getShakesByName()`

```php
getShakesByName($name): \OpenAPI\Client\Model\Shakes
```

Get shakes by name



### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\ShakesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The name that needs to be fetched. Use shakes1 for testing.

try {
    $result = $apiInstance->getShakesByName($name);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ShakesApi->getShakesByName: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The name that needs to be fetched. Use shakes1 for testing. | |

### Return type

[**\OpenAPI\Client\Model\Shakes**](../Model/Shakes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `updateShakes()`

```php
updateShakes($name, $shakes)
```

Update shakes

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\ShakesApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | name of shakes that needs to be deleted
$shakes = new \OpenAPI\Client\Model\Shakes(); // \OpenAPI\Client\Model\Shakes | Update an existent shakes in the system

try {
    $apiInstance->updateShakes($name, $shakes);
} catch (Exception $e) {
    echo 'Exception when calling ShakesApi->updateShakes: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| name of shakes that needs to be deleted | |
| **shakes** | [**\OpenAPI\Client\Model\Shakes**](../Model/Shakes.md)| Update an existent shakes in the system | [optional] |

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
