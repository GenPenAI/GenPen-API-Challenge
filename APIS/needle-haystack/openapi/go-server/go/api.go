/*
 * API Inspector
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.0.0
 * Generated by: OpenAPI Generator (https://openapi-generator.tech)
 */

package openapi

import (
	"context"
	"net/http"
)



// UserApiRouter defines the required methods for binding the api requests to a responses for the UserApi
// The UserApiRouter implementation should parse necessary information from the http request,
// pass the data to a UserApiServicer to perform the required actions, then write the service results to the http response.
type UserApiRouter interface { 
	CreateUser(http.ResponseWriter, *http.Request)
	CreateUsersWithListInput(http.ResponseWriter, *http.Request)
	DeleteUser(http.ResponseWriter, *http.Request)
	GetUserByName(http.ResponseWriter, *http.Request)
	LoginUser(http.ResponseWriter, *http.Request)
	LogoutUser(http.ResponseWriter, *http.Request)
	UpdateUser(http.ResponseWriter, *http.Request)
}


// UserApiServicer defines the api actions for the UserApi service
// This interface intended to stay up to date with the openapi yaml used to generate it,
// while the service implementation can be ignored with the .openapi-generator-ignore file
// and updated with the logic required for the API.
type UserApiServicer interface { 
	CreateUser(context.Context, User) (ImplResponse, error)
	CreateUsersWithListInput(context.Context, []User) (ImplResponse, error)
	DeleteUser(context.Context, string) (ImplResponse, error)
	GetUserByName(context.Context, string) (ImplResponse, error)
	LoginUser(context.Context, string, string) (ImplResponse, error)
	LogoutUser(context.Context) (ImplResponse, error)
	UpdateUser(context.Context, string, User) (ImplResponse, error)
}
