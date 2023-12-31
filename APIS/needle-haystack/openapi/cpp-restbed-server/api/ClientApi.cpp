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

#include "ClientApi.h"

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

ClientApiException::ClientApiException(int status_code, std::string what)
  : m_status(status_code),
    m_what(what)
{

}
int ClientApiException::getStatus() const
{
    return m_status;
}
const char* ClientApiException::what() const noexcept
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

namespace ClientApiResources {
ClientResource::ClientResource(const std::string& context /* = "/openapi-jaxrs-server-1.0.0/api/v3" */)
{
	this->set_path(context + "/client");
	this->set_method_handler("POST",
		std::bind(&ClientResource::handler_POST_internal, this,
			std::placeholders::_1));
}

std::pair<int, std::string> ClientResource::handleClientApiException(const ClientApiException& e)
{
    return std::make_pair<int, std::string>(e.getStatus(), e.what());
}

std::pair<int, std::string> ClientResource::handleStdException(const std::exception& e)
{
    return std::make_pair<int, std::string>(500, e.what());
}

std::pair<int, std::string> ClientResource::handleUnspecifiedException()
{
    return std::make_pair<int, std::string>(500, "Unknown exception occurred");
}

void ClientResource::setResponseHeader(const std::shared_ptr<restbed::Session>& session, const std::string& header)
{
    session->set_header(header, "");
}

void ClientResource::returnResponse(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result, std::multimap<std::string, std::string>& responseHeaders)
{
    responseHeaders.insert(std::make_pair("Connection", "close"));
    session->close(status, result, responseHeaders);
}

void ClientResource::defaultSessionClose(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result)
{
    session->close(status, result, { {"Connection", "close"} });
}

void ClientResource::handler_POST_internal(const std::shared_ptr<restbed::Session> session)
{
    const auto request = session->get_request();
    // body params or form params here from the body content string
    std::string bodyContent = extractBodyContent(session);
    auto client = extractJsonModelBodyParam<Client>(bodyContent);
    
    int status_code = 500;
    Client resultObject = Client{};
    std::string result = "";
    
    try {
        std::tie(status_code, resultObject) =
            handler_POST(client);
    }
    catch(const ClientApiException& e) {
        std::tie(status_code, result) = handleClientApiException(e);
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


std::pair<int, Client> ClientResource::handler_POST(
        Client & client)
{
    return handler_POST_func(client);
}


std::string ClientResource::extractBodyContent(const std::shared_ptr<restbed::Session>& session) {
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

std::string ClientResource::extractFormParamsFromBody(const std::string& paramName, const std::string& body) {
    const auto uri = restbed::Uri("urlencoded?" + body, true);
    const auto params = uri.get_query_parameters();
    const auto result = params.find(paramName);
    if (result != params.cend()) {
        return result->second;
    }
    return "";
}
ClientCreateWithListResource::ClientCreateWithListResource(const std::string& context /* = "/openapi-jaxrs-server-1.0.0/api/v3" */)
{
	this->set_path(context + "/client/createWithList");
	this->set_method_handler("POST",
		std::bind(&ClientCreateWithListResource::handler_POST_internal, this,
			std::placeholders::_1));
}

std::pair<int, std::string> ClientCreateWithListResource::handleClientApiException(const ClientApiException& e)
{
    return std::make_pair<int, std::string>(e.getStatus(), e.what());
}

std::pair<int, std::string> ClientCreateWithListResource::handleStdException(const std::exception& e)
{
    return std::make_pair<int, std::string>(500, e.what());
}

std::pair<int, std::string> ClientCreateWithListResource::handleUnspecifiedException()
{
    return std::make_pair<int, std::string>(500, "Unknown exception occurred");
}

void ClientCreateWithListResource::setResponseHeader(const std::shared_ptr<restbed::Session>& session, const std::string& header)
{
    session->set_header(header, "");
}

void ClientCreateWithListResource::returnResponse(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result, std::multimap<std::string, std::string>& responseHeaders)
{
    responseHeaders.insert(std::make_pair("Connection", "close"));
    session->close(status, result, responseHeaders);
}

void ClientCreateWithListResource::defaultSessionClose(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result)
{
    session->close(status, result, { {"Connection", "close"} });
}

void ClientCreateWithListResource::handler_POST_internal(const std::shared_ptr<restbed::Session> session)
{
    const auto request = session->get_request();
    // body params or form params here from the body content string
    std::string bodyContent = extractBodyContent(session);
    auto client = extractJsonArrayBodyParam<Client>(bodyContent);
    
    int status_code = 500;
    Client resultObject = Client{};
    std::string result = "";
    
    try {
        std::tie(status_code, resultObject) =
            handler_POST(client);
    }
    catch(const ClientApiException& e) {
        std::tie(status_code, result) = handleClientApiException(e);
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


std::pair<int, Client> ClientCreateWithListResource::handler_POST(
        std::vector<Client> & client)
{
    return handler_POST_func(client);
}


std::string ClientCreateWithListResource::extractBodyContent(const std::shared_ptr<restbed::Session>& session) {
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

std::string ClientCreateWithListResource::extractFormParamsFromBody(const std::string& paramName, const std::string& body) {
    const auto uri = restbed::Uri("urlencoded?" + body, true);
    const auto params = uri.get_query_parameters();
    const auto result = params.find(paramName);
    if (result != params.cend()) {
        return result->second;
    }
    return "";
}
ClientNameResource::ClientNameResource(const std::string& context /* = "/openapi-jaxrs-server-1.0.0/api/v3" */)
{
	this->set_path(context + "/client/{name: .*}");
	this->set_method_handler("DELETE",
		std::bind(&ClientNameResource::handler_DELETE_internal, this,
			std::placeholders::_1));
	this->set_method_handler("GET",
		std::bind(&ClientNameResource::handler_GET_internal, this,
			std::placeholders::_1));
	this->set_method_handler("PUT",
		std::bind(&ClientNameResource::handler_PUT_internal, this,
			std::placeholders::_1));
}

std::pair<int, std::string> ClientNameResource::handleClientApiException(const ClientApiException& e)
{
    return std::make_pair<int, std::string>(e.getStatus(), e.what());
}

std::pair<int, std::string> ClientNameResource::handleStdException(const std::exception& e)
{
    return std::make_pair<int, std::string>(500, e.what());
}

std::pair<int, std::string> ClientNameResource::handleUnspecifiedException()
{
    return std::make_pair<int, std::string>(500, "Unknown exception occurred");
}

void ClientNameResource::setResponseHeader(const std::shared_ptr<restbed::Session>& session, const std::string& header)
{
    session->set_header(header, "");
}

void ClientNameResource::returnResponse(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result, std::multimap<std::string, std::string>& responseHeaders)
{
    responseHeaders.insert(std::make_pair("Connection", "close"));
    session->close(status, result, responseHeaders);
}

void ClientNameResource::defaultSessionClose(const std::shared_ptr<restbed::Session>& session, const int status, const std::string& result)
{
    session->close(status, result, { {"Connection", "close"} });
}

void ClientNameResource::handler_DELETE_internal(const std::shared_ptr<restbed::Session> session)
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
    catch(const ClientApiException& e) {
        std::tie(status_code, result) = handleClientApiException(e);
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
        result = "Invalid Client variable supplied";
    
        returnResponse(session, 400, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    if (status_code == 404) {
        responseHeaders.insert(std::make_pair("Content-Type", "text/plain"));
        result = "Client not found";
    
        returnResponse(session, 404, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    defaultSessionClose(session, status_code, result);
}

// x-extension
void ClientNameResource::handler_GET_internal(const std::shared_ptr<restbed::Session> session) {
    const auto request = session->get_request();
    // Getting the path params
    std::string name = request->get_path_parameter("name", "");
    
    int status_code = 500;
    Client resultObject = Client{};
    std::string result = "";
    
    try {
        std::tie(status_code, resultObject) =
            handler_GET(name);
    }
    catch(const ClientApiException& e) {
        std::tie(status_code, result) = handleClientApiException(e);
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
        result = "Invalid Client variable supplied";
    
        returnResponse(session, 400, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    if (status_code == 404) {
        responseHeaders.insert(std::make_pair("Content-Type", "text/plain"));
        result = "Client not found";
    
        returnResponse(session, 404, result.empty() ? "{}" : result, responseHeaders);
        return;
    }
    defaultSessionClose(session, status_code, result);
}
// x-extension
void ClientNameResource::handler_PUT_internal(const std::shared_ptr<restbed::Session> session) {
    const auto request = session->get_request();
    // body params or form params here from the body content string
    std::string bodyContent = extractBodyContent(session);
    auto client = extractJsonModelBodyParam<Client>(bodyContent);
    // Getting the path params
    std::string name = request->get_path_parameter("name", "");
    
    int status_code = 500;
    std::string result = "";
    
    try {
        status_code =
            handler_PUT(name, client);
    }
    catch(const ClientApiException& e) {
        std::tie(status_code, result) = handleClientApiException(e);
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

int ClientNameResource::handler_DELETE(
        std::string & name)
{
    return handler_DELETE_func(name);
}

std::pair<int, Client> ClientNameResource::handler_GET(
    std::string & name)
{
    return handler_GET_func(name);
}
int ClientNameResource::handler_PUT(
    std::string & name, Client & client)
{
    return handler_PUT_func(name, client);
}

std::string ClientNameResource::extractBodyContent(const std::shared_ptr<restbed::Session>& session) {
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

std::string ClientNameResource::extractFormParamsFromBody(const std::string& paramName, const std::string& body) {
    const auto uri = restbed::Uri("urlencoded?" + body, true);
    const auto params = uri.get_query_parameters();
    const auto result = params.find(paramName);
    if (result != params.cend()) {
        return result->second;
    }
    return "";
}

} /* namespace ClientApiResources */

ClientApi::ClientApi(std::shared_ptr<restbed::Service> const& restbedService)
: m_service(restbedService)
{
}

ClientApi::~ClientApi() {}

std::shared_ptr<ClientApiResources::ClientResource> ClientApi::getClientResource() {
    if (!m_spClientResource) {
        setResource(std::make_shared<ClientApiResources::ClientResource>());
    }
    return m_spClientResource;
}
std::shared_ptr<ClientApiResources::ClientCreateWithListResource> ClientApi::getClientCreateWithListResource() {
    if (!m_spClientCreateWithListResource) {
        setResource(std::make_shared<ClientApiResources::ClientCreateWithListResource>());
    }
    return m_spClientCreateWithListResource;
}
std::shared_ptr<ClientApiResources::ClientNameResource> ClientApi::getClientNameResource() {
    if (!m_spClientNameResource) {
        setResource(std::make_shared<ClientApiResources::ClientNameResource>());
    }
    return m_spClientNameResource;
}
void ClientApi::setResource(std::shared_ptr<ClientApiResources::ClientResource> resource) {
    m_spClientResource = resource;
    m_service->publish(m_spClientResource);
}
void ClientApi::setResource(std::shared_ptr<ClientApiResources::ClientCreateWithListResource> resource) {
    m_spClientCreateWithListResource = resource;
    m_service->publish(m_spClientCreateWithListResource);
}
void ClientApi::setResource(std::shared_ptr<ClientApiResources::ClientNameResource> resource) {
    m_spClientNameResource = resource;
    m_service->publish(m_spClientNameResource);
}
void ClientApi::setClientApiClientResource(std::shared_ptr<ClientApiResources::ClientResource> spClientResource) {
    m_spClientResource = spClientResource;
    m_service->publish(m_spClientResource);
}
void ClientApi::setClientApiClientCreateWithListResource(std::shared_ptr<ClientApiResources::ClientCreateWithListResource> spClientCreateWithListResource) {
    m_spClientCreateWithListResource = spClientCreateWithListResource;
    m_service->publish(m_spClientCreateWithListResource);
}
void ClientApi::setClientApiClientNameResource(std::shared_ptr<ClientApiResources::ClientNameResource> spClientNameResource) {
    m_spClientNameResource = spClientNameResource;
    m_service->publish(m_spClientNameResource);
}


void ClientApi::publishDefaultResources() {
    if (!m_spClientResource) {
        setResource(std::make_shared<ClientApiResources::ClientResource>());
    }
    if (!m_spClientCreateWithListResource) {
        setResource(std::make_shared<ClientApiResources::ClientCreateWithListResource>());
    }
    if (!m_spClientNameResource) {
        setResource(std::make_shared<ClientApiResources::ClientNameResource>());
    }
}

std::shared_ptr<restbed::Service> ClientApi::service() {
    return m_service;
}


}
}
}
}

