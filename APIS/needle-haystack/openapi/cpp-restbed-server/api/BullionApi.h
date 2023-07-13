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
 * BullionApi.h
 *
 * 
 */

#ifndef BullionApi_H_
#define BullionApi_H_


#include <memory>
#include <utility>
#include <exception>
#include <functional>

#include <corvusoft/restbed/session.hpp>
#include <corvusoft/restbed/resource.hpp>
#include <corvusoft/restbed/request.hpp>
#include <corvusoft/restbed/service.hpp>
#include <corvusoft/restbed/settings.hpp>

#include "Bullion.h"
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
class  BullionApiException: public std::exception
{
public:
    BullionApiException(int status_code, std::string what);

    int getStatus() const;
    const char* what() const noexcept override;

private:
    int m_status;
    std::string m_what;
};

namespace BullionApiResources {
/// <summary>
/// Create Bullion
/// </summary>
/// <remarks>
/// This can only be done by the logged in user.
/// </remarks>
class  BullionResource: public restbed::Resource
{
public:
    BullionResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~BullionResource() = default;

    BullionResource(
        const BullionResource& other) = default; // copy constructor
    BullionResource(BullionResource&& other) noexcept = default; // move constructor

    BullionResource& operator=(const BullionResource& other) = default; // copy assignment
    BullionResource& operator=(BullionResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<std::pair<int, Bullion>(
        Bullion & bullion)> handler_POST_func =
            [](Bullion &) -> std::pair<int, Bullion>
                { throw BullionApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual std::pair<int, Bullion> handler_POST(
        Bullion & bullion);


protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleBullionApiException(const BullionApiException& e);
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
/// Creates list of Bullion with given input array
/// </summary>
/// <remarks>
/// Creates list of bullion with given input array
/// </remarks>
class  BullionCreateWithListResource: public restbed::Resource
{
public:
    BullionCreateWithListResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~BullionCreateWithListResource() = default;

    BullionCreateWithListResource(
        const BullionCreateWithListResource& other) = default; // copy constructor
    BullionCreateWithListResource(BullionCreateWithListResource&& other) noexcept = default; // move constructor

    BullionCreateWithListResource& operator=(const BullionCreateWithListResource& other) = default; // copy assignment
    BullionCreateWithListResource& operator=(BullionCreateWithListResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<std::pair<int, Bullion>(
        std::vector<Bullion> & bullion)> handler_POST_func =
            [](std::vector<Bullion> &) -> std::pair<int, Bullion>
                { throw BullionApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual std::pair<int, Bullion> handler_POST(
        std::vector<Bullion> & bullion);


protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleBullionApiException(const BullionApiException& e);
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
/// Delete bullion
/// </summary>
/// <remarks>
/// This can only be done by the logged in user.
/// </remarks>
class  BullionNameResource: public restbed::Resource
{
public:
    BullionNameResource(const std::string& context = "/openapi-jaxrs-server-1.0.0/api/v3");
    virtual ~BullionNameResource() = default;

    BullionNameResource(
        const BullionNameResource& other) = default; // copy constructor
    BullionNameResource(BullionNameResource&& other) noexcept = default; // move constructor

    BullionNameResource& operator=(const BullionNameResource& other) = default; // copy assignment
    BullionNameResource& operator=(BullionNameResource&& other) noexcept = default; // move assignment

    /////////////////////////////////////////////////////
    // Set these to implement the server functionality //
    /////////////////////////////////////////////////////
    std::function<int(
        std::string & name)> handler_DELETE_func =
            [](std::string &) -> int
                { throw BullionApiException(501, "Not implemented"); };

    std::function<std::pair<int, Bullion>(
        std::string & name)> handler_GET_func =
            [](std::string &) -> std::pair<int, Bullion>
                { throw BullionApiException(501, "Not implemented"); };

    std::function<int(
        std::string & name, Bullion & bullion)> handler_PUT_func =
            [](std::string &, Bullion &) -> int
                { throw BullionApiException(501, "Not implemented"); };


protected:
    //////////////////////////////////////////////////////////
    // As an alternative to setting the `std::function`s    //
    // override these to implement the server functionality //
    //////////////////////////////////////////////////////////

    virtual int handler_DELETE(
        std::string & name);

    virtual std::pair<int, Bullion> handler_GET(
        std::string & name);
    virtual int handler_PUT(
        std::string & name, Bullion & bullion);

protected:
    //////////////////////////////////////
    // Override these for customization //
    //////////////////////////////////////

    virtual std::string extractBodyContent(const std::shared_ptr<restbed::Session>& session);
    virtual std::string extractFormParamsFromBody(const std::string& paramName, const std::string& body);

    virtual std::pair<int, std::string> handleBullionApiException(const BullionApiException& e);
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

} /* namespace BullionApiResources */

using BullionApiBullionResource [[deprecated]] = BullionApiResources::BullionResource;
using BullionApiBullionCreateWithListResource [[deprecated]] = BullionApiResources::BullionCreateWithListResource;
using BullionApiBullionNameResource [[deprecated]] = BullionApiResources::BullionNameResource;

//
// The restbed service to actually implement the REST server
//
class  BullionApi
{
public:
    explicit BullionApi(std::shared_ptr<restbed::Service> const& restbedService);
	virtual ~BullionApi();

    std::shared_ptr<BullionApiResources::BullionResource> getBullionResource();
    std::shared_ptr<BullionApiResources::BullionCreateWithListResource> getBullionCreateWithListResource();
    std::shared_ptr<BullionApiResources::BullionNameResource> getBullionNameResource();

    void setResource(std::shared_ptr<BullionApiResources::BullionResource> resource);
    void setResource(std::shared_ptr<BullionApiResources::BullionCreateWithListResource> resource);
    void setResource(std::shared_ptr<BullionApiResources::BullionNameResource> resource);
    [[deprecated("use setResource()")]]
    virtual void setBullionApiBullionResource(std::shared_ptr<BullionApiResources::BullionResource> spBullionApiBullionResource);
    [[deprecated("use setResource()")]]
    virtual void setBullionApiBullionCreateWithListResource(std::shared_ptr<BullionApiResources::BullionCreateWithListResource> spBullionApiBullionCreateWithListResource);
    [[deprecated("use setResource()")]]
    virtual void setBullionApiBullionNameResource(std::shared_ptr<BullionApiResources::BullionNameResource> spBullionApiBullionNameResource);

    virtual void publishDefaultResources();

    virtual std::shared_ptr<restbed::Service> service();

protected:
	std::shared_ptr<BullionApiResources::BullionResource> m_spBullionResource;
	std::shared_ptr<BullionApiResources::BullionCreateWithListResource> m_spBullionCreateWithListResource;
	std::shared_ptr<BullionApiResources::BullionNameResource> m_spBullionNameResource;

private:
    std::shared_ptr<restbed::Service> m_service;
};


}
}
}
}

#endif /* BullionApi_H_ */

