# OpenAPI\Client\BurgersApi

All URIs are relative to http://localhost:8080/openapi-jaxrs-server-1.0.0/api/v3, except if the operation defines another base path.

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**createBurgers()**](BurgersApi.md#createBurgers) | **POST** /burgers | Create Burgers |
| [**createBurgerssWithListInput()**](BurgersApi.md#createBurgerssWithListInput) | **POST** /burgers/createWithList | Creates list of Burgers with given input array |
| [**deleteBurgers()**](BurgersApi.md#deleteBurgers) | **DELETE** /burgers/{name} | Delete burgers |
| [**getBurgersByName()**](BurgersApi.md#getBurgersByName) | **GET** /burgers/{name} | Get burgers by name |
| [**updateBurgers()**](BurgersApi.md#updateBurgers) | **PUT** /burgers/{name} | Update burgers |


## `createBurgers()`

```php
createBurgers($burgers): \OpenAPI\Client\Model\Burgers
```

Create Burgers

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\BurgersApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$burgers = new \OpenAPI\Client\Model\Burgers(); // \OpenAPI\Client\Model\Burgers | Created burgers object

try {
    $result = $apiInstance->createBurgers($burgers);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling BurgersApi->createBurgers: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **burgers** | [**\OpenAPI\Client\Model\Burgers**](../Model/Burgers.md)| Created burgers object | [optional] |

### Return type

[**\OpenAPI\Client\Model\Burgers**](../Model/Burgers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`, `application/xml`, `application/x-www-form-urlencoded`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `createBurgerssWithListInput()`

```php
createBurgerssWithListInput($burgers): \OpenAPI\Client\Model\Burgers
```

Creates list of Burgers with given input array

Creates list of burgers with given input array

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\BurgersApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$burgers = array(new \OpenAPI\Client\Model\Burgers()); // \OpenAPI\Client\Model\Burgers[]

try {
    $result = $apiInstance->createBurgerssWithListInput($burgers);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling BurgersApi->createBurgerssWithListInput: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **burgers** | [**\OpenAPI\Client\Model\Burgers[]**](../Model/Burgers.md)|  | [optional] |

### Return type

[**\OpenAPI\Client\Model\Burgers**](../Model/Burgers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `deleteBurgers()`

```php
deleteBurgers($name)
```

Delete burgers

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\BurgersApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The burgers that needs to be deleted by name

try {
    $apiInstance->deleteBurgers($name);
} catch (Exception $e) {
    echo 'Exception when calling BurgersApi->deleteBurgers: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The burgers that needs to be deleted by name | |

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

## `getBurgersByName()`

```php
getBurgersByName($name): \OpenAPI\Client\Model\Burgers
```

Get burgers by name



### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\BurgersApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | The name that needs to be fetched. Use burgers1 for testing.

try {
    $result = $apiInstance->getBurgersByName($name);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling BurgersApi->getBurgersByName: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| The name that needs to be fetched. Use burgers1 for testing. | |

### Return type

[**\OpenAPI\Client\Model\Burgers**](../Model/Burgers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`, `application/xml`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `updateBurgers()`

```php
updateBurgers($name, $burgers)
```

Update burgers

This can only be done by the logged in user.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\BurgersApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$name = 'name_example'; // string | name of burgers that needs to be deleted
$burgers = new \OpenAPI\Client\Model\Burgers(); // \OpenAPI\Client\Model\Burgers | Update an existent burgers in the system

try {
    $apiInstance->updateBurgers($name, $burgers);
} catch (Exception $e) {
    echo 'Exception when calling BurgersApi->updateBurgers: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **name** | **string**| name of burgers that needs to be deleted | |
| **burgers** | [**\OpenAPI\Client\Model\Burgers**](../Model/Burgers.md)| Update an existent burgers in the system | [optional] |

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
