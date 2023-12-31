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


#include <corvusoft/restbed/byte.hpp>
#include <corvusoft/restbed/string.hpp>
#include <corvusoft/restbed/settings.hpp>
#include <corvusoft/restbed/request.hpp>
#include <corvusoft/restbed/uri.hpp>
#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/json_parser.hpp>
#include <boost/lexical_cast.hpp>
#include <boost/algorithm/string.hpp>

#include "BullionApi.h"

namespace org {
namespace openapitools {
namespace server {
namespace api {

using namespace org::openapitools::server::model;

namespace {
[[maybe_unused]]
std::string selectPreferredContentType(const std::vector<std::string>& contentTypes) {
    if (contentTypes.size() == 0) {
        return "application/json";
    }

    if (contentTypes.size() == 1) {
        return contentTypes.at(0);
    }

    static const std::array<std::string, 2> preferredTypes = {"json", "xml"};
    for (const auto& preferredType: preferredTypes) {
        const auto ret = std::find_if(contentTypes.cbegin(),
        contentTypes.cend(),
        [preferredType](const std::string& str) {
            return str.find(preferredType) != std::string::npos;});
        if (ret != contentTypes.cend()) {
            return *ret;
        }
    }

    return contentTypes.at(0);
}
}

BullionApiException::BullionApiException(int status_code, std::string what)
  : m_status(status_code),
    m_what(what)
{

}
int BullionApiException::getStatus() const
{
    return m_status;
}
const char* BullionApiException::what() const noexcept
{
    return m_what.c_str();
}


template<class MODEL_T>
MODEL_T extractJsonModelBodyParam(const std::string& bodyContent)
{
    std::stringstream sstream(bodyContent);
    boost::property_tree::ptree pt;
    boost::property_tree::json_parser::read_json(sstream, pt);

    auto model = MODEL_T(pt);
    return model;
}

template<class MODEL_T>
std::vector<MODEL_T> extractJsonArrayBodyParam(const std::string& bodyContent)
{
    std::stringstream sstream(bodyContent);
    boost::property_tree::ptree pt;
    boost::property_tree::json_parser::read_json(sstream, pt);

    auto arrayRet = std::vector<MODEL_T>();
    for (const auto& child: pt) {
        arrayRet.emplace_back(MODEL_T(child.second));
    }
    return arrayRet;
}

template <class KEY_T, class VAL_T>
std::string convertMapResponse(const std::map<KEY_T, VAL_T>& map)
{
    boost::property_tree::ptree pt;
    for(const auto &kv: map) {
    pt.push_back(boost::property_tree::ptree::value_type(
        boost::lexical_cast<std::string>(kv.first),
        boost::property_tree::ptree(
        boost::lexical_cast<std::string>(kv.second))));
    }
    std::stringstream sstream;
    write_json(sstream, pt);
    std::string result = sstream.str();
    return result;
}

namespace BullionApiResources {
BullionResource::BullionResource(const std::string& context /* = "/openapi-jaxrs-server-1.0.0/api/v3" */)
{
	this->set_path(context + "/bullion");
	this->set_method_handler("POST",
		std::bind(&BullionResource::handler_POST_internal, this,
			std::placeholders::_1));
}

std::pair<int, std::string> BullionResource::handleBullionApiException(const BullionApiException& e)
{
    return std::make_pair<int, std::string>(e.getStatus(), e.what());
}

std::pair<int, std::string> BullionResource::handleStdException(const std::exception& e)
{
    return std::make_pair<int, std::string>(500, e.what());
}

std::pair<int, std::string> BullionResource::handleUnspecifiedException()
{
    return std::make_pair<int, std::string>(500, "Unknown exception occurred");
}

void BullionResource::setResponseHeader(const std::shared_ptr<restbed::Session>& session, const std::string& header)
{
    session->set_header(header, "");
}

void BullionResource::returnResponse(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result, std::multimap<std::string, std::string>& responseHeaders)
{
    responseHeaders.insert(std::make_pair("Connection", "close"));
    session->close(status, result, responseHeaders);
}

void BullionResource::defaultSessionClose(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result)
{
    session->close(status, result, { {"Connection", "close"} });
}

void BullionResource::handler_POST_internal(const std::shared_ptr<restbed::Session> session)
{
    const auto request = session->get_request();
    // body params or form params here from the body content string
    std::string bodyContent = extractBodyContent(session);
    auto bullion = extractJsonModelBodyParam<Bullion>(bodyContent);
    
    int status_code = 500;
    Bullion resultObject = Bullion{};
    std::string result = "";
    
    try {
        std::tie(status_code, resultObject) =
            handler_POST(bullion);
    }
    catch(const BullionApiException& e) {
        std::tie(status_code, result) = handleBullionApiException(e);
    }
    catch(const std::exception& e) {
        std::tie(status_code, result) = handleStdException(e);
    }
    catch(...) {
        std::tie(status_code, result) = handleUnspecifiedException();
    }
    
    std::multimap< std::string, std::string > responseHeaders {};
    static const std::vector<std::string> contentTypes{
        "application/json","application/xml",
    };
    static const std::string acceptTypes{
        "application/json, application/xml, application/x-www-form-urlencoded, "
    };
    
    if (status_code == 0) {
        responseHeaders.insert(std::make_pair("Content-Type", "text/plain"));
        result = "successful operation";
    
        result = resultObject.toJsonString();
        returnResponse(session, 0, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    defaultSessionClose(session, status_code, result);
}


std::pair<int, Bullion> BullionResource::handler_POST(
        Bullion & bullion)
{
    return handler_POST_func(bullion);
}


std::string BullionResource::extractBodyContent(const std::shared_ptr<restbed::Session>& session) {
  const auto request = session->get_request();
  int content_length = request->get_header("Content-Length", 0);
  std::string bodyContent;
  session->fetch(content_length,
                 [&bodyContent](const std::shared_ptr<restbed::Session> session,
                                const restbed::Bytes &body) {
                   bodyContent = restbed::String::format(
                       "%.*s\n", (int)body.size(), body.data());
                 });
  return bodyContent;
}

std::string BullionResource::extractFormParamsFromBody(const std::string& paramName, const std::string& body) {
    const auto uri = restbed::Uri("urlencoded?" + body, true);
    const auto params = uri.get_query_parameters();
    const auto result = params.find(paramName);
    if (result != params.cend()) {
        return result->second;
    }
    return "";
}
BullionCreateWithListResource::BullionCreateWithListResource(const std::string& context /* = "/openapi-jaxrs-server-1.0.0/api/v3" */)
{
	this->set_path(context + "/bullion/createWithList");
	this->set_method_handler("POST",
		std::bind(&BullionCreateWithListResource::handler_POST_internal, this,
			std::placeholders::_1));
}

std::pair<int, std::string> BullionCreateWithListResource::handleBullionApiException(const BullionApiException& e)
{
    return std::make_pair<int, std::string>(e.getStatus(), e.what());
}

std::pair<int, std::string> BullionCreateWithListResource::handleStdException(const std::exception& e)
{
    return std::make_pair<int, std::string>(500, e.what());
}

std::pair<int, std::string> BullionCreateWithListResource::handleUnspecifiedException()
{
    return std::make_pair<int, std::string>(500, "Unknown exception occurred");
}

void BullionCreateWithListResource::setResponseHeader(const std::shared_ptr<restbed::Session>& session, const std::string& header)
{
    session->set_header(header, "");
}

void BullionCreateWithListResource::returnResponse(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result, std::multimap<std::string, std::string>& responseHeaders)
{
    responseHeaders.insert(std::make_pair("Connection", "close"));
    session->close(status, result, responseHeaders);
}

void BullionCreateWithListResource::defaultSessionClose(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result)
{
    session->close(status, result, { {"Connection", "close"} });
}

void BullionCreateWithListResource::handler_POST_internal(const std::shared_ptr<restbed::Session> session)
{
    const auto request = session->get_request();
    // body params or form params here from the body content string
    std::string bodyContent = extractBodyContent(session);
    auto bullion = extractJsonArrayBodyParam<Bullion>(bodyContent);
    
    int status_code = 500;
    Bullion resultObject = Bullion{};
    std::string result = "";
    
    try {
        std::tie(status_code, resultObject) =
            handler_POST(bullion);
    }
    catch(const BullionApiException& e) {
        std::tie(status_code, result) = handleBullionApiException(e);
    }
    catch(const std::exception& e) {
        std::tie(status_code, result) = handleStdException(e);
    }
    catch(...) {
        std::tie(status_code, result) = handleUnspecifiedException();
    }
    
    std::multimap< std::string, std::string > responseHeaders {};
    static const std::vector<std::string> contentTypes{
        "application/json","application/xml",
    };
    static const std::string acceptTypes{
        "application/json, "
    };
    
    if (status_code == 200) {
        responseHeaders.insert(std::make_pair("Content-Type", selectPreferredContentType(contentTypes)));
        if (!acceptTypes.empty()) {
            responseHeaders.insert(std::make_pair("Accept", acceptTypes));
        }
    
        result = resultObject.toJsonString();
        returnResponse(session, 200, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    if (status_code == 0) {
        responseHeaders.insert(std::make_pair("Content-Type", "text/plain"));
        result = "successful operation";
    
        returnResponse(session, 0, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    defaultSessionClose(session, status_code, result);
}


std::pair<int, Bullion> BullionCreateWithListResource::handler_POST(
        std::vector<Bullion> & bullion)
{
    return handler_POST_func(bullion);
}


std::string BullionCreateWithListResource::extractBodyContent(const std::shared_ptr<restbed::Session>& session) {
  const auto request = session->get_request();
  int content_length = request->get_header("Content-Length", 0);
  std::string bodyContent;
  session->fetch(content_length,
                 [&bodyContent](const std::shared_ptr<restbed::Session> session,
                                const restbed::Bytes &body) {
                   bodyContent = restbed::String::format(
                       "%.*s\n", (int)body.size(), body.data());
                 });
  return bodyContent;
}

std::string BullionCreateWithListResource::extractFormParamsFromBody(const std::string& paramName, const std::string& body) {
    const auto uri = restbed::Uri("urlencoded?" + body, true);
    const auto params = uri.get_query_parameters();
    const auto result = params.find(paramName);
    if (result != params.cend()) {
        return result->second;
    }
    return "";
}
BullionNameResource::BullionNameResource(const std::string& context /* = "/openapi-jaxrs-server-1.0.0/api/v3" */)
{
	this->set_path(context + "/bullion/{name: .*}");
	this->set_method_handler("DELETE",
		std::bind(&BullionNameResource::handler_DELETE_internal, this,
			std::placeholders::_1));
	this->set_method_handler("GET",
		std::bind(&BullionNameResource::handler_GET_internal, this,
			std::placeholders::_1));
	this->set_method_handler("PUT",
		std::bind(&BullionNameResource::handler_PUT_internal, this,
			std::placeholders::_1));
}

std::pair<int, std::string> BullionNameResource::handleBullionApiException(const BullionApiException& e)
{
    return std::make_pair<int, std::string>(e.getStatus(), e.what());
}

std::pair<int, std::string> BullionNameResource::handleStdException(const std::exception& e)
{
    return std::make_pair<int, std::string>(500, e.what());
}

std::pair<int, std::string> BullionNameResource::handleUnspecifiedException()
{
    return std::make_pair<int, std::string>(500, "Unknown exception occurred");
}

void BullionNameResource::setResponseHeader(const std::shared_ptr<restbed::Session>& session, const std::string& header)
{
    session->set_header(header, "");
}

void BullionNameResource::returnResponse(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result, std::multimap<std::string, std::string>& responseHeaders)
{
    responseHeaders.insert(std::make_pair("Connection", "close"));
    session->close(status, result, responseHeaders);
}

void BullionNameResource::defaultSessionClose(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result)
{
    session->close(status, result, { {"Connection", "close"} });
}

void BullionNameResource::handler_DELETE_internal(const std::shared_ptr<restbed::Session> session)
{
    const auto request = session->get_request();
    // Getting the path params
    std::string name = request->get_path_parameter("name", "");
    
    int status_code = 500;
    std::string result = "";
    
    try {
        status_code =
            handler_DELETE(name);
    }
    catch(const BullionApiException& e) {
        std::tie(status_code, result) = handleBullionApiException(e);
    }
    catch(const std::exception& e) {
        std::tie(status_code, result) = handleStdException(e);
    }
    catch(...) {
        std::tie(status_code, result) = handleUnspecifiedException();
    }
    
    std::multimap< std::string, std::string > responseHeaders {};
    static const std::vector<std::string> contentTypes{
        "application/json"
    };
    static const std::string acceptTypes{
    };
    
    if (status_code == 400) {
        responseHeaders.insert(std::make_pair("Content-Type", "text/plain"));
        result = "Invalid Bullion variable supplied";
    
        returnResponse(session, 400, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    if (status_code == 404) {
        responseHeaders.insert(std::make_pair("Content-Type", "text/plain"));
        result = "Bullion not found";
    
        returnResponse(session, 404, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    defaultSessionClose(session, status_code, result);
}

// x-extension
void BullionNameResource::handler_GET_internal(const std::shared_ptr<restbed::Session> session) {
    const auto request = session->get_request();
    // Getting the path params
    std::string name = request->get_path_parameter("name", "");
    
    int status_code = 500;
    Bullion resultObject = Bullion{};
    std::string result = "";
    
    try {
        std::tie(status_code, resultObject) =
            handler_GET(name);
    }
    catch(const BullionApiException& e) {
        std::tie(status_code, result) = handleBullionApiException(e);
    }
    catch(const std::exception& e) {
        std::tie(status_code, result) = handleStdException(e);
    }
    catch(...) {
        std::tie(status_code, result) = handleUnspecifiedException();
    }
    
    std::multimap< std::string, std::string > responseHeaders {};
    static const std::vector<std::string> contentTypes{
        "application/json","application/xml",
    };
    static const std::string acceptTypes{
    };
    
    if (status_code == 200) {
        responseHeaders.insert(std::make_pair("Content-Type", selectPreferredContentType(contentTypes)));
        if (!acceptTypes.empty()) {
            responseHeaders.insert(std::make_pair("Accept", acceptTypes));
        }
    
        result = resultObject.toJsonString();
        returnResponse(session, 200, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    if (status_code == 400) {
        responseHeaders.insert(std::make_pair("Content-Type", "text/plain"));
        result = "Invalid Bullion variable supplied";
    
        returnResponse(session, 400, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    if (status_code == 404) {
        responseHeaders.insert(std::make_pair("Content-Type", "text/plain"));
        result = "Bullion not found";
    
        returnResponse(session, 404, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    defaultSessionClose(session, status_code, result);
}
// x-extension
void BullionNameResource::handler_PUT_internal(const std::shared_ptr<restbed::Session> session) {
    const auto request = session->get_request();
    // body params or form params here from the body content string
    std::string bodyContent = extractBodyContent(session);
    auto bullion = extractJsonModelBodyParam<Bullion>(bodyContent);
    // Getting the path params
    std::string name = request->get_path_parameter("name", "");
    
    int status_code = 500;
    std::string result = "";
    
    try {
        status_code =
            handler_PUT(name, bullion);
    }
    catch(const BullionApiException& e) {
        std::tie(status_code, result) = handleBullionApiException(e);
    }
    catch(const std::exception& e) {
        std::tie(status_code, result) = handleStdException(e);
    }
    catch(...) {
        std::tie(status_code, result) = handleUnspecifiedException();
    }
    
    std::multimap< std::string, std::string > responseHeaders {};
    static const std::vector<std::string> contentTypes{
        "application/json"
    };
    static const std::string acceptTypes{
        "application/json, application/xml, application/x-www-form-urlencoded, "
    };
    
    if (status_code == 0) {
        responseHeaders.insert(std::make_pair("Content-Type", "text/plain"));
        result = "successful operation";
    
        returnResponse(session, 0, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    defaultSessionClose(session, status_code, result);
}

int BullionNameResource::handler_DELETE(
        std::string & name)
{
    return handler_DELETE_func(name);
}

std::pair<int, Bullion> BullionNameResource::handler_GET(
    std::string & name)
{
    return handler_GET_func(name);
}
int BullionNameResource::handler_PUT(
    std::string & name, Bullion & bullion)
{
    return handler_PUT_func(name, bullion);
}

std::string BullionNameResource::extractBodyContent(const std::shared_ptr<restbed::Session>& session) {
  const auto request = session->get_request();
  int content_length = request->get_header("Content-Length", 0);
  std::string bodyContent;
  session->fetch(content_length,
                 [&bodyContent](const std::shared_ptr<restbed::Session> session,
                                const restbed::Bytes &body) {
                   bodyContent = restbed::String::format(
                       "%.*s\n", (int)body.size(), body.data());
                 });
  return bodyContent;
}

std::string BullionNameResource::extractFormParamsFromBody(const std::string& paramName, const std::string& body) {
    const auto uri = restbed::Uri("urlencoded?" + body, true);
    const auto params = uri.get_query_parameters();
    const auto result = params.find(paramName);
    if (result != params.cend()) {
        return result->second;
    }
    return "";
}

} /* namespace BullionApiResources */

BullionApi::BullionApi(std::shared_ptr<restbed::Service> const& restbedService)
: m_service(restbedService)
{
}

BullionApi::~BullionApi() {}

std::shared_ptr<BullionApiResources::BullionResource> BullionApi::getBullionResource() {
    if (!m_spBullionResource) {
        setResource(std::make_shared<BullionApiResources::BullionResource>());
    }
    return m_spBullionResource;
}
std::shared_ptr<BullionApiResources::BullionCreateWithListResource> BullionApi::getBullionCreateWithListResource() {
    if (!m_spBullionCreateWithListResource) {
        setResource(std::make_shared<BullionApiResources::BullionCreateWithListResource>());
    }
    return m_spBullionCreateWithListResource;
}
std::shared_ptr<BullionApiResources::BullionNameResource> BullionApi::getBullionNameResource() {
    if (!m_spBullionNameResource) {
        setResource(std::make_shared<BullionApiResources::BullionNameResource>());
    }
    return m_spBullionNameResource;
}
void BullionApi::setResource(std::shared_ptr<BullionApiResources::BullionResource> resource) {
    m_spBullionResource = resource;
    m_service->publish(m_spBullionResource);
}
void BullionApi::setResource(std::shared_ptr<BullionApiResources::BullionCreateWithListResource> resource) {
    m_spBullionCreateWithListResource = resource;
    m_service->publish(m_spBullionCreateWithListResource);
}
void BullionApi::setResource(std::shared_ptr<BullionApiResources::BullionNameResource> resource) {
    m_spBullionNameResource = resource;
    m_service->publish(m_spBullionNameResource);
}
void BullionApi::setBullionApiBullionResource(std::shared_ptr<BullionApiResources::BullionResource> spBullionResource) {
    m_spBullionResource = spBullionResource;
    m_service->publish(m_spBullionResource);
}
void BullionApi::setBullionApiBullionCreateWithListResource(std::shared_ptr<BullionApiResources::BullionCreateWithListResource> spBullionCreateWithListResource) {
    m_spBullionCreateWithListResource = spBullionCreateWithListResource;
    m_service->publish(m_spBullionCreateWithListResource);
}
void BullionApi::setBullionApiBullionNameResource(std::shared_ptr<BullionApiResources::BullionNameResource> spBullionNameResource) {
    m_spBullionNameResource = spBullionNameResource;
    m_service->publish(m_spBullionNameResource);
}


void BullionApi::publishDefaultResources() {
    if (!m_spBullionResource) {
        setResource(std::make_shared<BullionApiResources::BullionResource>());
    }
    if (!m_spBullionCreateWithListResource) {
        setResource(std::make_shared<BullionApiResources::BullionCreateWithListResource>());
    }
    if (!m_spBullionNameResource) {
        setResource(std::make_shared<BullionApiResources::BullionNameResource>());
    }
}

std::shared_ptr<restbed::Service> BullionApi::service() {
    return m_service;
}


}
}
}
}

