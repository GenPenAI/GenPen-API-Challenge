/**
 * API Inspector
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI-Generator 6.2.1.
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

/*
 * CNNNeuralNetworkApi.h
 *
 * 
 */

#ifndef CNNNeuralNetworkApi_H_
#define CNNNeuralNetworkApi_H_


#include <memory>
#include <utility>
#include <exception>
#include <functional>

#include <corvusoft/restbed/session.hpp>
#include <corvusoft/restbed/resource.hpp>
#include <corvusoft/restbed/request.hpp>
#include <corvusoft/restbed/service.hpp>
#include <corvusoft/restbed/settings.hpp>

#include "CnnNeuralNetwork.h"
#include <string>
#include <vector>

namespace org {
namespace openapitools {
namespace server {
namespace api {

using namespace org::openapitools::server::model;

///
/// Exception to flag problems in the handlers
///
class  CNNNeuralNetworkApiException: public std::exception
{
public:
    CNNNeuralNetworkApiException(int status_code, std::string what);

    int getStatus() const;
    const char* what() const noexcept override;

private:
    int m_status;
    std::string m_what;
};

namespace CNNNeuralNetworkApiResources {
/// <summary>
/// Create CnnNeuralNetwork
/// </summary>
/// <remarks>
/// This can only be done by the logged in user.
/// </remarks>
class  CNNNeuralNetworkResource: public restbed::Resource
{
public:
    CNNNeuralNetworkResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~CNNNeuralNetworkResource() = default;

    CNNNeuralNetworkResource(
        const CNNNeuralNetworkResource& other) = default; // copy constructor
    CNNNeuralNetworkResource(CNNNeuralNetworkResource&& other) noexcept = default; // move constructor

    CNNNeuralNetworkResource& operator=(const CNNNeuralNetworkResource& other) = default; // copy assignment
    CNNNeuralNetworkResource& operator=(CNNNeuralNetworkResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<std::pair<int, CnnNeuralNetwork>(
        CnnNeuralNetwork & cnnNeuralNetwork)> handler_POST_func =
            [](CnnNeuralNetwork &) -> std::pair<int, CnnNeuralNetwork>
                { throw CNNNeuralNetworkApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual std::pair<int, CnnNeuralNetwork> handler_POST(
        CnnNeuralNetwork & cnnNeuralNetwork);


protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleCNNNeuralNetworkApiException(const CNNNeuralNetworkApiException& e);
    virtual std::pair<int, std::string> handleStdException(const std::exception& e);
    virtual std::pair<int, std::string> handleUnspecifiedException();

    virtual void setResponseHeader(const std::shared_ptr<restbed::Session>& session,
        const std::string& header);

    virtual void returnResponse(const std::shared_ptr<restbed::Session>& session,
        const int status, const std::string& result, std::multimap<std::string, std::string>& contentType);
    virtual void defaultSessionClose(const std::shared_ptr<restbed::Session>& session,
        const int status, const std::string& result);

private:
    void handler_POST_internal(const std::shared_ptr<restbed::Session> session);
};

/// <summary>
/// Creates list of CnnNeuralNetwork with given input array
/// </summary>
/// <remarks>
/// Creates list of cNNNeuralNetwork with given input array
/// </remarks>
class  CNNNeuralNetworkCreateWithListResource: public restbed::Resource
{
public:
    CNNNeuralNetworkCreateWithListResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~CNNNeuralNetworkCreateWithListResource() = default;

    CNNNeuralNetworkCreateWithListResource(
        const CNNNeuralNetworkCreateWithListResource& other) = default; // copy constructor
    CNNNeuralNetworkCreateWithListResource(CNNNeuralNetworkCreateWithListResource&& other) noexcept = default; // move constructor

    CNNNeuralNetworkCreateWithListResource& operator=(const CNNNeuralNetworkCreateWithListResource& other) = default; // copy assignment
    CNNNeuralNetworkCreateWithListResource& operator=(CNNNeuralNetworkCreateWithListResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<std::pair<int, CnnNeuralNetwork>(
        std::vector<CnnNeuralNetwork> & cnnNeuralNetwork)> handler_POST_func =
            [](std::vector<CnnNeuralNetwork> &) -> std::pair<int, CnnNeuralNetwork>
                { throw CNNNeuralNetworkApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual std::pair<int, CnnNeuralNetwork> handler_POST(
        std::vector<CnnNeuralNetwork> & cnnNeuralNetwork);


protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleCNNNeuralNetworkApiException(const CNNNeuralNetworkApiException& e);
    virtual std::pair<int, std::string> handleStdException(const std::exception& e);
    virtual std::pair<int, std::string> handleUnspecifiedException();

    virtual void setResponseHeader(const std::shared_ptr<restbed::Session>& session,
        const std::string& header);

    virtual void returnResponse(const std::shared_ptr<restbed::Session>& session,
        const int status, const std::string& result, std::multimap<std::string, std::string>& contentType);
    virtual void defaultSessionClose(const std::shared_ptr<restbed::Session>& session,
        const int status, const std::string& result);

private:
    void handler_POST_internal(const std::shared_ptr<restbed::Session> session);
};

/// <summary>
/// Delete cNNNeuralNetwork
/// </summary>
/// <remarks>
/// This can only be done by the logged in user.
/// </remarks>
class  CNNNeuralNetworkNameResource: public restbed::Resource
{
public:
    CNNNeuralNetworkNameResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~CNNNeuralNetworkNameResource() = default;

    CNNNeuralNetworkNameResource(
        const CNNNeuralNetworkNameResource& other) = default; // copy constructor
    CNNNeuralNetworkNameResource(CNNNeuralNetworkNameResource&& other) noexcept = default; // move constructor

    CNNNeuralNetworkNameResource& operator=(const CNNNeuralNetworkNameResource& other) = default; // copy assignment
    CNNNeuralNetworkNameResource& operator=(CNNNeuralNetworkNameResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<int(
        std::string & name)> handler_DELETE_func =
            [](std::string &) -> int
                { throw CNNNeuralNetworkApiException(501, "Not implemented"); };

    std::function<std::pair<int, CnnNeuralNetwork>(
        std::string & name)> handler_GET_func =
            [](std::string &) -> std::pair<int, CnnNeuralNetwork>
                { throw CNNNeuralNetworkApiException(501, "Not implemented"); };

    std::function<int(
        std::string & name, CnnNeuralNetwork & cnnNeuralNetwork)> handler_PUT_func =
            [](std::string &, CnnNeuralNetwork &) -> int
                { throw CNNNeuralNetworkApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual int handler_DELETE(
        std::string & name);

    virtual std::pair<int, CnnNeuralNetwork> handler_GET(
        std::string & name);
    virtual int handler_PUT(
        std::string & name, CnnNeuralNetwork & cnnNeuralNetwork);

protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleCNNNeuralNetworkApiException(const CNNNeuralNetworkApiException& e);
    virtual std::pair<int, std::string> handleStdException(const std::exception& e);
    virtual std::pair<int, std::string> handleUnspecifiedException();

    virtual void setResponseHeader(const std::shared_ptr<restbed::Session>& session,
        const std::string& header);

    virtual void returnResponse(const std::shared_ptr<restbed::Session>& session,
        const int status, const std::string& result, std::multimap<std::string, std::string>& contentType);
    virtual void defaultSessionClose(const std::shared_ptr<restbed::Session>& session,
        const int status, const std::string& result);

private:
    void handler_DELETE_internal(const std::shared_ptr<restbed::Session> session);
    void handler_GET_internal(const std::shared_ptr<restbed::Session> session);
    void handler_PUT_internal(const std::shared_ptr<restbed::Session> session);
};

} /* namespace CNNNeuralNetworkApiResources */

using CNNNeuralNetworkApiCNNNeuralNetworkResource [[deprecated]] = CNNNeuralNetworkApiResources::CNNNeuralNetworkResource;
using CNNNeuralNetworkApiCNNNeuralNetworkCreateWithListResource [[deprecated]] = CNNNeuralNetworkApiResources::CNNNeuralNetworkCreateWithListResource;
using CNNNeuralNetworkApiCNNNeuralNetworkNameResource [[deprecated]] = CNNNeuralNetworkApiResources::CNNNeuralNetworkNameResource;

//
// The restbed service to actually implement the REST server
//
class  CNNNeuralNetworkApi
{
public:
    explicit CNNNeuralNetworkApi(std::shared_ptr<restbed::Service> const& restbedService);
	virtual ~CNNNeuralNetworkApi();

    std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkResource> getCNNNeuralNetworkResource();
    std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkCreateWithListResource> getCNNNeuralNetworkCreateWithListResource();
    std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkNameResource> getCNNNeuralNetworkNameResource();

    void setResource(std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkResource> resource);
    void setResource(std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkCreateWithListResource> resource);
    void setResource(std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkNameResource> resource);
    [[deprecated("use setResource()")]]
    virtual void setCNNNeuralNetworkApiCNNNeuralNetworkResource(std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkResource> spCNNNeuralNetworkApiCNNNeuralNetworkResource);
    [[deprecated("use setResource()")]]
    virtual void setCNNNeuralNetworkApiCNNNeuralNetworkCreateWithListResource(std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkCreateWithListResource> spCNNNeuralNetworkApiCNNNeuralNetworkCreateWithListResource);
    [[deprecated("use setResource()")]]
    virtual void setCNNNeuralNetworkApiCNNNeuralNetworkNameResource(std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkNameResource> spCNNNeuralNetworkApiCNNNeuralNetworkNameResource);

    virtual void publishDefaultResources();

    virtual std::shared_ptr<restbed::Service> service();

protected:
	std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkResource> m_spCNNNeuralNetworkResource;
	std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkCreateWithListResource> m_spCNNNeuralNetworkCreateWithListResource;
	std::shared_ptr<CNNNeuralNetworkApiResources::CNNNeuralNetworkNameResource> m_spCNNNeuralNetworkNameResource;

private:
    std::shared_ptr<restbed::Service> m_service;
};


}
}
}
}

#endif /* CNNNeuralNetworkApi_H_ */

