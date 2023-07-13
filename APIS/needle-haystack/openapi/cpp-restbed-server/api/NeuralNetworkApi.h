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
 * NeuralNetworkApi.h
 *
 * 
 */

#ifndef NeuralNetworkApi_H_
#define NeuralNetworkApi_H_


#include <memory>
#include <utility>
#include <exception>
#include <functional>

#include <corvusoft/restbed/session.hpp>
#include <corvusoft/restbed/resource.hpp>
#include <corvusoft/restbed/request.hpp>
#include <corvusoft/restbed/service.hpp>
#include <corvusoft/restbed/settings.hpp>

#include "NeuralNetwork.h"
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
class  NeuralNetworkApiException: public std::exception
{
public:
    NeuralNetworkApiException(int status_code, std::string what);

    int getStatus() const;
    const char* what() const noexcept override;

private:
    int m_status;
    std::string m_what;
};

namespace NeuralNetworkApiResources {
/// <summary>
/// Create NeuralNetwork
/// </summary>
/// <remarks>
/// This can only be done by the logged in user.
/// </remarks>
class  NeuralNetworkResource: public restbed::Resource
{
public:
    NeuralNetworkResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~NeuralNetworkResource() = default;

    NeuralNetworkResource(
        const NeuralNetworkResource& other) = default; // copy constructor
    NeuralNetworkResource(NeuralNetworkResource&& other) noexcept = default; // move constructor

    NeuralNetworkResource& operator=(const NeuralNetworkResource& other) = default; // copy assignment
    NeuralNetworkResource& operator=(NeuralNetworkResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<std::pair<int, NeuralNetwork>(
        NeuralNetwork & neuralNetwork)> handler_POST_func =
            [](NeuralNetwork &) -> std::pair<int, NeuralNetwork>
                { throw NeuralNetworkApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual std::pair<int, NeuralNetwork> handler_POST(
        NeuralNetwork & neuralNetwork);


protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleNeuralNetworkApiException(const NeuralNetworkApiException& e);
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
/// Creates list of NeuralNetwork with given input array
/// </summary>
/// <remarks>
/// Creates list of neuralNetwork with given input array
/// </remarks>
class  NeuralNetworkCreateWithListResource: public restbed::Resource
{
public:
    NeuralNetworkCreateWithListResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~NeuralNetworkCreateWithListResource() = default;

    NeuralNetworkCreateWithListResource(
        const NeuralNetworkCreateWithListResource& other) = default; // copy constructor
    NeuralNetworkCreateWithListResource(NeuralNetworkCreateWithListResource&& other) noexcept = default; // move constructor

    NeuralNetworkCreateWithListResource& operator=(const NeuralNetworkCreateWithListResource& other) = default; // copy assignment
    NeuralNetworkCreateWithListResource& operator=(NeuralNetworkCreateWithListResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<std::pair<int, NeuralNetwork>(
        std::vector<NeuralNetwork> & neuralNetwork)> handler_POST_func =
            [](std::vector<NeuralNetwork> &) -> std::pair<int, NeuralNetwork>
                { throw NeuralNetworkApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual std::pair<int, NeuralNetwork> handler_POST(
        std::vector<NeuralNetwork> & neuralNetwork);


protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleNeuralNetworkApiException(const NeuralNetworkApiException& e);
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
/// Delete neuralNetwork
/// </summary>
/// <remarks>
/// This can only be done by the logged in user.
/// </remarks>
class  NeuralNetworkNameResource: public restbed::Resource
{
public:
    NeuralNetworkNameResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~NeuralNetworkNameResource() = default;

    NeuralNetworkNameResource(
        const NeuralNetworkNameResource& other) = default; // copy constructor
    NeuralNetworkNameResource(NeuralNetworkNameResource&& other) noexcept = default; // move constructor

    NeuralNetworkNameResource& operator=(const NeuralNetworkNameResource& other) = default; // copy assignment
    NeuralNetworkNameResource& operator=(NeuralNetworkNameResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<int(
        std::string & name)> handler_DELETE_func =
            [](std::string &) -> int
                { throw NeuralNetworkApiException(501, "Not implemented"); };

    std::function<std::pair<int, NeuralNetwork>(
        std::string & name)> handler_GET_func =
            [](std::string &) -> std::pair<int, NeuralNetwork>
                { throw NeuralNetworkApiException(501, "Not implemented"); };

    std::function<int(
        std::string & name, NeuralNetwork & neuralNetwork)> handler_PUT_func =
            [](std::string &, NeuralNetwork &) -> int
                { throw NeuralNetworkApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual int handler_DELETE(
        std::string & name);

    virtual std::pair<int, NeuralNetwork> handler_GET(
        std::string & name);
    virtual int handler_PUT(
        std::string & name, NeuralNetwork & neuralNetwork);

protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleNeuralNetworkApiException(const NeuralNetworkApiException& e);
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

} /* namespace NeuralNetworkApiResources */

using NeuralNetworkApiNeuralNetworkResource [[deprecated]] = NeuralNetworkApiResources::NeuralNetworkResource;
using NeuralNetworkApiNeuralNetworkCreateWithListResource [[deprecated]] = NeuralNetworkApiResources::NeuralNetworkCreateWithListResource;
using NeuralNetworkApiNeuralNetworkNameResource [[deprecated]] = NeuralNetworkApiResources::NeuralNetworkNameResource;

//
// The restbed service to actually implement the REST server
//
class  NeuralNetworkApi
{
public:
    explicit NeuralNetworkApi(std::shared_ptr<restbed::Service> const& restbedService);
	virtual ~NeuralNetworkApi();

    std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkResource> getNeuralNetworkResource();
    std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkCreateWithListResource> getNeuralNetworkCreateWithListResource();
    std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkNameResource> getNeuralNetworkNameResource();

    void setResource(std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkResource> resource);
    void setResource(std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkCreateWithListResource> resource);
    void setResource(std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkNameResource> resource);
    [[deprecated("use setResource()")]]
    virtual void setNeuralNetworkApiNeuralNetworkResource(std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkResource> spNeuralNetworkApiNeuralNetworkResource);
    [[deprecated("use setResource()")]]
    virtual void setNeuralNetworkApiNeuralNetworkCreateWithListResource(std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkCreateWithListResource> spNeuralNetworkApiNeuralNetworkCreateWithListResource);
    [[deprecated("use setResource()")]]
    virtual void setNeuralNetworkApiNeuralNetworkNameResource(std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkNameResource> spNeuralNetworkApiNeuralNetworkNameResource);

    virtual void publishDefaultResources();

    virtual std::shared_ptr<restbed::Service> service();

protected:
	std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkResource> m_spNeuralNetworkResource;
	std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkCreateWithListResource> m_spNeuralNetworkCreateWithListResource;
	std::shared_ptr<NeuralNetworkApiResources::NeuralNetworkNameResource> m_spNeuralNetworkNameResource;

private:
    std::shared_ptr<restbed::Service> m_service;
};


}
}
}
}

#endif /* NeuralNetworkApi_H_ */

