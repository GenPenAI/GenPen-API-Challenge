//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use openapi_client::{
    Api,
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
    UpdateUserResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Create AlienwareAuroraR15GamingDesktop
    async fn create_alienware_aurora_r15_gaming_desktop(
        &self,
        alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        context: &C) -> Result<CreateAlienwareAuroraR15GamingDesktopResponse, ApiError>
    {
        let context = context.clone();
        info!("create_alienware_aurora_r15_gaming_desktop({:?}) - X-Span-ID: {:?}", alienware_aurora_r15_gaming_desktop, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates list of AlienwareAuroraR15GamingDesktop with given input array
    async fn create_alienware_aurora_r15_gaming_desktops_with_list_input(
        &self,
        alienware_aurora_r15_gaming_desktop: Option<&Vec<models::AlienwareAuroraR15GamingDesktop>>,
        context: &C) -> Result<CreateAlienwareAuroraR15GamingDesktopsWithListInputResponse, ApiError>
    {
        let context = context.clone();
        info!("create_alienware_aurora_r15_gaming_desktops_with_list_input({:?}) - X-Span-ID: {:?}", alienware_aurora_r15_gaming_desktop, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete alienwareAuroraR15GamingDesktop
    async fn delete_alienware_aurora_r15_gaming_desktop(
        &self,
        name: String,
        context: &C) -> Result<DeleteAlienwareAuroraR15GamingDesktopResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_alienware_aurora_r15_gaming_desktop(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get alienwareAuroraR15GamingDesktop by name
    async fn get_alienware_aurora_r15_gaming_desktop_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetAlienwareAuroraR15GamingDesktopByNameResponse, ApiError>
    {
        let context = context.clone();
        info!("get_alienware_aurora_r15_gaming_desktop_by_name(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update alienwareAuroraR15GamingDesktop
    async fn update_alienware_aurora_r15_gaming_desktop(
        &self,
        name: String,
        alienware_aurora_r15_gaming_desktop: Option<models::AlienwareAuroraR15GamingDesktop>,
        context: &C) -> Result<UpdateAlienwareAuroraR15GamingDesktopResponse, ApiError>
    {
        let context = context.clone();
        info!("update_alienware_aurora_r15_gaming_desktop(\"{}\", {:?}) - X-Span-ID: {:?}", name, alienware_aurora_r15_gaming_desktop, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create Cat
    async fn create_cat(
        &self,
        cat: Option<models::Cat>,
        context: &C) -> Result<CreateCatResponse, ApiError>
    {
        let context = context.clone();
        info!("create_cat({:?}) - X-Span-ID: {:?}", cat, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates list of Cat with given input array
    async fn create_cats_with_list_input(
        &self,
        cat: Option<&Vec<models::Cat>>,
        context: &C) -> Result<CreateCatsWithListInputResponse, ApiError>
    {
        let context = context.clone();
        info!("create_cats_with_list_input({:?}) - X-Span-ID: {:?}", cat, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete cat
    async fn delete_cat(
        &self,
        name: String,
        context: &C) -> Result<DeleteCatResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_cat(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get cat by name
    async fn get_cat_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetCatByNameResponse, ApiError>
    {
        let context = context.clone();
        info!("get_cat_by_name(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update cat
    async fn update_cat(
        &self,
        name: String,
        cat: Option<models::Cat>,
        context: &C) -> Result<UpdateCatResponse, ApiError>
    {
        let context = context.clone();
        info!("update_cat(\"{}\", {:?}) - X-Span-ID: {:?}", name, cat, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create Dell274kUhdMonitor-S2721qs
    async fn create_dell274k_uhd_monitor_s2721qs(
        &self,
        dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        context: &C) -> Result<CreateDell274kUhdMonitorS2721qsResponse, ApiError>
    {
        let context = context.clone();
        info!("create_dell274k_uhd_monitor_s2721qs({:?}) - X-Span-ID: {:?}", dell274k_uhd_monitor_s2721qs, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates list of Dell274kUhdMonitor-S2721qs with given input array
    async fn create_dell274k_uhd_monitor_s2721qss_with_list_input(
        &self,
        dell274k_uhd_monitor_s2721qs: Option<&Vec<models::Dell274kUhdMonitorS2721qs>>,
        context: &C) -> Result<CreateDell274kUhdMonitorS2721qssWithListInputResponse, ApiError>
    {
        let context = context.clone();
        info!("create_dell274k_uhd_monitor_s2721qss_with_list_input({:?}) - X-Span-ID: {:?}", dell274k_uhd_monitor_s2721qs, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete dell274kUhDMonitor-S2721QS
    async fn delete_dell274k_uhd_monitor_s2721qs(
        &self,
        name: String,
        context: &C) -> Result<DeleteDell274kUhdMonitorS2721qsResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_dell274k_uhd_monitor_s2721qs(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get dell274kUhDMonitor-S2721QS by name
    async fn get_dell274k_uhd_monitor_s2721qs_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetDell274kUhdMonitorS2721qsByNameResponse, ApiError>
    {
        let context = context.clone();
        info!("get_dell274k_uhd_monitor_s2721qs_by_name(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update dell274kUhDMonitor-S2721QS
    async fn update_dell274k_uhd_monitor_s2721qs(
        &self,
        name: String,
        dell274k_uhd_monitor_s2721qs: Option<models::Dell274kUhdMonitorS2721qs>,
        context: &C) -> Result<UpdateDell274kUhdMonitorS2721qsResponse, ApiError>
    {
        let context = context.clone();
        info!("update_dell274k_uhd_monitor_s2721qs(\"{}\", {:?}) - X-Span-ID: {:?}", name, dell274k_uhd_monitor_s2721qs, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create Dog
    async fn create_dog(
        &self,
        dog: Option<models::Dog>,
        context: &C) -> Result<CreateDogResponse, ApiError>
    {
        let context = context.clone();
        info!("create_dog({:?}) - X-Span-ID: {:?}", dog, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates list of Dog with given input array
    async fn create_dogs_with_list_input(
        &self,
        dog: Option<&Vec<models::Dog>>,
        context: &C) -> Result<CreateDogsWithListInputResponse, ApiError>
    {
        let context = context.clone();
        info!("create_dogs_with_list_input({:?}) - X-Span-ID: {:?}", dog, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete dog
    async fn delete_dog(
        &self,
        name: String,
        context: &C) -> Result<DeleteDogResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_dog(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get dog by name
    async fn get_dog_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetDogByNameResponse, ApiError>
    {
        let context = context.clone();
        info!("get_dog_by_name(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update dog
    async fn update_dog(
        &self,
        name: String,
        dog: Option<models::Dog>,
        context: &C) -> Result<UpdateDogResponse, ApiError>
    {
        let context = context.clone();
        info!("update_dog(\"{}\", {:?}) - X-Span-ID: {:?}", name, dog, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        context: &C) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>
    {
        let context = context.clone();
        info!("create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor({:?}) - X-Span-ID: {:?}", lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates list of LenovoLegionTower7iGen8Desktop-IntelCoreI9Processor with given input array
    async fn create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processors_with_list_input(
        &self,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<&Vec<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>>,
        context: &C) -> Result<CreateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorsWithListInputResponse, ApiError>
    {
        let context = context.clone();
        info!("create_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processors_with_list_input({:?}) - X-Span-ID: {:?}", lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn delete_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        name: String,
        context: &C) -> Result<DeleteLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor by name
    async fn get_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_by_name(
        &self,
        name: String,
        context: &C) -> Result<GetLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorByNameResponse, ApiError>
    {
        let context = context.clone();
        info!("get_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor_by_name(\"{}\") - X-Span-ID: {:?}", name, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update lenovoLegionTower7iGen8Desktop-IntelCoreI9Processor
    async fn update_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(
        &self,
        name: String,
        lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor: Option<models::LenovoLegionTower7iGen8DesktopIntelCoreI9Processor>,
        context: &C) -> Result<UpdateLenovoLegionTower7iGen8DesktopIntelCoreI9ProcessorResponse, ApiError>
    {
        let context = context.clone();
        info!("update_lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor(\"{}\", {:?}) - X-Span-ID: {:?}", name, lenovo_legion_tower7i_gen8_desktop_intel_core_i9_processor, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create user
    async fn create_user(
        &self,
        user: Option<models::User>,
        context: &C) -> Result<CreateUserResponse, ApiError>
    {
        let context = context.clone();
        info!("create_user({:?}) - X-Span-ID: {:?}", user, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates list of users with given input array
    async fn create_users_with_list_input(
        &self,
        user: Option<&Vec<models::User>>,
        context: &C) -> Result<CreateUsersWithListInputResponse, ApiError>
    {
        let context = context.clone();
        info!("create_users_with_list_input({:?}) - X-Span-ID: {:?}", user, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete user
    async fn delete_user(
        &self,
        username: String,
        context: &C) -> Result<DeleteUserResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_user(\"{}\") - X-Span-ID: {:?}", username, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get user by user name
    async fn get_user_by_name(
        &self,
        username: String,
        context: &C) -> Result<GetUserByNameResponse, ApiError>
    {
        let context = context.clone();
        info!("get_user_by_name(\"{}\") - X-Span-ID: {:?}", username, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Logs user into the system
    async fn login_user(
        &self,
        username: Option<String>,
        password: Option<String>,
        context: &C) -> Result<LoginUserResponse, ApiError>
    {
        let context = context.clone();
        info!("login_user({:?}, {:?}) - X-Span-ID: {:?}", username, password, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Logs out current logged in user session
    async fn logout_user(
        &self,
        context: &C) -> Result<LogoutUserResponse, ApiError>
    {
        let context = context.clone();
        info!("logout_user() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update user
    async fn update_user(
        &self,
        username: String,
        user: Option<models::User>,
        context: &C) -> Result<UpdateUserResponse, ApiError>
    {
        let context = context.clone();
        info!("update_user(\"{}\", {:?}) - X-Span-ID: {:?}", username, user, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
