<?php

/**
 * API Inspector
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 * PHP version 7.2.5
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI-Generator
 * https://openapi-generator.tech
 * Do not edit the class manually.
 *
 * Source files are located at:
 *
 * > https://github.com/OpenAPITools/openapi-generator/blob/master/modules/openapi-generator/src/main/resources/php-laravel/
 */


use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;

/**
 * post createUser
 * Summary: Create user
 * Notes: This can only be done by the logged in user.
 * Output-Formats: [application/json, application/xml]
 */
Route::post('/openapi-jaxrs-server-1.0.0/api/v3/user', 'UserController@createUser');
/**
 * post createUsersWithListInput
 * Summary: Creates list of users with given input array
 * Notes: Creates list of users with given input array
 * Output-Formats: [application/xml, application/json]
 */
Route::post('/openapi-jaxrs-server-1.0.0/api/v3/user/createWithList', 'UserController@createUsersWithListInput');
/**
 * get loginUser
 * Summary: Logs user into the system
 * Notes: 
 * Output-Formats: [application/xml, application/json]
 */
Route::get('/openapi-jaxrs-server-1.0.0/api/v3/user/login', 'UserController@loginUser');
/**
 * get logoutUser
 * Summary: Logs out current logged in user session
 * Notes: 

 */
Route::get('/openapi-jaxrs-server-1.0.0/api/v3/user/logout', 'UserController@logoutUser');
/**
 * delete deleteUser
 * Summary: Delete user
 * Notes: This can only be done by the logged in user.

 */
Route::delete('/openapi-jaxrs-server-1.0.0/api/v3/user/{username}', 'UserController@deleteUser');
/**
 * get getUserByName
 * Summary: Get user by user name
 * Notes: 
 * Output-Formats: [application/xml, application/json]
 */
Route::get('/openapi-jaxrs-server-1.0.0/api/v3/user/{username}', 'UserController@getUserByName');
/**
 * put updateUser
 * Summary: Update user
 * Notes: This can only be done by the logged in user.

 */
Route::put('/openapi-jaxrs-server-1.0.0/api/v3/user/{username}', 'UserController@updateUser');

Route::middleware('auth:api')->get('/user', function (Request $request) {
    return $request->user();
});