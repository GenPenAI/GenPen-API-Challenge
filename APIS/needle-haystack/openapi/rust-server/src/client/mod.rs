use async_trait::async_trait;
use futures::{Stream, future, future::BoxFuture, stream, future::TryFutureExt, future::FutureExt, stream::StreamExt};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use hyper::{Body, Request, Response, service::Service, Uri};
use percent_encoding::{utf8_percent_encode, AsciiSet};
use std::borrow::Cow;
use std::convert::TryInto;
use std::io::{ErrorKind, Read};
use std::error::Error;
use std::future::Future;
use std::fmt;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::str;
use std::str::FromStr;
use std::string::ToString;
use std::task::{Context, Poll};
use swagger::{ApiError, AuthData, BodyExt, Connector, DropContextService, Has, XSpanIdString};
use url::form_urlencoded;


use crate::models;
use crate::header;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
#[allow(dead_code)]
const FRAGMENT_ENCODE_SET: &AsciiSet = &percent_encoding::CONTROLS
    .add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// This encode set is used for object IDs
///
/// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
/// the vertical bar (|) is encoded.
#[allow(dead_code)]
const ID_ENCODE_SET: &AsciiSet = &FRAGMENT_ENCODE_SET.add(b'|');

use crate::{Api,
     CreateAlienwareAuroraR15GamingDesktopResponse,
     CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse,
     DeleteAlienwareAuroraR15GamingDesktopResponse,
     GetAlienwareAuroraR15GamingDesktopByNameResponse,
     UpdateAlienwareAuroraR15GamingDesktopResponse,
     CreateCatResponse,
     CreateCatsWithListInputResponse,
     DeleteCatResponse,
     GetCatByNameResponse,
     UpdateCatResponse,
     CreateDell274kUhdMonitorS2721qsResponse,
     CreateDell274kUhdMonitorS2721qssWithListInputResponse,
     DeleteDell274kUhdMonitorS2721qsResponse,
     GetDell274kUhdMonitorS2721qsByNameResponse,
     UpdateDell274kUhdMonitorS2721qsResponse,
     CreateDogResponse,
     CreateDogsWithListInputResponse,
     DeleteDogResponse,
     GetDogByNameResponse,
     UpdateDogResponse,
     CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse,
     CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse,
     DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse,
     GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse,
     UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse,
     CreateUserResponse,
     CreateUsersWithListInputResponse,
     DeleteUserResponse,
     GetUserByNameResponse,
     LoginUserResponse,
     LogoutUserResponse,
     UpdateUserResponse
     };

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(input: impl TryInto<Uri, Error=hyper::http::uri::InvalidUri>, correct_scheme: Option<&'static str>) -> Result<String, ClientInitError> {
    // First convert to Uri, since a base path is a subset of Uri.
    let uri = input.try_into()?;

    let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = uri.host().ok_or(ClientInitError::MissingHost)?;
    let port = uri.port_u16().map(|x| format!(":{}", x)).unwrap_or_default();
    Ok(format!("{}://{}{}{}", scheme, host, port, uri.path().trim_end_matches('/')))
}

/// A client that implements the API by making HTTP calls out to a server.
pub struct Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    /// Inner service
    client_service: S,

    /// Base path of the API
    base_path: String,

    /// Marker
    marker: PhantomData<fn(C)>,
}

impl<S, C> fmt::Debug for Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl<S, C> Clone for Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Self {
            client_service: self.client_service.clone(),
            base_path: self.base_path.clone(),
            marker: PhantomData,
        }
    }
}

impl<Connector, C> Client<DropContextService<hyper::client::Client<Connector, Body>, C>, C> where
    Connector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Create a client with a custom implementation of hyper::client::Connect.
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with `swagger::Connector::builder()`.
    ///
    /// For ordinary tcp connections, prefer the use of `try_new_http`, `try_new_https`
    /// and `try_new_https_mutual`, to avoid introducing a dependency on the underlying transport layer.
    ///
    /// # Arguments
    ///
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector` - Implementation of `hyper::client::Connect` to use for the client
    pub fn try_new_with_connector(
        base_path: &str,
        protocol: Option<&'static str>,
        connector: Connector,
    ) -> Result<Self, ClientInitError>
    {
        let client_service = hyper::client::Client::builder().build(connector);
        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, protocol)?,
            marker: PhantomData,
        })
    }
}

#[derive(Debug, Clone)]
pub enum HyperClient {
    Http(hyper::client::Client<hyper::client::HttpConnector, Body>),
    Https(hyper::client::Client<HttpsConnector, Body>),
}

impl Service<Request<Body>> for HyperClient {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = hyper::client::ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
       match self {
          HyperClient::Http(client) => client.poll_ready(cx),
          HyperClient::Https(client) => client.poll_ready(cx),
       }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
       match self {
          HyperClient::Http(client) => client.call(req),
          HyperClient::Https(client) => client.call(req)
       }
    }
}

impl<C> Client<DropContextService<HyperClient, C>, C> where
    C: Clone + Send + Sync + 'static,
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new(
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        let uri = Uri::from_str(base_path)?;

        let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;
        let scheme = scheme.to_ascii_lowercase();

        let connector = Connector::builder();

        let client_service = match scheme.as_str() {
            "http" => {
                HyperClient::Http(hyper::client::Client::builder().build(connector.build()))
            },
            "https" => {
                let connector = connector.https()
                   .build()
                   .map_err(ClientInitError::SslError)?;
                HyperClient::Https(hyper::client::Client::builder().build(connector))
            },
            _ => {
                return Err(ClientInitError::InvalidScheme);
            }
        };

        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

impl<C> Client<DropContextService<hyper::client::Client<hyper::client::HttpConnector, Body>, C>, C> where
    C: Clone + Send + Sync + 'static
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new_http(
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        let http_connector = Connector::builder().build();

        Self::try_new_with_connector(base_path, Some("http"), http_connector)
    }
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
type HttpsConnector = hyper_openssl::HttpsConnector<hyper::client::HttpConnector>;

impl<C> Client<DropContextService<hyper::client::Client<HttpsConnector, Body>, C>, C> where
    C: Clone + Send + Sync + 'static
{
    /// Create a client with a TLS connection to the server
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    pub fn try_new_https(base_path: &str) -> Result<Self, ClientInitError>
    {
        let https_connector = Connector::builder()
            .https()
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a TLS connection to the server using a pinned certificate
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_pinned<CA>(
        base_path: &str,
        ca_certificate: CA,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a mutually authenticated TLS connection to the server.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    /// * `client_key` - Path to the client private key
    /// * `client_certificate` - Path to the client's public certificate associated with the private key
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_mutual<CA, K, D>(
        base_path: &str,
        ca_certificate: CA,
        client_key: K,
        client_certificate: D,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
        K: AsRef<Path>,
        D: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .client_authentication(client_key, client_certificate)
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }
}

impl<S, C> Client<S, C> where
    S: Service<
           (Request<Body>, C),
           Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static
{
    /// Constructor for creating a `Client` by passing in a pre-made `hyper::service::Service` /
    /// `tower::Service`
    ///
    /// This allows adding custom wrappers around the underlying transport, for example for logging.
    pub fn try_new_with_client_service(
        client_service: S,
        base_path: &str,
    ) -> Result<Self, ClientInitError>
    {
        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

/// Error type failing to create a Client
#[derive(Debug)]
pub enum ClientInitError {
    /// Invalid URL Scheme
    InvalidScheme,

    /// Invalid URI
    InvalidUri(hyper::http::uri::InvalidUri),

    /// Missing Hostname
    MissingHost,

    /// SSL Connection Error
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
    SslError(native_tls::Error),

    /// SSL Connection Error
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    SslError(openssl::error::ErrorStack),
}

impl From<hyper::http::uri::InvalidUri> for ClientInitError {
    fn from(err: hyper::http::uri::InvalidUri) -> ClientInitError {
        ClientInitError::InvalidUri(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &dyn fmt::Debug = self;
        s.fmt(f)
    }
}

impl Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}

#[async_trait]
impl<S, C> Api<C> for Client<S, C> where
    S: Service<
       (Request<Body>, C),
       Response=Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Has<XSpanIdString>  + Clone + Send + Sync + 'static,
{
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), crate::ServiceError>> {
        match self.client_service.clone().poll_ready(cx) {
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(o)) => Poll::Ready(Ok(o)),
            Poll::Pending => Poll::Pending,
        }
    }

    async fn create_alienware_aurora_r15_gaming_desktop(
        &self,
        param_alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        context: &C) -> Result<CreateAlienwareAuroraR15GamingDesktopResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/alienwareAuroraR15GamingDesktop",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        // Body parameter
        let body = param_alienware_aurora_r15_gaming_desktop.map(|ref body| {
            body.as_xml()
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AlienwareAuroraR15GamingDesktop>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAlienwareAuroraR15GamingDesktopResponse::SuccessfulOperation
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_alienware_aurora_r15_gaming_desktops_with_list_input(
        &self,
        param_alienware_aurora_r15_gaming_desktop: Option<&Vec<models::AlienwareAuroraR15GamingDesktop>>,
        context: &C) -> Result<CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/alienwareAuroraR15GamingDesktop/createWithList",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_alienware_aurora_r15_gaming_desktop.map(|ref body| {
            serde_json::to_string(body).expect("impossible to fail to serialize")
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AlienwareAuroraR15GamingDesktop>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse::SuccessfulOperation
                    (body)
                )
            }
            0 => {
                Ok(
                    CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse::SuccessfulOperation_2
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_alienware_aurora_r15_gaming_desktop(
        &self,
        param_name: String,
        context: &C) -> Result<DeleteAlienwareAuroraR15GamingDesktopResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/alienwareAuroraR15GamingDesktop/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            400 => {
                Ok(
                    DeleteAlienwareAuroraR15GamingDesktopResponse::InvalidAlienwareAuroraR
                )
            }
            404 => {
                Ok(
                    DeleteAlienwareAuroraR15GamingDesktopResponse::AlienwareAuroraR
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_alienware_aurora_r15_gaming_desktop_by_name(
        &self,
        param_name: String,
        context: &C) -> Result<GetAlienwareAuroraR15GamingDesktopByNameResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/alienwareAuroraR15GamingDesktop/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::AlienwareAuroraR15GamingDesktop>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetAlienwareAuroraR15GamingDesktopByNameResponse::SuccessfulOperation
                    (body)
                )
            }
            400 => {
                Ok(
                    GetAlienwareAuroraR15GamingDesktopByNameResponse::InvalidAlienwareAuroraR
                )
            }
            404 => {
                Ok(
                    GetAlienwareAuroraR15GamingDesktopByNameResponse::AlienwareAuroraR
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn update_alienware_aurora_r15_gaming_desktop(
        &self,
        param_name: String,
        param_alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        context: &C) -> Result<UpdateAlienwareAuroraR15GamingDesktopResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/alienwareAuroraR15GamingDesktop/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PUT")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_alienware_aurora_r15_gaming_desktop.map(|ref body| {
            body.as_xml()
        });

        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                Ok(
                    UpdateAlienwareAuroraR15GamingDesktopResponse::SuccessfulOperation
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_cat(
        &self,
        param_cat: Option<models::Cat>,
        context: &C) -> Result<CreateCatResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/cat",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        // Body parameter
        let body = param_cat.map(|ref body| {
            body.as_xml()
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Cat>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateCatResponse::SuccessfulOperation
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_cats_with_list_input(
        &self,
        param_cat: Option<&Vec<models::Cat>>,
        context: &C) -> Result<CreateCatsWithListInputResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/cat/createWithList",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_cat.map(|ref body| {
            serde_json::to_string(body).expect("impossible to fail to serialize")
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Cat>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateCatsWithListInputResponse::SuccessfulOperation
                    (body)
                )
            }
            0 => {
                Ok(
                    CreateCatsWithListInputResponse::SuccessfulOperation_2
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_cat(
        &self,
        param_name: String,
        context: &C) -> Result<DeleteCatResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/cat/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            400 => {
                Ok(
                    DeleteCatResponse::InvalidCatVariableSupplied
                )
            }
            404 => {
                Ok(
                    DeleteCatResponse::CatNotFound
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_cat_by_name(
        &self,
        param_name: String,
        context: &C) -> Result<GetCatByNameResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/cat/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Cat>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetCatByNameResponse::SuccessfulOperation
                    (body)
                )
            }
            400 => {
                Ok(
                    GetCatByNameResponse::InvalidCatVariableSupplied
                )
            }
            404 => {
                Ok(
                    GetCatByNameResponse::CatNotFound
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn update_cat(
        &self,
        param_name: String,
        param_cat: Option<models::Cat>,
        context: &C) -> Result<UpdateCatResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/cat/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PUT")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_cat.map(|ref body| {
            body.as_xml()
        });

        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                Ok(
                    UpdateCatResponse::SuccessfulOperation
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_dell274k_uhd_monitor_s2721qs(
        &self,
        param_dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        context: &C) -> Result<CreateDell274kUhdMonitorS2721qsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dell274kUhDMonitor-S2721QS",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        // Body parameter
        let body = param_dell274k_uhd_monitor_s2721qs.map(|ref body| {
            body.as_xml()
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Dell274kUhdMonitorS2721qs>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateDell274kUhdMonitorS2721qsResponse::SuccessfulOperation
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_dell274k_uhd_monitor_s2721qss_with_list_input(
        &self,
        param_dell274k_uhd_monitor_s2721qs: Option<&Vec<models::Dell274kUhdMonitorS2721qs>>,
        context: &C) -> Result<CreateDell274kUhdMonitorS2721qssWithListInputResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dell274kUhDMonitor-S2721QS/createWithList",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_dell274k_uhd_monitor_s2721qs.map(|ref body| {
            serde_json::to_string(body).expect("impossible to fail to serialize")
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Dell274kUhdMonitorS2721qs>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateDell274kUhdMonitorS2721qssWithListInputResponse::SuccessfulOperation
                    (body)
                )
            }
            0 => {
                Ok(
                    CreateDell274kUhdMonitorS2721qssWithListInputResponse::SuccessfulOperation_2
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_dell274k_uhd_monitor_s2721qs(
        &self,
        param_name: String,
        context: &C) -> Result<DeleteDell274kUhdMonitorS2721qsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dell274kUhDMonitor-S2721QS/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            400 => {
                Ok(
                    DeleteDell274kUhdMonitorS2721qsResponse::InvalidDell
                )
            }
            404 => {
                Ok(
                    DeleteDell274kUhdMonitorS2721qsResponse::Dell
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_dell274k_uhd_monitor_s2721qs_by_name(
        &self,
        param_name: String,
        context: &C) -> Result<GetDell274kUhdMonitorS2721qsByNameResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dell274kUhDMonitor-S2721QS/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Dell274kUhdMonitorS2721qs>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetDell274kUhdMonitorS2721qsByNameResponse::SuccessfulOperation
                    (body)
                )
            }
            400 => {
                Ok(
                    GetDell274kUhdMonitorS2721qsByNameResponse::InvalidDell
                )
            }
            404 => {
                Ok(
                    GetDell274kUhdMonitorS2721qsByNameResponse::Dell
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn update_dell274k_uhd_monitor_s2721qs(
        &self,
        param_name: String,
        param_dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        context: &C) -> Result<UpdateDell274kUhdMonitorS2721qsResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dell274kUhDMonitor-S2721QS/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PUT")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_dell274k_uhd_monitor_s2721qs.map(|ref body| {
            body.as_xml()
        });

        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                Ok(
                    UpdateDell274kUhdMonitorS2721qsResponse::SuccessfulOperation
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_dog(
        &self,
        param_dog: Option<models::Dog>,
        context: &C) -> Result<CreateDogResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dog",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        // Body parameter
        let body = param_dog.map(|ref body| {
            body.as_xml()
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Dog>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateDogResponse::SuccessfulOperation
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_dogs_with_list_input(
        &self,
        param_dog: Option<&Vec<models::Dog>>,
        context: &C) -> Result<CreateDogsWithListInputResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dog/createWithList",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_dog.map(|ref body| {
            serde_json::to_string(body).expect("impossible to fail to serialize")
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Dog>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateDogsWithListInputResponse::SuccessfulOperation
                    (body)
                )
            }
            0 => {
                Ok(
                    CreateDogsWithListInputResponse::SuccessfulOperation_2
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_dog(
        &self,
        param_name: String,
        context: &C) -> Result<DeleteDogResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dog/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            400 => {
                Ok(
                    DeleteDogResponse::InvalidDogVariableSupplied
                )
            }
            404 => {
                Ok(
                    DeleteDogResponse::DogNotFound
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_dog_by_name(
        &self,
        param_name: String,
        context: &C) -> Result<GetDogByNameResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dog/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::Dog>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetDogByNameResponse::SuccessfulOperation
                    (body)
                )
            }
            400 => {
                Ok(
                    GetDogByNameResponse::InvalidDogVariableSupplied
                )
            }
            404 => {
                Ok(
                    GetDogByNameResponse::DogNotFound
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn update_dog(
        &self,
        param_name: String,
        param_dog: Option<models::Dog>,
        context: &C) -> Result<UpdateDogResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/dog/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PUT")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_dog.map(|ref body| {
            body.as_xml()
        });

        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                Ok(
                    UpdateDogResponse::SuccessfulOperation
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        context: &C) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        // Body parameter
        let body = param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor.map(|ref body| {
            body.as_xml()
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse::SuccessfulOperation
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processors_with_list_input(
        &self,
        param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<&Vec<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>>,
        context: &C) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/createWithList",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor.map(|ref body| {
            serde_json::to_string(body).expect("impossible to fail to serialize")
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse::SuccessfulOperation
                    (body)
                )
            }
            0 => {
                Ok(
                    CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse::SuccessfulOperation_2
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        param_name: String,
        context: &C) -> Result<DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            400 => {
                Ok(
                    DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse::InvalidLenovoLegionTower
                )
            }
            404 => {
                Ok(
                    DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse::LenovoLegionTower
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_by_name(
        &self,
        param_name: String,
        context: &C) -> Result<GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse::SuccessfulOperation
                    (body)
                )
            }
            400 => {
                Ok(
                    GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse::InvalidLenovoLegionTower
                )
            }
            404 => {
                Ok(
                    GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse::LenovoLegionTower
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn update_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        param_name: String,
        param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        context: &C) -> Result<UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name}",
            self.base_path
            ,name=utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PUT")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor.map(|ref body| {
            body.as_xml()
        });

        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                Ok(
                    UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse::SuccessfulOperation
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_user(
        &self,
        param_user: Option<models::User>,
        context: &C) -> Result<CreateUserResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/user",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        // Body parameter
        let body = param_user.map(|ref body| {
            body.as_xml()
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::User>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateUserResponse::SuccessfulOperation
                    (body)
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_users_with_list_input(
        &self,
        param_user: Option<&Vec<models::User>>,
        context: &C) -> Result<CreateUsersWithListInputResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/user/createWithList",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_user.map(|ref body| {
            serde_json::to_string(body).expect("impossible to fail to serialize")
        });
        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                // ToDo: this will move to swagger-rs and become a standard From conversion trait
                // once https://github.com/RReverser/serde-xml-rs/pull/45 is accepted upstream
                let body = serde_xml_rs::from_str::<models::User>(body)
                    .map_err(|e| ApiError(format!("Response body did not match the schema: {}", e)))?;
                Ok(CreateUsersWithListInputResponse::SuccessfulOperation
                    (body)
                )
            }
            0 => {
                Ok(
                    CreateUsersWithListInputResponse::SuccessfulOperation_2
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_user(
        &self,
        param_username: String,
        context: &C) -> Result<DeleteUserResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/user/{username}",
            self.base_path
            ,username=utf8_percent_encode(&param_username.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("DELETE")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            400 => {
                Ok(
                    DeleteUserResponse::InvalidUsernameSupplied
                )
            }
            404 => {
                Ok(
                    DeleteUserResponse::UserNotFound
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_user_by_name(
        &self,
        param_username: String,
        context: &C) -> Result<GetUserByNameResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/user/{username}",
            self.base_path
            ,username=utf8_percent_encode(&param_username.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                // ToDo: this will move to swagger-rs and become a standard From conversion trait
                // once https://github.com/RReverser/serde-xml-rs/pull/45 is accepted upstream
                let body = serde_xml_rs::from_str::<models::User>(body)
                    .map_err(|e| ApiError(format!("Response body did not match the schema: {}", e)))?;
                Ok(GetUserByNameResponse::SuccessfulOperation
                    (body)
                )
            }
            400 => {
                Ok(
                    GetUserByNameResponse::InvalidUsernameSupplied
                )
            }
            404 => {
                Ok(
                    GetUserByNameResponse::UserNotFound
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn login_user(
        &self,
        param_username: Option<String>,
        param_password: Option<String>,
        context: &C) -> Result<LoginUserResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/user/login",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_username) = param_username {
                query_string.append_pair("username",
                    &param_username);
            }
            if let Some(param_password) = param_password {
                query_string.append_pair("password",
                    &param_password);
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            200 => {
                let response_x_rate_limit = match response.headers().get(HeaderName::from_static("x-rate-limit")) {
                    Some(response_x_rate_limit) => {
                        let response_x_rate_limit = response_x_rate_limit.clone();
                        let response_x_rate_limit = match TryInto::<header::IntoHeaderValue<i32>>::try_into(response_x_rate_limit) {
                            Ok(value) => value,
                            Err(e) => {
                                return Err(ApiError(format!("Invalid response header X-Rate-Limit for response 200 - {}", e)));
                            },
                        };
                        Some(response_x_rate_limit.0)
                        },
                    None => None,
                };

                let response_x_expires_after = match response.headers().get(HeaderName::from_static("x-expires-after")) {
                    Some(response_x_expires_after) => {
                        let response_x_expires_after = response_x_expires_after.clone();
                        let response_x_expires_after = match TryInto::<header::IntoHeaderValue<chrono::DateTime::<chrono::Utc>>>::try_into(response_x_expires_after) {
                            Ok(value) => value,
                            Err(e) => {
                                return Err(ApiError(format!("Invalid response header X-Expires-After for response 200 - {}", e)));
                            },
                        };
                        Some(response_x_expires_after.0)
                        },
                    None => None,
                };

                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                // ToDo: this will move to swagger-rs and become a standard From conversion trait
                // once https://github.com/RReverser/serde-xml-rs/pull/45 is accepted upstream
                let body = serde_xml_rs::from_str::<String>(body)
                    .map_err(|e| ApiError(format!("Response body did not match the schema: {}", e)))?;
                Ok(LoginUserResponse::SuccessfulOperation
                    {
                        body,
                        x_rate_limit: response_x_rate_limit,
                        x_expires_after: response_x_expires_after,
                    }
                )
            }
            400 => {
                Ok(
                    LoginUserResponse::InvalidUsername
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn logout_user(
        &self,
        context: &C) -> Result<LogoutUserResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/user/logout",
            self.base_path
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                Ok(
                    LogoutUserResponse::SuccessfulOperation
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn update_user(
        &self,
        param_username: String,
        param_user: Option<models::User>,
        context: &C) -> Result<UpdateUserResponse, ApiError>
    {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/openapi-jaxrs-server-1.0.0/api/v3/user/{username}",
            self.base_path
            ,username=utf8_percent_encode(&param_username.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("PUT")
            .uri(uri)
            .body(Body::empty()) {
                Ok(req) => req,
                Err(e) => return Err(ApiError(format!("Unable to create request: {}", e)))
        };

        let body = param_user.map(|ref body| {
            body.as_xml()
        });

        if let Some(body) = body {
                *request.body_mut() = Body::from(body);
        }

        let header = "application/json";
        request.headers_mut().insert(CONTENT_TYPE, match HeaderValue::from_str(header) {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create header: {} - {}", header, e)))
        });

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(HeaderName::from_static("x-span-id"), match header {
            Ok(h) => h,
            Err(e) => return Err(ApiError(format!("Unable to create X-Span ID header value: {}", e)))
        });

        let response = client_service.call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e))).await?;

        match response.status().as_u16() {
            0 => {
                Ok(
                    UpdateUserResponse::SuccessfulOperation
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

}
