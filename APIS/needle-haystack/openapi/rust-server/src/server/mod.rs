use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

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

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/openapi-jaxrs-server-1.0.0/api/v3/alienwareAuroraR15GamingDesktop$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/alienwareAuroraR15GamingDesktop/createWithList$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/alienwareAuroraR15GamingDesktop/(?P<name>[^/?#]*)$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/cat$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/cat/createWithList$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/cat/(?P<name>[^/?#]*)$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/dell274kUhDMonitor-S2721QS$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/dell274kUhDMonitor-S2721QS/createWithList$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/dell274kUhDMonitor-S2721QS/(?P<name>[^/?#]*)$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/dog$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/dog/createWithList$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/dog/(?P<name>[^/?#]*)$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/createWithList$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/(?P<name>[^/?#]*)$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/user$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/user/createWithList$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/user/login$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/user/logout$",
            r"^/openapi-jaxrs-server-1.0.0/api/v3/user/(?P<username>[^/?#]*)$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_ALIENWAREAURORAR15GAMINGDESKTOP: usize = 0;
    pub(crate) static ID_ALIENWAREAURORAR15GAMINGDESKTOP_CREATEWITHLIST: usize = 1;
    pub(crate) static ID_ALIENWAREAURORAR15GAMINGDESKTOP_NAME: usize = 2;
    lazy_static! {
        pub static ref REGEX_ALIENWAREAURORAR15GAMINGDESKTOP_NAME: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/openapi-jaxrs-server-1.0.0/api/v3/alienwareAuroraR15GamingDesktop/(?P<name>[^/?#]*)$")
                .expect("Unable to create regex for ALIENWAREAURORAR15GAMINGDESKTOP_NAME");
    }
    pub(crate) static ID_CAT: usize = 3;
    pub(crate) static ID_CAT_CREATEWITHLIST: usize = 4;
    pub(crate) static ID_CAT_NAME: usize = 5;
    lazy_static! {
        pub static ref REGEX_CAT_NAME: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/openapi-jaxrs-server-1.0.0/api/v3/cat/(?P<name>[^/?#]*)$")
                .expect("Unable to create regex for CAT_NAME");
    }
    pub(crate) static ID_DELL274KUHDMONITOR_S2721QS: usize = 6;
    pub(crate) static ID_DELL274KUHDMONITOR_S2721QS_CREATEWITHLIST: usize = 7;
    pub(crate) static ID_DELL274KUHDMONITOR_S2721QS_NAME: usize = 8;
    lazy_static! {
        pub static ref REGEX_DELL274KUHDMONITOR_S2721QS_NAME: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/openapi-jaxrs-server-1.0.0/api/v3/dell274kUhDMonitor-S2721QS/(?P<name>[^/?#]*)$")
                .expect("Unable to create regex for DELL274KUHDMONITOR_S2721QS_NAME");
    }
    pub(crate) static ID_DOG: usize = 9;
    pub(crate) static ID_DOG_CREATEWITHLIST: usize = 10;
    pub(crate) static ID_DOG_NAME: usize = 11;
    lazy_static! {
        pub static ref REGEX_DOG_NAME: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/openapi-jaxrs-server-1.0.0/api/v3/dog/(?P<name>[^/?#]*)$")
                .expect("Unable to create regex for DOG_NAME");
    }
    pub(crate) static ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR: usize = 12;
    pub(crate) static ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_CREATEWITHLIST: usize = 13;
    pub(crate) static ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME: usize = 14;
    lazy_static! {
        pub static ref REGEX_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/openapi-jaxrs-server-1.0.0/api/v3/lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/(?P<name>[^/?#]*)$")
                .expect("Unable to create regex for LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME");
    }
    pub(crate) static ID_USER: usize = 15;
    pub(crate) static ID_USER_CREATEWITHLIST: usize = 16;
    pub(crate) static ID_USER_LOGIN: usize = 17;
    pub(crate) static ID_USER_LOGOUT: usize = 18;
    pub(crate) static ID_USER_USERNAME: usize = 19;
    lazy_static! {
        pub static ref REGEX_USER_USERNAME: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/openapi-jaxrs-server-1.0.0/api/v3/user/(?P<username>[^/?#]*)$")
                .expect("Unable to create regex for USER_USERNAME");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString>  + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match method {

            // CreateAlienwareAuroraR15GamingDesktop - POST /alienwareAuroraR15GamingDesktop
            hyper::Method::POST if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_alienware_aurora_r15_gaming_desktop) => param_alienware_aurora_r15_gaming_desktop,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_alienware_aurora_r15_gaming_desktop(
                                            param_alienware_aurora_r15_gaming_desktop,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateAlienwareAuroraR15GamingDesktopResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_ALIENWARE_AURORA_R15_GAMING_DESKTOP_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter AlienwareAuroraR15GamingDesktop: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter AlienwareAuroraR15GamingDesktop")),
                        }
            },

            // CreateAlienwareAuroraR15GamingDesktopsWithListInput - POST /alienwareAuroraR15GamingDesktop/createWithList
            hyper::Method::POST if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_CREATEWITHLIST) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_alienware_aurora_r15_gaming_desktop: Option<Vec<models::AlienwareAuroraR15GamingDesktop>> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_alienware_aurora_r15_gaming_desktop) => param_alienware_aurora_r15_gaming_desktop,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_alienware_aurora_r15_gaming_desktops_with_list_input(
                                            param_alienware_aurora_r15_gaming_desktop.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_ALIENWARE_AURORA_R15_GAMING_DESKTOPS_WITH_LIST_INPUT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse::SuccessfulOperation_2
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter AlienwareAuroraR15GamingDesktop: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter AlienwareAuroraR15GamingDesktop")),
                        }
            },

            // DeleteAlienwareAuroraR15GamingDesktop - DELETE /alienwareAuroraR15GamingDesktop/{name}
            hyper::Method::DELETE if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ALIENWAREAURORAR15GAMINGDESKTOP_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ALIENWAREAURORAR15GAMINGDESKTOP_NAME in set but failed match against \"{}\"", path, paths::REGEX_ALIENWAREAURORAR15GAMINGDESKTOP_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_alienware_aurora_r15_gaming_desktop(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteAlienwareAuroraR15GamingDesktopResponse::InvalidAlienwareAuroraR
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                DeleteAlienwareAuroraR15GamingDesktopResponse::AlienwareAuroraR
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAlienwareAuroraR15GamingDesktopByName - GET /alienwareAuroraR15GamingDesktop/{name}
            hyper::Method::GET if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ALIENWAREAURORAR15GAMINGDESKTOP_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ALIENWAREAURORAR15GAMINGDESKTOP_NAME in set but failed match against \"{}\"", path, paths::REGEX_ALIENWAREAURORAR15GAMINGDESKTOP_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_alienware_aurora_r15_gaming_desktop_by_name(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAlienwareAuroraR15GamingDesktopByNameResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALIENWARE_AURORA_R15_GAMING_DESKTOP_BY_NAME_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetAlienwareAuroraR15GamingDesktopByNameResponse::InvalidAlienwareAuroraR
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                GetAlienwareAuroraR15GamingDesktopByNameResponse::AlienwareAuroraR
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UpdateAlienwareAuroraR15GamingDesktop - PUT /alienwareAuroraR15GamingDesktop/{name}
            hyper::Method::PUT if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ALIENWAREAURORAR15GAMINGDESKTOP_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ALIENWAREAURORAR15GAMINGDESKTOP_NAME in set but failed match against \"{}\"", path, paths::REGEX_ALIENWAREAURORAR15GAMINGDESKTOP_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_alienware_aurora_r15_gaming_desktop) => param_alienware_aurora_r15_gaming_desktop,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.update_alienware_aurora_r15_gaming_desktop(
                                            param_name,
                                            param_alienware_aurora_r15_gaming_desktop,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdateAlienwareAuroraR15GamingDesktopResponse::SuccessfulOperation
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter AlienwareAuroraR15GamingDesktop: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter AlienwareAuroraR15GamingDesktop")),
                        }
            },

            // CreateCat - POST /cat
            hyper::Method::POST if path.matched(paths::ID_CAT) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_cat: Option<models::Cat> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_cat) => param_cat,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_cat(
                                            param_cat,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateCatResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_CAT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Cat: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Cat")),
                        }
            },

            // CreateCatsWithListInput - POST /cat/createWithList
            hyper::Method::POST if path.matched(paths::ID_CAT_CREATEWITHLIST) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_cat: Option<Vec<models::Cat>> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_cat) => param_cat,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_cats_with_list_input(
                                            param_cat.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateCatsWithListInputResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_CATS_WITH_LIST_INPUT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                CreateCatsWithListInputResponse::SuccessfulOperation_2
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Cat: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Cat")),
                        }
            },

            // DeleteCat - DELETE /cat/{name}
            hyper::Method::DELETE if path.matched(paths::ID_CAT_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_CAT_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE CAT_NAME in set but failed match against \"{}\"", path, paths::REGEX_CAT_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_cat(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteCatResponse::InvalidCatVariableSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                DeleteCatResponse::CatNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetCatByName - GET /cat/{name}
            hyper::Method::GET if path.matched(paths::ID_CAT_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_CAT_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE CAT_NAME in set but failed match against \"{}\"", path, paths::REGEX_CAT_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_cat_by_name(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetCatByNameResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CAT_BY_NAME_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetCatByNameResponse::InvalidCatVariableSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                GetCatByNameResponse::CatNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UpdateCat - PUT /cat/{name}
            hyper::Method::PUT if path.matched(paths::ID_CAT_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_CAT_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE CAT_NAME in set but failed match against \"{}\"", path, paths::REGEX_CAT_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_cat: Option<models::Cat> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_cat) => param_cat,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.update_cat(
                                            param_name,
                                            param_cat,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdateCatResponse::SuccessfulOperation
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Cat: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Cat")),
                        }
            },

            // CreateDell274kUhdMonitorS2721qs - POST /dell274kUhDMonitor-S2721QS
            hyper::Method::POST if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_dell274k_uhd_monitor_s2721qs) => param_dell274k_uhd_monitor_s2721qs,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_dell274k_uhd_monitor_s2721qs(
                                            param_dell274k_uhd_monitor_s2721qs,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateDell274kUhdMonitorS2721qsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_DELL274K_UHD_MONITOR_S2721QS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Dell274kUhdMonitorS2721qs: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Dell274kUhdMonitorS2721qs")),
                        }
            },

            // CreateDell274kUhdMonitorS2721qssWithListInput - POST /dell274kUhDMonitor-S2721QS/createWithList
            hyper::Method::POST if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_CREATEWITHLIST) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_dell274k_uhd_monitor_s2721qs: Option<Vec<models::Dell274kUhdMonitorS2721qs>> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_dell274k_uhd_monitor_s2721qs) => param_dell274k_uhd_monitor_s2721qs,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_dell274k_uhd_monitor_s2721qss_with_list_input(
                                            param_dell274k_uhd_monitor_s2721qs.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateDell274kUhdMonitorS2721qssWithListInputResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_DELL274K_UHD_MONITOR_S2721QSS_WITH_LIST_INPUT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                CreateDell274kUhdMonitorS2721qssWithListInputResponse::SuccessfulOperation_2
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Dell274kUhdMonitor-S2721qs: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Dell274kUhdMonitor-S2721qs")),
                        }
            },

            // DeleteDell274kUhdMonitorS2721qs - DELETE /dell274kUhDMonitor-S2721QS/{name}
            hyper::Method::DELETE if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_DELL274KUHDMONITOR_S2721QS_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE DELL274KUHDMONITOR_S2721QS_NAME in set but failed match against \"{}\"", path, paths::REGEX_DELL274KUHDMONITOR_S2721QS_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_dell274k_uhd_monitor_s2721qs(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteDell274kUhdMonitorS2721qsResponse::InvalidDell
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                DeleteDell274kUhdMonitorS2721qsResponse::Dell
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetDell274kUhdMonitorS2721qsByName - GET /dell274kUhDMonitor-S2721QS/{name}
            hyper::Method::GET if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_DELL274KUHDMONITOR_S2721QS_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE DELL274KUHDMONITOR_S2721QS_NAME in set but failed match against \"{}\"", path, paths::REGEX_DELL274KUHDMONITOR_S2721QS_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_dell274k_uhd_monitor_s2721qs_by_name(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetDell274kUhdMonitorS2721qsByNameResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_DELL274K_UHD_MONITOR_S2721QS_BY_NAME_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetDell274kUhdMonitorS2721qsByNameResponse::InvalidDell
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                GetDell274kUhdMonitorS2721qsByNameResponse::Dell
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UpdateDell274kUhdMonitorS2721qs - PUT /dell274kUhDMonitor-S2721QS/{name}
            hyper::Method::PUT if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_DELL274KUHDMONITOR_S2721QS_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE DELL274KUHDMONITOR_S2721QS_NAME in set but failed match against \"{}\"", path, paths::REGEX_DELL274KUHDMONITOR_S2721QS_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_dell274k_uhd_monitor_s2721qs) => param_dell274k_uhd_monitor_s2721qs,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.update_dell274k_uhd_monitor_s2721qs(
                                            param_name,
                                            param_dell274k_uhd_monitor_s2721qs,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdateDell274kUhdMonitorS2721qsResponse::SuccessfulOperation
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Dell274kUhdMonitorS2721qs: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Dell274kUhdMonitorS2721qs")),
                        }
            },

            // CreateDog - POST /dog
            hyper::Method::POST if path.matched(paths::ID_DOG) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_dog: Option<models::Dog> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_dog) => param_dog,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_dog(
                                            param_dog,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateDogResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_DOG_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Dog: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Dog")),
                        }
            },

            // CreateDogsWithListInput - POST /dog/createWithList
            hyper::Method::POST if path.matched(paths::ID_DOG_CREATEWITHLIST) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_dog: Option<Vec<models::Dog>> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_dog) => param_dog,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_dogs_with_list_input(
                                            param_dog.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateDogsWithListInputResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_DOGS_WITH_LIST_INPUT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                CreateDogsWithListInputResponse::SuccessfulOperation_2
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Dog: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Dog")),
                        }
            },

            // DeleteDog - DELETE /dog/{name}
            hyper::Method::DELETE if path.matched(paths::ID_DOG_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_DOG_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE DOG_NAME in set but failed match against \"{}\"", path, paths::REGEX_DOG_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_dog(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteDogResponse::InvalidDogVariableSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                DeleteDogResponse::DogNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetDogByName - GET /dog/{name}
            hyper::Method::GET if path.matched(paths::ID_DOG_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_DOG_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE DOG_NAME in set but failed match against \"{}\"", path, paths::REGEX_DOG_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_dog_by_name(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetDogByNameResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_DOG_BY_NAME_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetDogByNameResponse::InvalidDogVariableSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                GetDogByNameResponse::DogNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UpdateDog - PUT /dog/{name}
            hyper::Method::PUT if path.matched(paths::ID_DOG_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_DOG_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE DOG_NAME in set but failed match against \"{}\"", path, paths::REGEX_DOG_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_dog: Option<models::Dog> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_dog) => param_dog,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.update_dog(
                                            param_name,
                                            param_dog,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdateDogResponse::SuccessfulOperation
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Dog: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Dog")),
                        }
            },

            // CreateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor - POST /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
            hyper::Method::POST if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor) => param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
                                            param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_LENOVO_LEGION_TOWER7I_GEN8_DESKTOP_INTEL_CORE_I9_PROCESSOR_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter LenovoLegionTower7iGen8DesktopIntelCoreI9Processor: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter LenovoLegionTower7iGen8DesktopIntelCoreI9Processor")),
                        }
            },

            // CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInput - POST /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/createWithList
            hyper::Method::POST if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_CREATEWITHLIST) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<Vec<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor) => param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processors_with_list_input(
                                            param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_LENOVO_LEGION_TOWER7I_GEN8_DESKTOP_INTEL_CORE_I9_PROCESSORS_WITH_LIST_INPUT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse::SuccessfulOperation_2
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor")),
                        }
            },

            // DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9Processor - DELETE /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name}
            hyper::Method::DELETE if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME in set but failed match against \"{}\"", path, paths::REGEX_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse::InvalidLenovoLegionTower
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse::LenovoLegionTower
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByName - GET /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name}
            hyper::Method::GET if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME in set but failed match against \"{}\"", path, paths::REGEX_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_by_name(
                                            param_name,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_LENOVO_LEGION_TOWER7I_GEN8_DESKTOP_INTEL_CORE_I9_PROCESSOR_BY_NAME_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse::InvalidLenovoLegionTower
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse::LenovoLegionTower
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor - PUT /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name}
            hyper::Method::PUT if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME in set but failed match against \"{}\"", path, paths::REGEX_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME.as_str())
                    );

                let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor) => param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.update_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
                                            param_name,
                                            param_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse::SuccessfulOperation
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter LenovoLegionTower7iGen8DesktopIntelCoreI9Processor: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter LenovoLegionTower7iGen8DesktopIntelCoreI9Processor")),
                        }
            },

            // CreateUser - POST /user
            hyper::Method::POST if path.matched(paths::ID_USER) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_user: Option<models::User> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_user) => param_user,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_user(
                                            param_user,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateUserResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_USER_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter User: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter User")),
                        }
            },

            // CreateUsersWithListInput - POST /user/createWithList
            hyper::Method::POST if path.matched(paths::ID_USER_CREATEWITHLIST) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_user: Option<Vec<models::User>> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_user) => param_user,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_users_with_list_input(
                                            param_user.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateUsersWithListInputResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml")
                                                            .expect("Unable to create Content-Type header for CREATE_USERS_WITH_LIST_INPUT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                CreateUsersWithListInputResponse::SuccessfulOperation_2
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter User: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter User")),
                        }
            },

            // DeleteUser - DELETE /user/{username}
            hyper::Method::DELETE if path.matched(paths::ID_USER_USERNAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_USER_USERNAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE USER_USERNAME in set but failed match against \"{}\"", path, paths::REGEX_USER_USERNAME.as_str())
                    );

                let param_username = match percent_encoding::percent_decode(path_params["username"].as_bytes()).decode_utf8() {
                    Ok(param_username) => match param_username.parse::<String>() {
                        Ok(param_username) => param_username,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter username: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["username"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_user(
                                            param_username,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteUserResponse::InvalidUsernameSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                DeleteUserResponse::UserNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetUserByName - GET /user/{username}
            hyper::Method::GET if path.matched(paths::ID_USER_USERNAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_USER_USERNAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE USER_USERNAME in set but failed match against \"{}\"", path, paths::REGEX_USER_USERNAME.as_str())
                    );

                let param_username = match percent_encoding::percent_decode(path_params["username"].as_bytes()).decode_utf8() {
                    Ok(param_username) => match param_username.parse::<String>() {
                        Ok(param_username) => param_username,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter username: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["username"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_user_by_name(
                                            param_username,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetUserByNameResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml")
                                                            .expect("Unable to create Content-Type header for GET_USER_BY_NAME_SUCCESSFUL_OPERATION"));
                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetUserByNameResponse::InvalidUsernameSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                GetUserByNameResponse::UserNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // LoginUser - GET /user/login
            hyper::Method::GET if path.matched(paths::ID_USER_LOGIN) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_username = query_params.iter().filter(|e| e.0 == "username").map(|e| e.1.to_owned())
                    .next();
                let param_username = match param_username {
                    Some(param_username) => {
                        let param_username =
                            <String as std::str::FromStr>::from_str
                                (&param_username);
                        match param_username {
                            Ok(param_username) => Some(param_username),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter username - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter username")),
                        }
                    },
                    None => None,
                };
                let param_password = query_params.iter().filter(|e| e.0 == "password").map(|e| e.1.to_owned())
                    .next();
                let param_password = match param_password {
                    Some(param_password) => {
                        let param_password =
                            <String as std::str::FromStr>::from_str
                                (&param_password);
                        match param_password {
                            Ok(param_password) => Some(param_password),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter password - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter password")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.login_user(
                                            param_username,
                                            param_password,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                LoginUserResponse::SuccessfulOperation
                                                    {
                                                        body,
                                                        x_rate_limit,
                                                        x_expires_after
                                                    }
                                                => {
                                                    if let Some(x_rate_limit) = x_rate_limit {
                                                    let x_rate_limit = match header::IntoHeaderValue(x_rate_limit).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling x_rate_limit header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("x-rate-limit"),
                                                        x_rate_limit
                                                    );
                                                    }
                                                    if let Some(x_expires_after) = x_expires_after {
                                                    let x_expires_after = match header::IntoHeaderValue(x_expires_after).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling x_expires_after header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("x-expires-after"),
                                                        x_expires_after
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml")
                                                            .expect("Unable to create Content-Type header for LOGIN_USER_SUCCESSFUL_OPERATION"));
                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                LoginUserResponse::InvalidUsername
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // LogoutUser - GET /user/logout
            hyper::Method::GET if path.matched(paths::ID_USER_LOGOUT) => {
                                let result = api_impl.logout_user(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                LogoutUserResponse::SuccessfulOperation
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UpdateUser - PUT /user/{username}
            hyper::Method::PUT if path.matched(paths::ID_USER_USERNAME) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_USER_USERNAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE USER_USERNAME in set but failed match against \"{}\"", path, paths::REGEX_USER_USERNAME.as_str())
                    );

                let param_username = match percent_encoding::percent_decode(path_params["username"].as_bytes()).decode_utf8() {
                    Ok(param_username) => match param_username.parse::<String>() {
                        Ok(param_username) => param_username,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter username: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["username"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_user: Option<models::User> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_user) => param_user,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.update_user(
                                            param_username,
                                            param_user,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdateUserResponse::SuccessfulOperation
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter User: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter User")),
                        }
            },

            _ if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP) => method_not_allowed(),
            _ if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_CREATEWITHLIST) => method_not_allowed(),
            _ if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_NAME) => method_not_allowed(),
            _ if path.matched(paths::ID_CAT) => method_not_allowed(),
            _ if path.matched(paths::ID_CAT_CREATEWITHLIST) => method_not_allowed(),
            _ if path.matched(paths::ID_CAT_NAME) => method_not_allowed(),
            _ if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS) => method_not_allowed(),
            _ if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_CREATEWITHLIST) => method_not_allowed(),
            _ if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_NAME) => method_not_allowed(),
            _ if path.matched(paths::ID_DOG) => method_not_allowed(),
            _ if path.matched(paths::ID_DOG_CREATEWITHLIST) => method_not_allowed(),
            _ if path.matched(paths::ID_DOG_NAME) => method_not_allowed(),
            _ if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR) => method_not_allowed(),
            _ if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_CREATEWITHLIST) => method_not_allowed(),
            _ if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME) => method_not_allowed(),
            _ if path.matched(paths::ID_USER) => method_not_allowed(),
            _ if path.matched(paths::ID_USER_CREATEWITHLIST) => method_not_allowed(),
            _ if path.matched(paths::ID_USER_LOGIN) => method_not_allowed(),
            _ if path.matched(paths::ID_USER_LOGOUT) => method_not_allowed(),
            _ if path.matched(paths::ID_USER_USERNAME) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // CreateAlienwareAuroraR15GamingDesktop - POST /alienwareAuroraR15GamingDesktop
            hyper::Method::POST if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP) => Some("CreateAlienwareAuroraR15GamingDesktop"),
            // CreateAlienwareAuroraR15GamingDesktopsWithListInput - POST /alienwareAuroraR15GamingDesktop/createWithList
            hyper::Method::POST if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_CREATEWITHLIST) => Some("CreateAlienwareAuroraR15GamingDesktopsWithListInput"),
            // DeleteAlienwareAuroraR15GamingDesktop - DELETE /alienwareAuroraR15GamingDesktop/{name}
            hyper::Method::DELETE if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_NAME) => Some("DeleteAlienwareAuroraR15GamingDesktop"),
            // GetAlienwareAuroraR15GamingDesktopByName - GET /alienwareAuroraR15GamingDesktop/{name}
            hyper::Method::GET if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_NAME) => Some("GetAlienwareAuroraR15GamingDesktopByName"),
            // UpdateAlienwareAuroraR15GamingDesktop - PUT /alienwareAuroraR15GamingDesktop/{name}
            hyper::Method::PUT if path.matched(paths::ID_ALIENWAREAURORAR15GAMINGDESKTOP_NAME) => Some("UpdateAlienwareAuroraR15GamingDesktop"),
            // CreateCat - POST /cat
            hyper::Method::POST if path.matched(paths::ID_CAT) => Some("CreateCat"),
            // CreateCatsWithListInput - POST /cat/createWithList
            hyper::Method::POST if path.matched(paths::ID_CAT_CREATEWITHLIST) => Some("CreateCatsWithListInput"),
            // DeleteCat - DELETE /cat/{name}
            hyper::Method::DELETE if path.matched(paths::ID_CAT_NAME) => Some("DeleteCat"),
            // GetCatByName - GET /cat/{name}
            hyper::Method::GET if path.matched(paths::ID_CAT_NAME) => Some("GetCatByName"),
            // UpdateCat - PUT /cat/{name}
            hyper::Method::PUT if path.matched(paths::ID_CAT_NAME) => Some("UpdateCat"),
            // CreateDell274kUhdMonitorS2721qs - POST /dell274kUhDMonitor-S2721QS
            hyper::Method::POST if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS) => Some("CreateDell274kUhdMonitorS2721qs"),
            // CreateDell274kUhdMonitorS2721qssWithListInput - POST /dell274kUhDMonitor-S2721QS/createWithList
            hyper::Method::POST if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_CREATEWITHLIST) => Some("CreateDell274kUhdMonitorS2721qssWithListInput"),
            // DeleteDell274kUhdMonitorS2721qs - DELETE /dell274kUhDMonitor-S2721QS/{name}
            hyper::Method::DELETE if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_NAME) => Some("DeleteDell274kUhdMonitorS2721qs"),
            // GetDell274kUhdMonitorS2721qsByName - GET /dell274kUhDMonitor-S2721QS/{name}
            hyper::Method::GET if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_NAME) => Some("GetDell274kUhdMonitorS2721qsByName"),
            // UpdateDell274kUhdMonitorS2721qs - PUT /dell274kUhDMonitor-S2721QS/{name}
            hyper::Method::PUT if path.matched(paths::ID_DELL274KUHDMONITOR_S2721QS_NAME) => Some("UpdateDell274kUhdMonitorS2721qs"),
            // CreateDog - POST /dog
            hyper::Method::POST if path.matched(paths::ID_DOG) => Some("CreateDog"),
            // CreateDogsWithListInput - POST /dog/createWithList
            hyper::Method::POST if path.matched(paths::ID_DOG_CREATEWITHLIST) => Some("CreateDogsWithListInput"),
            // DeleteDog - DELETE /dog/{name}
            hyper::Method::DELETE if path.matched(paths::ID_DOG_NAME) => Some("DeleteDog"),
            // GetDogByName - GET /dog/{name}
            hyper::Method::GET if path.matched(paths::ID_DOG_NAME) => Some("GetDogByName"),
            // UpdateDog - PUT /dog/{name}
            hyper::Method::PUT if path.matched(paths::ID_DOG_NAME) => Some("UpdateDog"),
            // CreateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor - POST /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
            hyper::Method::POST if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR) => Some("CreateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor"),
            // CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInput - POST /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/createWithList
            hyper::Method::POST if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_CREATEWITHLIST) => Some("CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInput"),
            // DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9Processor - DELETE /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name}
            hyper::Method::DELETE if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME) => Some("DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9Processor"),
            // GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByName - GET /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name}
            hyper::Method::GET if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME) => Some("GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByName"),
            // UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor - PUT /lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor/{name}
            hyper::Method::PUT if path.matched(paths::ID_LENOVOLEGIONTOWER7IGEN8DESKTOP_INTELCOREI9PROCESSOR_NAME) => Some("UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9Processor"),
            // CreateUser - POST /user
            hyper::Method::POST if path.matched(paths::ID_USER) => Some("CreateUser"),
            // CreateUsersWithListInput - POST /user/createWithList
            hyper::Method::POST if path.matched(paths::ID_USER_CREATEWITHLIST) => Some("CreateUsersWithListInput"),
            // DeleteUser - DELETE /user/{username}
            hyper::Method::DELETE if path.matched(paths::ID_USER_USERNAME) => Some("DeleteUser"),
            // GetUserByName - GET /user/{username}
            hyper::Method::GET if path.matched(paths::ID_USER_USERNAME) => Some("GetUserByName"),
            // LoginUser - GET /user/login
            hyper::Method::GET if path.matched(paths::ID_USER_LOGIN) => Some("LoginUser"),
            // LogoutUser - GET /user/logout
            hyper::Method::GET if path.matched(paths::ID_USER_LOGOUT) => Some("LogoutUser"),
            // UpdateUser - PUT /user/{username}
            hyper::Method::PUT if path.matched(paths::ID_USER_USERNAME) => Some("UpdateUser"),
            _ => None,
        }
    }
}
