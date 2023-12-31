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
 * ReinforcementLearningAgentApi.h
 *
 * 
 */

#ifndef ReinforcementLearningAgentApi_H_
#define ReinforcementLearningAgentApi_H_


#include <memory>
#include <utility>
#include <exception>
#include <functional>

#include <corvusoft/restbed/session.hpp>
#include <corvusoft/restbed/resource.hpp>
#include <corvusoft/restbed/request.hpp>
#include <corvusoft/restbed/service.hpp>
#include <corvusoft/restbed/settings.hpp>

#include "ReinforcementLearningAgent.h"
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
class  ReinforcementLearningAgentApiException: public std::exception
{
public:
    ReinforcementLearningAgentApiException(int status_code, std::string what);

    int getStatus() const;
    const char* what() const noexcept override;

private:
    int m_status;
    std::string m_what;
};

namespace ReinforcementLearningAgentApiResources {
/// <summary>
/// Create ReinforcementLearningAgent
/// </summary>
/// <remarks>
/// This can only be done by the logged in user.
/// </remarks>
class  ReinforcementLearningAgentResource: public restbed::Resource
{
public:
    ReinforcementLearningAgentResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~ReinforcementLearningAgentResource() = default;

    ReinforcementLearningAgentResource(
        const ReinforcementLearningAgentResource& other) = default; // copy constructor
    ReinforcementLearningAgentResource(ReinforcementLearningAgentResource&& other) noexcept = default; // move constructor

    ReinforcementLearningAgentResource& operator=(const ReinforcementLearningAgentResource& other) = default; // copy assignment
    ReinforcementLearningAgentResource& operator=(ReinforcementLearningAgentResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<std::pair<int, ReinforcementLearningAgent>(
        ReinforcementLearningAgent & reinforcementLearningAgent)> handler_POST_func =
            [](ReinforcementLearningAgent &) -> std::pair<int, ReinforcementLearningAgent>
                { throw ReinforcementLearningAgentApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual std::pair<int, ReinforcementLearningAgent> handler_POST(
        ReinforcementLearningAgent & reinforcementLearningAgent);


protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleReinforcementLearningAgentApiException(const ReinforcementLearningAgentApiException& e);
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
/// Creates list of ReinforcementLearningAgent with given input array
/// </summary>
/// <remarks>
/// Creates list of reinforcementLearningAgent with given input array
/// </remarks>
class  ReinforcementLearningAgentCreateWithListResource: public restbed::Resource
{
public:
    ReinforcementLearningAgentCreateWithListResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~ReinforcementLearningAgentCreateWithListResource() = default;

    ReinforcementLearningAgentCreateWithListResource(
        const ReinforcementLearningAgentCreateWithListResource& other) = default; // copy constructor
    ReinforcementLearningAgentCreateWithListResource(ReinforcementLearningAgentCreateWithListResource&& other) noexcept = default; // move constructor

    ReinforcementLearningAgentCreateWithListResource& operator=(const ReinforcementLearningAgentCreateWithListResource& other) = default; // copy assignment
    ReinforcementLearningAgentCreateWithListResource& operator=(ReinforcementLearningAgentCreateWithListResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<std::pair<int, ReinforcementLearningAgent>(
        std::vector<ReinforcementLearningAgent> & reinforcementLearningAgent)> handler_POST_func =
            [](std::vector<ReinforcementLearningAgent> &) -> std::pair<int, ReinforcementLearningAgent>
                { throw ReinforcementLearningAgentApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual std::pair<int, ReinforcementLearningAgent> handler_POST(
        std::vector<ReinforcementLearningAgent> & reinforcementLearningAgent);


protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleReinforcementLearningAgentApiException(const ReinforcementLearningAgentApiException& e);
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
/// Delete reinforcementLearningAgent
/// </summary>
/// <remarks>
/// This can only be done by the logged in user.
/// </remarks>
class  ReinforcementLearningAgentNameResource: public restbed::Resource
{
public:
    ReinforcementLearningAgentNameResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~ReinforcementLearningAgentNameResource() = default;

    ReinforcementLearningAgentNameResource(
        const ReinforcementLearningAgentNameResource& other) = default; // copy constructor
    ReinforcementLearningAgentNameResource(ReinforcementLearningAgentNameResource&& other) noexcept = default; // move constructor

    ReinforcementLearningAgentNameResource& operator=(const ReinforcementLearningAgentNameResource& other) = default; // copy assignment
    ReinforcementLearningAgentNameResource& operator=(ReinforcementLearningAgentNameResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<int(
        std::string & name)> handler_DELETE_func =
            [](std::string &) -> int
                { throw ReinforcementLearningAgentApiException(501, "Not implemented"); };

    std::function<std::pair<int, ReinforcementLearningAgent>(
        std::string & name)> handler_GET_func =
            [](std::string &) -> std::pair<int, ReinforcementLearningAgent>
                { throw ReinforcementLearningAgentApiException(501, "Not implemented"); };

    std::function<int(
        std::string & name, ReinforcementLearningAgent & reinforcementLearningAgent)> handler_PUT_func =
            [](std::string &, ReinforcementLearningAgent &) -> int
                { throw ReinforcementLearningAgentApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual int handler_DELETE(
        std::string & name);

    virtual std::pair<int, ReinforcementLearningAgent> handler_GET(
        std::string & name);
    virtual int handler_PUT(
        std::string & name, ReinforcementLearningAgent & reinforcementLearningAgent);

protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleReinforcementLearningAgentApiException(const ReinforcementLearningAgentApiException& e);
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

} /* namespace ReinforcementLearningAgentApiResources */

using ReinforcementLearningAgentApiReinforcementLearningAgentResource [[deprecated]] = ReinforcementLearningAgentApiResources::ReinforcementLearningAgentResource;
using ReinforcementLearningAgentApiReinforcementLearningAgentCreateWithListResource [[deprecated]] = ReinforcementLearningAgentApiResources::ReinforcementLearningAgentCreateWithListResource;
using ReinforcementLearningAgentApiReinforcementLearningAgentNameResource [[deprecated]] = ReinforcementLearningAgentApiResources::ReinforcementLearningAgentNameResource;

//
// The restbed service to actually implement the REST server
//
class  ReinforcementLearningAgentApi
{
public:
    explicit ReinforcementLearningAgentApi(std::shared_ptr<restbed::Service> const& restbedService);
	virtual ~ReinforcementLearningAgentApi();

    std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentResource> getReinforcementLearningAgentResource();
    std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentCreateWithListResource> getReinforcementLearningAgentCreateWithListResource();
    std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentNameResource> getReinforcementLearningAgentNameResource();

    void setResource(std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentResource> resource);
    void setResource(std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentCreateWithListResource> resource);
    void setResource(std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentNameResource> resource);
    [[deprecated("use setResource()")]]
    virtual void setReinforcementLearningAgentApiReinforcementLearningAgentResource(std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentResource> spReinforcementLearningAgentApiReinforcementLearningAgentResource);
    [[deprecated("use setResource()")]]
    virtual void setReinforcementLearningAgentApiReinforcementLearningAgentCreateWithListResource(std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentCreateWithListResource> spReinforcementLearningAgentApiReinforcementLearningAgentCreateWithListResource);
    [[deprecated("use setResource()")]]
    virtual void setReinforcementLearningAgentApiReinforcementLearningAgentNameResource(std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentNameResource> spReinforcementLearningAgentApiReinforcementLearningAgentNameResource);

    virtual void publishDefaultResources();

    virtual std::shared_ptr<restbed::Service> service();

protected:
	std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentResource> m_spReinforcementLearningAgentResource;
	std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentCreateWithListResource> m_spReinforcementLearningAgentCreateWithListResource;
	std::shared_ptr<ReinforcementLearningAgentApiResources::ReinforcementLearningAgentNameResource> m_spReinforcementLearningAgentNameResource;

private:
    std::shared_ptr<restbed::Service> m_service;
};


}
}
}
}

#endif /* ReinforcementLearningAgentApi_H_ */

